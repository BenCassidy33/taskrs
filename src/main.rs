#![allow(unused_imports)]

use actix_web::{
    dev::HttpServiceFactory, get, post, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use types::project::Project;

pub mod actions;
pub mod types;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(hello))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello World!")
// }

// #[get("/get/{project_id}")]
// async fn get_project() -> impl Responder {
//     let file = std::fs::File::open("project.json");

//     todo!();
// }

fn main() {
    let project = crate::actions::actions::read_project("src/test.json");
    println!("{:?}", project);
}
