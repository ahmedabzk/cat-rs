use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatError{
    #[error("reading or writing of the content of the file is interrupted")]
    Interrupted(#[from] std::io::Error),
    #[error("The specified file does not exist and neither create or create_new is set")]
    NotFound(String),
    #[error("The user lacks permission to get the specified access rights for the file")]
    PermissionDenied(String),
    #[error("create_new was specified and the file already exists.")]
    AlreadyExists(String),
    #[error("Invalid combinations of open options (truncate without write access, no access mode set, etc.)")]
    InvalidInput(String),
}



