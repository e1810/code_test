use actix_web::{web, get, post, HttpResponse, ResponseError};
use askama::Template;
use serde::Deserialize;
use thiserror::Error;
use once_cell::sync::Lazy;

mod exec;


static INTERNAL_ERROR_MESSAGE: Lazy<Vec<String>> = Lazy::new(|| vec![
	"Internal Error".to_string(),
	"Sorry, Some problems occur".to_string(),
	"Please contact the author".to_string()
]);


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
	result: exec::ExecResult,
}


#[get("/")]
pub async fn index_get() -> Result<HttpResponse, MyError> {
	println!("'/' was accessed on GET");
	let html = IndexTemplate{
		lang: "Bash".to_string(),
		code: "echo \"Hello, world\"".to_string(),
		result:
			match exec::exec("Bash".to_string(), "echo \"Hello, world\"".to_string()) {
				Ok(res) => res,
				Err(_e) => exec::ExecResult {
						status: -1, stdout: vec!(),
						stderr: INTERNAL_ERROR_MESSAGE.clone(),
					},
			}
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
	println!("'/' was accessed on POST");
	let html = IndexTemplate{
		lang: params.lang.clone(), 
		code: params.code.clone(),
		result:
			match exec::exec(params.lang.clone(), params.code.clone()) {
				Ok(res) => res,
				Err(_e) => exec::ExecResult {
						status: -1, stdout: vec!(),
						stderr: INTERNAL_ERROR_MESSAGE.clone(),
					},
			}
	};
	let res_body = html.render()?;
	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(res_body)
	)
}	
