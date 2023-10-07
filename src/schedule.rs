use std::{path::Path, str::FromStr};
use serde::Serialize;
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder, DataType};

use crate::{
    group::*, 
    error::{ScheduleResult, ScheduleError}
};

#[derive(Serialize, Clone, Debug)]
pub struct Schedule {
    #[serde(rename = "Факультети")]
    faculties: Vec<Faculty>,
}

impl Schedule {
    pub fn new<P: AsRef<Path>>(paths: &[P]) -> ScheduleResult<Self> {
        let mut faculties = vec![];

        for path in paths {
            faculties.push(Schedule::new_faculty(path.as_ref())?); // TODO: Single/Multiple specialities separation
        }

        Ok(Schedule {
            faculties,
        })
    }

    fn new_faculty(path: &Path) -> ScheduleResult<Faculty> {
        let mut workbook: Xlsx<_> = open_workbook(path)
            .map_err(|e| Error::from(e))?;
        let range = workbook.worksheet_range("Аркуш1")
            .ok_or(Error::Msg("Cannot find 'Аркуш1' sheet"))?
            .map_err(|e| Error::from(e))?;

        let mut reserved_day = Day::default();
        let mut reserved_time = LessonTime::default();

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

            println!("Day: {day}, Time: {time}, Name: {name}, Weeks: {weeks}");
        }

        Err(ScheduleError::Dummy)
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule {
            faculties: vec![
                Faculty {
                    name: "Факультет Економічних Наук".to_owned(),
                    specialities: vec![Speciality {
                        name: "Менеджмент".to_owned(),
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
                    }],
                }
            ]
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Faculty {
    #[serde(rename = "Назва факультету")]
    name: String,
    #[serde(rename = "Cпецільності")]
    specialities: Vec<Speciality>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Speciality {
    #[serde(rename = "Назва спеціальності")]
    name: String,
    #[serde(rename = "Групи")]
    groups: Vec<Group>,
}

// fn parse_docx(file_name: &str) -> anyhow::Result<()> {
//     let docx = read_docx(&read_to_vec(file_name)?)?;
//     let document = docx.document;

//     let tables: Vec<Table> = document.children.iter().filter_map(|c| { 
//         if let DocumentChild::Table(t) = c { 
//             return Some((**t).clone());
//         } 

//         None
//     }).collect();

//     std::fs::write("document", format!("{tables:#?}"))?;

//     Ok(())
// }

// fn read_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
//     let mut buf = Vec::new();
//     std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
//     Ok(buf)
// }