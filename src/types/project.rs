#![allow(dead_code)]

use crate::types::tasks::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub title: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>,
    pub users: Vec<User>,
    #[serde(rename = "colorTags")]
    pub color_tags: Vec<ColorTag>,
}
