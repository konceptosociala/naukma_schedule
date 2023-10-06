#![allow(dead_code)]

use std::fmt::Display;
use serde::Serialize;
use validator::{Validate, ValidationErrors};

use crate::serde_string;

#[derive(Serialize, Default, Clone, Debug)]
pub struct Group {
    #[serde(rename = "Назва")]
    pub name: LessonType,
    #[serde(rename = "Час")]
    pub time: LessonTime,
    #[serde(rename = "Тижні")]
    pub weeks: Weeks,
    #[serde(rename = "Аудиторія")]
    pub auditorium: Auditorium,
    #[serde(rename = "День тижня")]
    pub day: Day,
}

serde_string!(LessonType, LessonTime, Weeks, Auditorium, Day);

pub type GroupNumber = u8;

#[derive(Default, Clone, Debug)]
pub enum LessonType {
    #[default]
    Lection,
    Classes(GroupNumber),
}

impl Display for LessonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LessonType::Lection => write!(f, "Лекція"),
            LessonType::Classes(number) => write!(f, "{number}")
        }
    }
}

#[derive(Validate, Default, Clone, Debug)]
pub struct Time {
    #[validate(range(min = 0, max = 23))]
    hours: u8,
    #[validate(range(min = 0, max = 59))]
    minutes: u8,
}

impl Time {
    pub fn new(hours: u8, minutes: u8) -> Result<Self, ValidationErrors> {
        let time = Time { hours, minutes };
        time.validate()?;

        Ok(time)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = {
            if self.hours < 10 {
                format!("0{}", self.hours)
            } else {
                self.hours.to_string()
            }
        };

        let minutes = {
            if self.minutes < 10 {
                format!("0{}", self.minutes)
            } else {
                self.minutes.to_string()
            }
        };

        write!(f, "{hours}:{minutes}")
    }
}

#[derive(Default, Clone, Debug)]
pub struct LessonTime {
    pub from: Time,
    pub to: Time,
}

impl Display for LessonTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

#[derive(Validate, Default, Clone, Debug)]
pub struct Weeks {
    #[validate(range(min = 1, max = 40))]
    first: u8,
    #[validate(range(min = 1, max = 40))]
    last: u8,
}

impl Display for Weeks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.first, self.last)
    }
}

impl Weeks {
    pub fn new(first: u8, last: u8) -> Result<Self, ValidationErrors> {
        let weeks = Weeks { first, last };
        weeks.validate()?;

        Ok(weeks)
    }
}

#[derive(Default, Clone, Debug)]
pub enum Auditorium {
    #[default]
    Distance,
    ArtCenter,
    Pavilion(AuditoriumNumber),
}

impl Display for Auditorium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Auditorium::Distance => write!(f, "Дистанційно"),
            Auditorium::ArtCenter => write!(f, "КМЦ"),
            Auditorium::Pavilion(number) => write!(f, "{number}"),
        }
    }
}

#[derive(Validate, Default, Clone, Debug)]
pub struct AuditoriumNumber {
    #[validate(range(min = 1, max = 9))]
    pavilion: u8,
    #[validate(range(min = 1, max = 499))]
    room: u16,
}

impl AuditoriumNumber {
    pub fn new(pavilion: u8, room: u16) -> Result<Self, ValidationErrors> {
        let number = AuditoriumNumber { pavilion, room };
        number.validate()?;

        Ok(number)
    }
}

impl Display for AuditoriumNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.pavilion, self.room)
    }
}

#[derive(Default, Clone, Debug)]
pub enum Day {
    #[default]
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Day::*;

        let stringed = match self {
            Monday => "Понеділок",
            Tuesday => "Вівторок",
            Wednesday => "Середа",
            Thursday => "Четвер",
            Friday => "П'ятниця",
            Saturday => "Субота",
            Sunday => "Неділя",
        };

        write!(f, "{stringed}")
    }
}

