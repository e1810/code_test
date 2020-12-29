use std::process::Command;
use std::env;


pub fn exec(lang: String, code: String) -> Vec<String> {
	let mut path = env::current_dir().unwrap();
	path.push("dockerdir");
	path.push(
		match lang.as_ref() {
			"Bash" => "Main.sh",
			_ => "",
		}
	);
	std::fs::write(path.into_os_string(), code).unwrap();

	// Docker image の build
	let mut docker = Command::new("docker");
	docker.arg("build").arg("-t").arg("code_executer").arg("dockerdir");
	{
		let out = docker.output().unwrap();
		if !out.status.success() {
			eprintln!("{}", String::from_utf8(out.stderr).unwrap());
		}
	}

	// コンテナを実行
	docker = Command::new("docker");
	docker.arg("run").arg("code_executer");
	match lang.as_ref() {
		"Bash" => {docker.arg("bash").arg("./Main.sh");},
		_ => ()
	}
	let result = docker.output().unwrap();

	let result_string = {
		if result.status.success() {
			"Result: OK\n".to_string() + &String::from_utf8(result.stdout).unwrap()
		} else {
			"Result: Error\n".to_string() + &String::from_utf8(result.stderr).unwrap()
		}
	}.to_string();

	result_string.split("\n").map(|x| x.to_string()).collect()
}
