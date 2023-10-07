#![allow(dead_code)]

use std::fmt::Display;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationErrors};

use crate::serde_string;
use crate::error::ScheduleError;

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

serde_string!(LessonType, Time, LessonTime, Weeks, Auditorium, Day);

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

impl FromStr for LessonType {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("лекція") || s.contains("Лекція") {
            Ok(LessonType::Lection)
        } else {
            let number: String = s.chars().filter(|c| c.is_digit(10)).collect();

            Ok(
                LessonType::Classes(number.parse::<u8>()
                    .map_err(|_| ScheduleError::InvalidLessonType(s.to_owned()))?)
            )
        }
    }
}

#[derive(Validate, Default, Clone, Copy, Debug)]
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

impl FromStr for Time {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((h, m)) = s.split_once(':') {
            let hours = h.parse::<u8>().map_err(|_| ScheduleError::InvalidTimeFormat(s.to_owned()))?;
            let minutes = m.parse::<u8>().map_err(|_| ScheduleError::InvalidTimeFormat(s.to_owned()))?;

            Ok(Time::new(hours, minutes)?)
        } else if let Some((h, m)) = s.split_once('.') {
            let hours = h.parse::<u8>().map_err(|_| ScheduleError::InvalidTimeFormat(s.to_owned()))?;
            let minutes = m.parse::<u8>().map_err(|_| ScheduleError::InvalidTimeFormat(s.to_owned()))?;

            Ok(Time::new(hours, minutes)?)
        } else {
            Err(ScheduleError::InvalidTimeFormat(s.to_owned()))
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct LessonTime {
    pub from: Time,
    pub to: Time,
}

impl Display for LessonTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

impl FromStr for LessonTime {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once('-') {
            Ok(LessonTime {
                from: Time::from_str(from)?, 
                to: Time::from_str(to)?
            })
        } else {
            Err(ScheduleError::InvalidLessonTime(s.to_owned()))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Weeks {
    Single(u8),
    Range {
        first: u8,
        last: u8,
    },
    Array(Vec<u8>),
    Combined(Vec<Weeks>),
}

impl Display for Weeks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weeks::Single(day) => write!(f, "{day}"),
            Weeks::Range { first, last } => write!(f, "{first}-{last}"),
            Weeks::Array(days) => {
                let mut display = String::new();

                for day in days {
                    display.push_str(format!("{day},").as_str());
                }

                write!(f, "{display}")
            }
            Weeks::Combined(weeks) => {
                let mut display = String::new();

                for week in weeks {
                    display.push_str(format!("{week},").as_str())
                }

                write!(f, "")
            }
        }
    }
}

impl FromStr for Weeks {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(&[' ', '\n'][..], "");

        if s.contains(',') && s.contains('-') {
            // Combined
            // TODO: Parse combined weeks
            todo!("Cannot parse combined weeks");
        } else {
            // Range
            if let Some((f, l)) = s.split_once('-') {
                let first = f.parse::<u8>().map_err(|_| ScheduleError::InvalidWeeksFormat(s.to_owned()))?;
                let last = l.parse::<u8>().map_err(|_| ScheduleError::InvalidWeeksFormat(s.to_owned()))?;
    
                Ok(Weeks::Range { first, last })
            // Single
            } else if let Ok(day) = s.parse::<u8>() {
                Ok(Weeks::Single(day))
            // Array
            } else {
                let results: Vec<Result<u8, ScheduleError>> = s.split(',').map(|n| {
                    n.parse::<u8>().map_err(|_| ScheduleError::InvalidWeeksFormat(s.to_owned()))
                }).collect();

                let mut days = vec![];

                for result in results {
                    days.push(result?);
                }

                Ok(Weeks::Array(days))
            }
        }
    }
}

impl Default for Weeks {
    fn default() -> Self {
        Weeks::Single(1)
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

#[derive(Default, Clone, Copy, Debug)]
pub enum Day {
    #[default]
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
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
        };

        write!(f, "{stringed}")
    }
}

impl FromStr for Day {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Day::*;

        let s = s.replace(&[' ', '\n'][..], "");

        match s.as_str() {
            "Понеділок" => Ok(Monday),
            "Вівторок" => Ok(Tuesday),
            "Середа" => Ok(Wednesday),
            "Четвер" => Ok(Thursday),
            "П’ятниця" | "П`ятниця" | "П'ятниця" => Ok(Friday),
            "Субота" => Ok(Saturday),
            _ => Err(ScheduleError::InvalidDayOfWeek(s.to_owned())),
        }
    }
}