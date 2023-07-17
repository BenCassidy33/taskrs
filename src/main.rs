use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

pub mod actions;
pub mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_project))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/api/v1/project/{project_id}")]
async fn get_project(project_id: web::Path<String>) -> impl Responder {
    let project = actions::actions::get_project_by_id(String::from(project_id.into_inner()));

    return match project {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::Ok().json(json!({"error": "Project Not Found", "code": 404})),
    };
}
