use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

// type Percentage = RangeInclusive<f32>;
// static VALID_PROGRESS_RANGE: Percentage = 0.0..=100.0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColorTag {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DueType {
    Date(String),
    NotDue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    Bug,
    Feature,
    Improvement,
    Documentation,
    Refactor,
    Test,
    Other,
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: Priority,
    // pub due: DueType,
    pub progress: RangeInclusive<u8>,
    pub comments: Option<Vec<String>>,
    #[serde(rename = "colorTag")]
    pub color_tag: Option<ColorTag>,
    #[serde(rename = "taskType")]
    pub task_type: TaskType,
}
