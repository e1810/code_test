use std::process::{Command, Output};
use std::env;
use anyhow::{bail, Context, Result};


pub struct ExecResult {
	pub status: i32,
	pub stdout: Vec<String>,
	pub stderr: Vec<String>
}


fn output2result(output: Output) -> ExecResult {
	ExecResult {
		status: match output.status.code() {
			Some(code) => code,
			None => -1,
		},
		stdout:
			String::from_utf8(output.stdout).unwrap()
				.split("\n").map(|x| x.to_string()).collect(),
		stderr:
			String::from_utf8(output.stderr).unwrap()
				.split("\n").map(|x| x.to_string()).collect(),
	}
}


pub fn exec(lang: String, code: String) -> Result<ExecResult> {
	let mut path = env::current_dir().unwrap();
	path.push("dockerdir");
	path.push(
		match lang.as_ref() {
			"Bash" => "Main.sh",
			"C" => "Main.c",
			"C++" => "Main.cpp",
			_ => "",
		}
	);
	std::fs::write(path.into_os_string(), code).unwrap();

	// Docker image の build
	let mut docker = Command::new("docker");
	docker.arg("build").arg("-t").arg("code_executer").arg("dockerdir");
	let out = docker.output().context("Cannot build docker: invalid command")?;
	if !out.status.success() {
		bail!("Cannot build docker:\n{}", String::from_utf8(out.stderr).unwrap());
	}


	// コンテナを実行
	docker = Command::new("docker");
	docker.arg("run").arg("code_executer");
	match lang.as_ref() {
		"Bash" => {docker.arg("bash").arg("./Main.sh");},
		"C" => {docker.arg("./Main_c.out");},
		"C++" => {docker.arg("./Main_cpp.out");},
		_ => ()
	}

	let out = docker.output().context("Cannot run docker: invalid command")?;
	Ok(output2result(out))
}
