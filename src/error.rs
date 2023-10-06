use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScheduleError {
    
}

pub type ScheduleResult<T> = Result<T, ScheduleError>;