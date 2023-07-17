#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SysErrorType {
    FileNotFound,
    FileNotReadable,
    FileNotWritable,
    FileNotValid,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum UsrErrorType {
    InvalidInput,
    InvalidProject,
    InvalidTask,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ErrorType {
    System(SysErrorType),
    User(UsrErrorType),
    Unknown,
}
