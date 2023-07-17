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

pub fn write() {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Test {
    id: String,
    name: String,
    description: String,
    tasks: Vec<Task>,
}

pub fn read_project<P: AsRef<Path>>(path: P) -> Result<Project, ErrorType> {
    let contents: Result<String, std::io::Error> = std::fs::read_to_string(path);
    dbg!(contents);
    let data: Project =
        serde_json::from_str(std::fs::read_to_string("src/test.json").unwrap().as_str()).unwrap();

    dbg!(data);

    let _data = Project {
        id: String::from(""),
        name: String::from(""),
        description: String::from(""),
        tasks: vec![],
        users: vec![],
        color_tags: vec![],
    };

    Ok(_data)
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
