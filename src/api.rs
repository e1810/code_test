use std::process::Command;
use actix_web::{web, get, post, HttpResponse, ResponseError};
use askama::Template;
use serde::Deserialize;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum MyError {
	#[error("Failed to render HTML")]
	AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}



#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate {
	lang: String,
	code: String,
	result: String,
}


#[get("/")]
pub async fn index_get() -> Result<HttpResponse, MyError> {
	let html = IndexTemplate{
		lang: "Bash".to_string(),
		code: "echo \"Hello, world\"".to_string(),
		result: "0".to_string(),
	};
	let res_body = html.render()?;
	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(res_body)
	)
}


#[derive(Deserialize)]
pub struct Submission {
	lang: String,
	code: String,
}


#[post("/")]
pub async fn index_post(params: web::Form<Submission>) -> Result<HttpResponse, MyError> {
	let html = IndexTemplate{
		lang: params.lang.clone(), 
		code: params.code.clone(),
		result: exec(params.lang.clone(), params.code.clone()),
	};
	let res_body = html.render()?;
	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(res_body)
	)
}


fn exec(lang: String, code: String) -> String {
	std::fs::write("dockerdir/Main.sh", code).unwrap();

	// Docker image の build
	let mut docker = Command::new("docker");
	docker.arg("build").arg("-t").arg("code_executer").arg("dockerdir/");
	println!("{:?}", docker.output());

	// コンテナを実行
	docker = Command::new("docker");
	docker.arg("run").arg("code_executer");
	match lang.as_ref() {
		"Bash" => {
			docker.arg("sh").arg("./Main.sh");
			()
		},
		_ => ()
	}
	let result = docker.output().unwrap();

	if result.status.success() {
		String::from_utf8(result.stdout).unwrap()
	} else {
		String::from_utf8(result.stderr).unwrap()
	}
}
	
