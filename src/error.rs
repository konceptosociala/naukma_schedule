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
    

    #[error("Invalid auditorium: `{0}`.\nExamples: `3-205`, `1-313`, `КМЦ`, `Д`, `д`, `Дистанційно`")]
    InvalidAuditorium(String),
    #[error("Invalid study weeks format: `{0}`.\nExamples: `1-13`, `2,3,7,9`, `1`, `1,3-8,10,12-16`")]
    InvalidWeeksFormat(String),
    #[error("Invalid time format: `{0}`.\nExamples: `13:25`, `06.45`")]
    InvalidTimeFormat(String),
    #[error("Invalid lesson time: `{0}`.\nExamples: `08:30-09:50`, `11.40-13.00`")]
    InvalidLessonTime(String),
    #[error("Invalid lesson type passed: {0}")]
    InvalidLessonType(String),
    #[error("Wrong day of the week passed: {0}")]
    InvalidDayOfWeek(String),
    #[error("No such speciality: {0}")]
    InvalidSpeciality(String),
}

pub type ScheduleResult<T> = Result<T, ScheduleError>;