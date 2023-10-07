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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schedule {
    #[serde(rename = "Факультети")]
    faculties: Vec<Faculty>,
}

impl Schedule {
    pub fn new<P: AsRef<Path>>(paths: &[P]) -> ScheduleResult<Self> {
        let mut faculties = vec![];

        for path in paths {
            faculties.push(Faculty::new(path.as_ref())?);
        }

        Ok(Schedule { faculties })
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule {
            faculties: vec![
                Faculty {
                    name: "Факультет Економічних Наук".to_owned(),
                    specialities: [(
                        SpecialityName::Management,
                        Speciality {
                            disciplines: [(
                                "Інвестування".to_owned(),
                                Discipline {
                                    groups: vec![
                                        Group {
                                            name: LessonType::Lection,
                                            time: LessonTime {
                                                from: Time::new(08, 30).unwrap(),
                                                to: Time::new(09, 50).unwrap(),
                                            },
                                            weeks: Weeks::Range { first: 1, last: 12 },
                                            auditorium: Auditorium::Pavilion(AuditoriumNumber::new(3, 205).unwrap()),
                                            day: Day::Wednesday,
                                        },
                                        Group {
                                            name: LessonType::Classes(1),
                                            time: LessonTime {
                                                from: Time::new(10, 00).unwrap(),
                                                to: Time::new(11, 20).unwrap(),
                                            },
                                            weeks: Weeks::Single(3),
                                            auditorium: Auditorium::Pavilion(AuditoriumNumber::new(1, 313).unwrap()),
                                            day: Day::Friday,
                                        }
                                    ]
                                }
                            )].into()
                        }
                    )].into(),
                }
            ]
        }
    }
}

impl_serde_display_fromstr!(SpecialityName);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Faculty {
    #[serde(rename = "Назва факультету")]
    name: String,
    #[serde(rename = "Cпеціальності")]
    specialities: Specialities,
}

impl Faculty {
    pub fn new(path: &Path) -> ScheduleResult<Self> {
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
            .map_err(|e| Error::from(e))?;
        let range = workbook.worksheet_range("Аркуш1")
            .ok_or(Error::Msg("Cannot find 'Аркуш1' sheet"))?
            .map_err(|e| Error::from(e))?;

        let mut reserved_day = Day::default();
        let mut reserved_time = LessonTime::default();

        let defined_speciality = !specialities.is_empty();

        for row in range.rows() {
            let day = match &row[0] {
                DataType::String(s) => {
                    if s == "День" {
                        continue;
                    } else {
                        reserved_day = Day::from_str(s)?;
                        reserved_day
                    }
                },
                _ => reserved_day,
            };
            
            let time = match &row[1] {
                DataType::String(s) => {
                    reserved_time = LessonTime::from_str(s)?;
                    reserved_time
                },
                _ => reserved_time,
            };

            let name = match &row[3] {
                DataType::String(s) => LessonType::from_str(s)?,
                DataType::Int(number) => LessonType::Classes(*number as u8),
                DataType::Float(number) => LessonType::Classes(*number as u8),
                DataType::Empty => continue,
                _ => Err(ScheduleError::InvalidLessonType(row[3].to_string()))?,
            };

            let weeks = match &row[4] {
                DataType::String(s) => Weeks::from_str(s)?,
                DataType::Int(number) => Weeks::Single(*number as u8),
                DataType::Float(number) => Weeks::Single(*number as u8),
                DataType::Empty => continue,
                _ => Err(ScheduleError::InvalidWeeksFormat(row[4].to_string()))?,
            };

            let auditorium = match &row[5] {
                DataType::String(s) => Auditorium::from_str(s)?,
                _ => Err(ScheduleError::InvalidAuditorium(row[5].to_string()))?
            };

            let group = Group { name, time, weeks, auditorium, day };

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Speciality {
    #[serde(rename = "Дисципліни")]
    disciplines: Disciplines,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialityName {
    SoftwareEngineering,
    Economics,
    Management,
    Finances,
    Marketing,
    #[default]
    General,
}

impl SpecialityName {
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

pub type Specialities = HashMap<SpecialityName, Speciality>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Discipline {
    #[serde(rename = "Групи")]
    groups: Vec<Group>,
}

pub type Disciplines = HashMap<String, Discipline>;