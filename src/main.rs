use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

pub mod actions;
pub mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_project_by_id)
            .service(get_task_by_id)
            .service(get_tasks_by_project_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/api/v1/project/project/{project_id}")]
async fn get_project_by_id(project_id: web::Path<String>) -> impl Responder {
    let project = actions::actions::get_project_by_id(String::from(project_id.into_inner()));

    return match project {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::Ok().json(json!({"error": "Project Not Found", "code": 404})),
    };
}

#[get("/api/v1/project/task/{task_id}")]
async fn get_task_by_id(task_id: web::Path<String>) -> impl Responder {
    let task = actions::actions::get_task_by_id(String::from(task_id.into_inner()));

    return match task {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::Ok().json(json!({"error": "Project Not Found", "code": 404})),
    };
}

#[get("/api/v1/project/tasks/{project_id}")]
async fn get_tasks_by_project_id(project_id: web::Path<String>) -> impl Responder {
    let tasks = actions::actions::get_tasks(String::from(project_id.into_inner()));

    return match tasks {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::Ok().json(json!({"error": "Project Not Found", "code": 404})),
    };
}
