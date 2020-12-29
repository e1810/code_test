use actix_web::{App, HttpServer};

mod api;


#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
	HttpServer::new(move || {
		App::new()
			.service(api::index_get)
			.service(api::index_post)
	})
	.bind("0.0.0.0:8888")?
	.run()
	.await?;
	Ok(())
}
