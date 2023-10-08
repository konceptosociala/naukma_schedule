use std::fmt::Display;
use std::vec;
use std::{path::Path, collections::HashMap};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use calamine::{open_workbook, Error, Xlsx, Reader, DataType};

use crate::{
    group::*, 
    macros::impl_serde_display_fromstr,
    error::{ScheduleResult, ScheduleError}
};

/// Represents a university schedule, including information 
/// about university faculties.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schedule {
    /// University faculties presented in schedule. Can be 
    /// parsed and added manually as well.
    #[serde(rename = "Факультети")]
    pub faculties: Vec<Faculty>,
}

impl Schedule {
    /// Creates a new `Schedule` by parsing university schedules from Excel files.
    ///
    /// # Arguments
    ///
    /// * `paths`: A slice of paths to Excel files containing faculty schedules.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Schedule` if successful, or an error if parsing fails.
    pub fn new<P: AsRef<Path>>(paths: &[P]) -> ScheduleResult<Self> {
        let mut faculties = vec![];

        for path in paths {
            faculties.push(Faculty::new(path.as_ref())?);
        }

        Ok(Schedule { faculties })
    }
}

impl_serde_display_fromstr!(SpecialityName);

/// Represents a university faculty, including its name and 
/// a collection of specialities with their schedules.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Faculty {
    /// The name of the faculty.
    #[serde(rename = "Назва факультету")]
    name: String,
    /// A mapping of speciality names to their associated specialities.
    #[serde(rename = "Cпеціальності")]
    specialities: Specialities,
}

impl Faculty {
    /// Creates a new `Faculty` by parsing faculty data from an Excel file.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the Excel file containing faculty schedule data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Faculty` if successful, or an error if parsing fails.
    pub fn new(path: &Path) -> ScheduleResult<Self> {
        // Get faculty name (and optionally a defined speciality name)
        let (name, mut specialities) = {
            let elements: Vec<&str> = path.to_str().unwrap().split('.').collect();
            let mut specialities = Specialities::new();

            if elements.len() == 3 {
                specialities.insert(
                    SpecialityName::from_str(elements[1])?,
                    Speciality { disciplines: Disciplines::new() }
                );
            }

            (elements[0].to_owned(), specialities)
        };

        let mut workbook: Xlsx<_> = open_workbook(path)
            .map_err(Error::from)?;
        let range = workbook.worksheet_range("Аркуш1")
            .ok_or(Error::Msg("Cannot find 'Аркуш1' sheet"))?
            .map_err(Error::from)?;

        let mut reserved_day = Day::default();
        let mut reserved_time = LessonTime::default();

        let defined_speciality = !specialities.is_empty();

        for row in range.rows() {
            // Get day of the week
            let day = match &row[0] {
                DataType::String(s) => {
                    // Skip first header row
                    if s == "День" {
                        continue;
                    } else {
                        reserved_day = Day::from_str(s)?;
                        reserved_day
                    }
                },
                _ => reserved_day,
            };
            
            // Get lesson time
            let time = match &row[1] {
                DataType::String(s) => {
                    reserved_time = LessonTime::from_str(s)?;
                    reserved_time
                },
                _ => reserved_time,
            };

            // Get lesson type (group number or a lection)
            let name = match &row[3] {
                DataType::String(s) => LessonType::from_str(s)?,
                DataType::Int(number) => LessonType::Classes(*number as u8),
                DataType::Float(number) => LessonType::Classes(*number as u8),
                DataType::Empty => continue,
                _ => Err(ScheduleError::InvalidLessonType(row[3].to_string()))?,
            };

            // Get studying weeks
            let weeks = match &row[4] {
                DataType::String(s) => Weeks::from_str(s)?,
                DataType::Int(number) => Weeks::Single(*number as u8),
                DataType::Float(number) => Weeks::Single(*number as u8),
                DataType::Empty => continue,
                _ => Err(ScheduleError::InvalidWeeksFormat(row[4].to_string()))?,
            };

            // Get auditorium number (may be also art center or distance)
            let auditorium = match &row[5] {
                DataType::String(s) => Auditorium::from_str(s)?,
                _ => Err(ScheduleError::InvalidAuditorium(row[5].to_string()))?
            };

            let group = Group { name, time, weeks, auditorium, day };

            // If speciality defined in the filename, use it in parsing
            // and just copy discipline name without processing.
            //
            // Else if not defined, parse speciality names from
            // discipline names and write them to the specialities collection.
            // If not present, prefer `General` speciality (for common
            // lection attendance)
            if defined_speciality {
                let discipline = row[2].to_string().replace("  ", " ").replace('\n', "");
                let spec = specialities.values_mut().next().unwrap();
                match spec.disciplines.get_mut(&discipline) {
                    Some(disc) => disc.groups.push(group),
                    _ => {
                        spec.disciplines.insert(discipline, Discipline { 
                            groups: vec![group]
                        });
                    }
                }
            } else {
                let discipline = row[2].to_string().replace("  ", " ").replace('\n', "");
                let speciality_names = SpecialityName::from_discipline(&discipline);
                
                for name in speciality_names {
                    if let Some(spec) = specialities.get_mut(&name) {
                        match spec.disciplines.get_mut(&discipline) {
                            Some(disc) => disc.groups.push(group.clone()),
                            _ => {
                                spec.disciplines.insert(discipline.clone(), Discipline { 
                                    groups: vec![group.clone()]
                                });
                            }
                        }
                    } else {
                        specialities.insert(name, Speciality {
                            disciplines: [(discipline.clone(), Discipline {
                                groups: vec![group.clone()],
                            })].into()
                        });
                    }
                }
            }
        }

        Ok(Faculty { name, specialities })
    }
}

/// Represents a university speciality, including a collection of disciplines.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Speciality {
    /// A mapping of discipline names to their associated groups.
    #[serde(rename = "Дисципліни")]
    disciplines: Disciplines,
}

/// Represents the names of university specialities.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialityName {
    /// Software Engineering speciality of Faculty of Informatics.
    SoftwareEngineering,
    /// Economics speciality of Faculty of Economical Science.
    Economics,
    /// Management speciality of Faculty of Economical Science.
    Management,
    /// Finances speciality of Faculty of Economical Science.
    Finances,
    /// Marketing speciality of Faculty of Economical Science.
    Marketing,
    /// General speciality (default when no specific speciality is identified).
    #[default]
    General,
}

impl SpecialityName {
    /// Converts a discipline name into a list of associated speciality names.
    ///
    /// # Arguments
    ///
    /// * `discipline`: The name of the discipline with defined 
    /// specialities (e.g. `(марк.)`, `(екон.+фін.)`).
    ///
    /// # Returns
    ///
    /// Parsed speciality names representing the associated specialities.
    pub fn from_discipline(discipline: &str) -> Vec<SpecialityName> {
        use SpecialityName::*;

        let mut names = vec![];

        let elements = {
            let left_bracket: Vec<&str> = discipline.split('(').collect();
            let mut right_bracket = vec![];
            let mut plus = vec![];
            let mut elements = vec![];

            for e in left_bracket {
                for element in e.split(')').collect::<Vec<&str>>() {
                    right_bracket.push(element);
                }
            }

            for e in right_bracket {
                for element in e.split('+').collect::<Vec<&str>>() {
                    plus.push(element);
                }
            }

            for e in plus {
                for element in e.split(',').collect::<Vec<&str>>() {
                    elements.push(element);
                }
            }

            elements
        };

        for e in elements {
            match e {
                "ек" | "ек." | "екон." | "екон" | "економіка" => names.push(Economics),
                "мен." | "мен" | "менеджмент" => names.push(Management),
                "фін." | "фін" | "фінанси" => names.push(Finances),
                "мар." | "марк." | "мар" | "марк" | "маркетинг" => names.push(Marketing),
                _ => {},
            }
        }

        if names.is_empty() {
            names.push(General);
        }

        names
    }
}

impl Display for SpecialityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SpecialityName::*;

        match self {
            SoftwareEngineering => write!(f, "Інженерія програмного забезпечення"),
            Economics => write!(f, "Економіка"),
            Management => write!(f, "Менеджмент"),
            Finances => write!(f, "Фінанси"),
            Marketing => write!(f, "Маркетинг"),
            General => write!(f, "<загальна>"),
        }
    }
}

impl FromStr for SpecialityName {
    type Err = ScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SpecialityName::*;

        match s {
            "Інженерія програмного забезпечення" => Ok(SoftwareEngineering),
            "Економіка" => Ok(Economics),
            "Менеджмент" => Ok(Management),
            "Фінанси" => Ok(Finances),
            "Маркетинг" => Ok(Marketing),
            "<загальна>" => Ok(General),
            _ => Err(ScheduleError::InvalidSpeciality(s.to_owned()))
        }
    }
}

/// A mapping of university speciality names to their associated specialities.
pub type Specialities = HashMap<SpecialityName, Speciality>;

/// Represents a university discipline, including a list of associated groups.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Discipline {
    /// A list of student groups associated with this discipline.
    #[serde(rename = "Групи")]
    groups: Vec<Group>,
}

/// A mapping of discipline names to their associated disciplines.
pub type Disciplines = HashMap<String, Discipline>;