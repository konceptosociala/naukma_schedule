use std::path::Path;
use serde::Serialize;

use crate::{
    group::*, 
    error::ScheduleResult
};

#[derive(Serialize, Clone, Debug)]
pub struct Schedule {
    #[serde(rename = "Факультети")]
    faculties: Vec<Faculty>,
}

impl Schedule {
    pub fn new(paths: &[impl AsRef<Path>]) -> ScheduleResult<Self> {
        let mut faculties = vec![];

        for path in paths {
            faculties.push(Schedule::new_faculty(path.as_ref()));
        }

        Ok(Schedule {
            faculties,
        })
    }

    fn new_faculty(path: &Path) -> Faculty {
        todo!();
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
                                weeks: Weeks::new(1, 12).unwrap(),
                                auditorium: Auditorium::Pavilion(AuditoriumNumber::new(3, 205).unwrap()),
                                day: Day::Wednesday,
                            },
                            Group {
                                name: LessonType::Classes(1),
                                time: LessonTime {
                                    from: Time::new(10, 00).unwrap(),
                                    to: Time::new(11, 20).unwrap(),
                                },
                                weeks: Weeks::new(3, 14).unwrap(),
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