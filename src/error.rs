use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ScheduleError {
    #[error("Input/Output error")]
    IoError(#[from] std::io::Error),
    #[error("Cannot process .xlsx document")]
    XlsxError(#[from] calamine::Error),
    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),
    
    #[error("Invalid study weeks format: `{0}`.\nExamples: `1-13`, `2,3,7,9`, `1`, `1,2-9,10,12-16`")]
    InvalidWeeksFormat(String),
    #[error("Invalid time format: `{0}`.\nExamples: `13:25`, `06.45`")]
    InvalidTimeFormat(String),
    #[error("Invalid lesson time: `{0}`.\nExamples: `08:30-09:50`, `11.40-13.00`")]
    InvalidLessonTime(String),
    #[error("Invalid lesson type passed: {0}")]
    InvalidLessonType(String),
    #[error("Wrong day of the week passed: {0}")]
    InvalidDayOfWeek(String),
    
    #[error("dummy error")]
    Dummy
}

pub type ScheduleResult<T> = Result<T, ScheduleError>;