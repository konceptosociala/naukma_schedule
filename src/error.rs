use thiserror::Error;
use validator::ValidationErrors;

/// Represents custom error types for the university schedule parser program.
#[derive(Debug, Error)]
pub enum ScheduleError {
    /// Input/Output error while reading or writing files.
    #[error("Input/Output error")]
    IoError(#[from] std::io::Error),
    /// Error related to processing .xlsx documents.
    #[error("Cannot process .xlsx document")]
    XlsxError(#[from] calamine::Error),
    /// Validation error indicating that data does not meet expected criteria.
    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),
    
    /// Error indicating an invalid auditorium format.
    #[error("Invalid auditorium: `{0}`.\nExamples: `3-205`, `1-313`, `КМЦ`, `Д`, `д`, `Дистанційно`")]
    InvalidAuditorium(String),
    /// Error indicating an invalid study weeks format.
    #[error("Invalid study weeks format: `{0}`.\nExamples: `1-13`, `2,3,7,9`, `1`, `1,3-8,10,12-16`")]
    InvalidWeeksFormat(String),
    /// Error indicating an invalid time format.
    #[error("Invalid time format: `{0}`.\nExamples: `13:25`, `06.45`")]
    InvalidTimeFormat(String),
    /// Error indicating an invalid lesson time format.
    #[error("Invalid lesson time: `{0}`.\nExamples: `08:30-09:50`, `11.40-13.00`")]
    InvalidLessonTime(String),
    /// Error indicating an invalid lesson type.
    #[error("Invalid lesson type passed: {0}")]
    InvalidLessonType(String),
    /// Error indicating an invalid day of the week.
    #[error("Wrong day of the week passed: {0}")]
    InvalidDayOfWeek(String),
    /// Error indicating that a speciality does not exist.
    #[error("No such speciality: {0}")]
    InvalidSpeciality(String),
}

/// A type alias for results that may return a `ScheduleError`.
pub type ScheduleResult<T> = Result<T, ScheduleError>;