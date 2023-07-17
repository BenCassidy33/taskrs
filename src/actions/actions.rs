/*
* ToDo: Add write() read_task() seraialize_project() seraialize_task() get_project_by_id()
* get_task_by_id()
*
* fix task date and maybe task type enums
*/

use actix_web::web::Json;
use serde_json::json;
use std::path::Path;

use crate::types::{project::Project, tasks::Task};

#[derive(Debug)]
pub enum SysErrorType {
    FileNotFound,
    FileNotReadable,
    FileNotWritable,
    FileNotValid,
}

#[derive(Debug)]
pub enum UsrErrorType {
    InvalidInput,
    InvalidProject,
    InvalidTask,
}

#[derive(Debug)]
pub enum ErrorType {
    System(SysErrorType),
    User(UsrErrorType),
    Unknown,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Test {
    id: String,
    name: String,
    description: String,
    tasks: Vec<Task>,
}

pub fn get_project_by_id(id: String) -> Result<Project, ErrorType> {
    let contents: Result<String, std::io::Error> = std::fs::read_to_string("src/data.json");
    let data: Vec<Project> = serde_json::from_str(contents.unwrap().as_str()).unwrap();

    for project in data.iter() {
        if project.id == id {
            return Ok(project.clone());
        }
    }

    Err(ErrorType::User(UsrErrorType::InvalidProject))
}

pub fn read_task() -> Task {
    todo!()
}

pub fn seraialize_project() -> Json<Project> {
    todo!()
}
pub fn seraialize_task() -> Json<Task> {
    todo!()
}
