use actix_web::{web, get, post, HttpResponse, ResponseError};
use askama::Template;
use serde::Deserialize;
use thiserror::Error;

mod exec;


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
		result: exec::exec(params.lang.clone(), params.code.clone()),
	};
	let res_body = html.render()?;
	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(res_body)
	)
}	
