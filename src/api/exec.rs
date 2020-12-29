use std::process::Command;
use std::env;
use std::path::Path;


pub fn exec(lang: String, code: String) -> String {
	let mut path = env::current_dir().unwrap();
	path.push("dockerdir");
	path.push(
		match lang.as_ref() {
			"Bash" => "Main.sh",
			_ => "",
		}
	);
	println!("{:?}", path);
	std::fs::write(path.into_os_string(), code).unwrap();

	// Docker image の build
	let mut docker = Command::new("docker");
	docker.arg("build").arg("-t").arg("code_executer").arg("dockerdir");
	println!("{:?}", docker.output());

	// コンテナを実行
	docker = Command::new("docker");
	docker.arg("run").arg("code_executer");
	match lang.as_ref() {
		"Bash" => {docker.arg("sh").arg("./Main.sh");},
		_ => ()
	}
	let result = docker.output().unwrap();

	if result.status.success() {
		String::from_utf8(result.stdout).unwrap()
	} else {
		String::from_utf8(result.stderr).unwrap()
	}
}
