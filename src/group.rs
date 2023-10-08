use std::fmt::Display;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationErrors};

use crate::macros::impl_serde_display_fromstr;
use crate::error::ScheduleError;

/// Represents a university group, including its name, 
/// lesson time, studying weeks, auditorium and day of the week.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Group {
    /// The name of the group / actually type of the lesson being taught.
    #[serde(rename = "Назва")]
    pub name: LessonType,
    /// The time of the lesson.
    #[serde(rename = "Час")]
    pub time: LessonTime,
    /// The weeks during which the lesson occurs.
    #[serde(rename = "Тижні")]
    pub weeks: Weeks,
    /// The auditorium where the lesson takes place.
    #[serde(rename = "Аудиторія")]
    pub auditorium: Auditorium,
    /// The day of the week when the lesson occurs.
    #[serde(rename = "День тижня")]
    pub day: Day,
}

impl_serde_display_fromstr!(LessonType, Time, LessonTime, Weeks, Auditorium, Day);

/// Represents the number of a discipline group.
pub type GroupNumber = u8;

/// Represents the name of the group / type of a university lesson, 
/// which can be a lection or classes with a group number.
#[derive(Default, Clone, Debug)]
pub enum LessonType {
    /// A lection lesson type (intended for each group).
    #[default]
    Lection,
    /// A classes lesson type with a group number specified.
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
            let number: String = s.chars().filter(|c| c.is_ascii_digit()).collect();

            Ok(
                LessonType::Classes(number.parse::<u8>()
                    .map_err(|_| ScheduleError::InvalidLessonType(s.to_owned()))?)
            )
        }
    }
}

/// Represents a time of day, including hours and minutes.
///
/// The `Time` struct is used to represent a specific time of day and can be validated
/// to ensure that the hours and minutes are within the valid range.
#[derive(Validate, Default, Clone, Copy, Debug)]
pub struct Time {
    /// The hours component of the time
    #[validate(range(min = 0, max = 23))]
    hours: u8,
    /// The minutes component of the time
    #[validate(range(min = 0, max = 59))]
    minutes: u8,
}

impl Time {
    /// Creates a new `Time` instance with the specified hours and minutes.
    ///
    /// # Arguments
    ///
    /// * `hours`: The hours component of the time.
    /// * `minutes`: The minutes component of the time.
    ///
    /// # Returns
    ///
    /// A `Result` containing the created `Time` if it passes validation,
    /// or validation errors if validation fails.
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

/// Represents a time range for a lesson, including start and end time.
#[derive(Default, Clone, Copy, Debug)]
pub struct LessonTime {
    /// The starting time of the lesson.
    pub from: Time,
    /// The ending time of the lesson.
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

/// Represents the weeks during which a lesson occurs in the university schedule.
#[derive(Clone, Debug, PartialEq)]
pub enum Weeks {
    /// A single week number. Example `6`.
    Single(u8),
    /// A range of week numbers, specified by the first and last week of studying. Example `3-14`.
    Range {
        /// The first week in the range.
        first: u8,
        /// The last week in the range.
        last: u8,
    },
    /// A combination of multiple week specifications. Example `1,3-5,7,8,10-12`.
    Combined(Vec<Weeks>),
}

impl Display for Weeks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weeks::Single(day) => write!(f, "{day}"),
            Weeks::Range { first, last } => write!(f, "{first}-{last}"),
            Weeks::Combined(weeks) => {
                let mut display = String::new();

                for week in weeks {
                    display.push_str(format!("{week},").as_str())
                }

                write!(f, "{}", display.trim_end_matches(','))
            }
        }
    }
}

impl FromStr for Weeks {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(&[' ', '\n'][..], "");

        let elements: Vec<&str> = s.split(',').collect();

        if elements.len() < 2 {
            // Range
            if let Some((f, l)) = s.split_once('-') {
                let first = f.parse::<u8>().map_err(|_| ScheduleError::InvalidWeeksFormat(s.to_owned()))?;
                let last = l.parse::<u8>().map_err(|_| ScheduleError::InvalidWeeksFormat(s.to_owned()))?;

                Ok(Weeks::Range { first, last })
            // Single
            } else if let Ok(day) = s.parse::<u8>() {
                return Ok(Weeks::Single(day));
            // Invalid
            } else {
                return Err(ScheduleError::InvalidWeeksFormat(s.to_owned()));
            }
        // Combined
        } else {
            let mut weeks = vec![];
            for e in elements {
                weeks.push(Weeks::from_str(e)?);
            }
    
            Ok(Weeks::Combined(weeks))
        }
    }
}

impl Default for Weeks {
    fn default() -> Self {
        Weeks::Single(1)
    }
}

/// Represents an auditorium for university lessons, which can be a distance learning auditorium (online),
/// an art center (for lections), or a pavilion with a specific room number.
#[derive(Default, Clone, Debug)]
pub enum Auditorium {
    /// Auditorium type representing distance learning.
    #[default]
    Distance,
    /// Auditorium type representing the Culture Art Center.
    ArtCenter,
    /// Auditorium type representing a pavilion with a specific number.
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

impl FromStr for Auditorium {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Дистанційно" | "Д" | "д" => Ok(Auditorium::Distance),
            "КМЦ" | "кмц" => Ok(Auditorium::ArtCenter),
            _ => Ok(Auditorium::Pavilion(AuditoriumNumber::from_str(s)?))
        }
    }
}

/// Represents an auditorium number, including a pavilion number and a room number.
///
/// The `AuditoriumNumber` struct is used to represent a specific auditorium number,
/// and it can be validated to ensure that both the pavilion and room numbers are within
/// their respective valid ranges.
#[derive(Validate, Default, Clone, Debug)]
pub struct AuditoriumNumber {
    /// The pavilion number.
    #[validate(range(min = 1, max = 9))]
    pavilion: u8,
    /// The room number.
    #[validate(range(min = 1, max = 599))]
    room: u16,
}

impl AuditoriumNumber {
    /// Creates a new `AuditoriumNumber` instance with the specified pavilion and room numbers.
    ///
    /// # Arguments
    ///
    /// * `pavilion`: The pavilion number.
    /// * `room`: The room number.
    ///
    /// # Returns
    ///
    /// A `Result` containing the created `AuditoriumNumber` if it passes validation,
    /// or validation errors if validation fails.
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

impl FromStr for AuditoriumNumber {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((pavilion, room)) = s.split_once('-') {
            Ok(AuditoriumNumber::new(
                pavilion.parse::<u8>().map_err(|_| ScheduleError::InvalidAuditorium(s.to_owned()))?, 
                room.parse::<u16>().map_err(|_| ScheduleError::InvalidAuditorium(s.to_owned()))?,
            )?)
        } else {
            Err(ScheduleError::InvalidAuditorium(s.to_owned()))
        }
    }
}

/// Represents a day of the week of the university schedule.
#[allow(clippy::missing_docs_in_private_items)]
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