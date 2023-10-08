var searchIndex = JSON.parse('{\
"naukma_schedule":{"doc":"NaUKMA Schedule","t":"DLLLLLLAMLLLLALOLAFALLLLLNNNNNNNNEGNNLLLLLLLLLLLLLLLNEDNNENNDGNDENNNNNNDNNEMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLMLLLLLLLLMMLLMMLLLLLLLMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMMCDGNDNNNNDNGDELLLLLLLLLLLLLLLLLLLLLLLLLLMLLMLLLLLLLLLLLLLMLLLLLLMLLLLLLLMLLLLLLLLLLLLLLLLLLLLL","n":["Args","augment_args","augment_args_for_update","borrow","borrow_mut","command","command_for_update","error","files","fmt","from","from_arg_matches","from_arg_matches_mut","group","group_id","impl_serde_display_fromstr","into","macros","main","schedule","try_from","try_into","type_id","update_from_arg_matches","update_from_arg_matches_mut","InvalidAuditorium","InvalidDayOfWeek","InvalidLessonTime","InvalidLessonType","InvalidSpeciality","InvalidTimeFormat","InvalidWeeksFormat","IoError","ScheduleError","ScheduleResult","ValidationError","XlsxError","borrow","borrow_mut","fmt","fmt","from","from","from","from","into","provide","source","to_string","try_from","try_into","type_id","ArtCenter","Auditorium","AuditoriumNumber","Classes","Combined","Day","Distance","Friday","Group","GroupNumber","Lection","LessonTime","LessonType","Monday","Pavilion","Range","Saturday","Single","Thursday","Time","Tuesday","Wednesday","Weeks","auditorium","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","day","default","default","default","default","default","default","default","default","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from_str","from_str","from_str","from_str","from_str","from_str","from_str","hours","into","into","into","into","into","into","into","into","minutes","name","new","new","pavilion","room","serialize","serialize","serialize","serialize","serialize","serialize","serialize","time","to","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","validate","validate","validate_args","validate_args","weeks","first","last","impl_serde_display_fromstr","Discipline","Disciplines","Economics","Faculty","Finances","General","Management","Marketing","Schedule","SoftwareEngineering","Specialities","Speciality","SpecialityName","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","default","deserialize","deserialize","deserialize","deserialize","deserialize","disciplines","eq","equivalent","faculties","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from_discipline","from_str","groups","hash","into","into","into","into","into","name","new","new","serialize","serialize","serialize","serialize","serialize","specialities","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id"],"q":[[0,"naukma_schedule"],[25,"naukma_schedule::error"],[52,"naukma_schedule::group"],[224,"naukma_schedule::group::Weeks"],[226,"naukma_schedule::macros"],[227,"naukma_schedule::schedule"]],"d":["The command-line arguments parsing structure.","","","","","","","Custom error types and error handling for the university …","The list of file paths to university schedule Excel files.","","Returns the argument unchanged.","","","Definitions related to university disciplines’ lesson …","","Simply implements <code>Serialize</code> and <code>Deserialize</code> traits for …","Calls <code>U::from(self)</code>.","Custom macros for parsing university schedule","The entry point of the university schedule parser program.","Definitions related to the university schedule, including …","","","","","","Error indicating an invalid auditorium format.","Error indicating an invalid day of the week.","Error indicating an invalid lesson time format.","Error indicating an invalid lesson type.","Error indicating that a speciality does not exist.","Error indicating an invalid time format.","Error indicating an invalid study weeks format.","Input/Output error while reading or writing files.","Represents custom error types for the university schedule …","A type alias for results that may return a <code>ScheduleError</code>.","Validation error indicating that data does not meet …","Error related to processing .xlsx documents.","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","","","Auditorium type representing the Culture Art Center.","Represents an auditorium for university lessons, which can …","Represents an auditorium number, including a pavilion …","A classes lesson type with a group number specified.","A combination of multiple week specifications. Example …","Represents a day of the week of the university schedule.","Auditorium type representing distance learning.","","Represents a university group, including its name,  lesson …","Represents the number of a discipline group.","A lection lesson type (intended for each group).","Represents a time range for a lesson, including start and …","Represents the name of the group / type of a university …","","Auditorium type representing a pavilion with a specific …","A range of week numbers, specified by the first and last …","","A single week number. Example <code>6</code>.","","Represents a time of day, including hours and minutes.","","","Represents the weeks during which a lesson occurs in the …","The auditorium where the lesson takes place.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The day of the week when the lesson occurs.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","The starting time of the lesson.","","","","","","","","The hours component of the time","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","The minutes component of the time","The name of the group / actually type of the lesson being …","Creates a new <code>Time</code> instance with the specified hours and …","Creates a new <code>AuditoriumNumber</code> instance with the specified …","The pavilion number.","The room number.","","","","","","","","The time of the lesson.","The ending time of the lesson.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The weeks during which the lesson occurs.","The first week in the range.","The last week in the range.","","Represents a university discipline, including a list of …","A mapping of discipline names to their associated …","Economics speciality of Faculty of Economical Science.","Represents a university faculty, including its name and  a …","Finances speciality of Faculty of Economical Science.","General speciality (default when no specific speciality is …","Management speciality of Faculty of Economical Science.","Marketing speciality of Faculty of Economical Science.","Represents a university schedule, including information  …","Software Engineering speciality of Faculty of Informatics.","A mapping of university speciality names to their …","Represents a university speciality, including a collection …","Represents the names of university specialities.","","","","","","","","","","","","","","","","","","","","","","","","","","","A mapping of discipline names to their associated groups.","","","University faculties presented in schedule. Can be  parsed …","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Converts a discipline name into a list of associated …","","A list of student groups associated with this discipline.","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","The name of the faculty.","Creates a new <code>Schedule</code> by parsing university schedules …","Creates a new <code>Faculty</code> by parsing faculty data from an …","","","","","","A mapping of speciality names to their associated …","","","","","","","","","","","","","","","","","","","","",""],"i":[0,2,2,2,2,2,2,0,2,2,2,2,2,0,2,0,2,0,0,0,2,2,2,2,2,12,12,12,12,12,12,12,12,0,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,24,0,0,20,23,0,24,26,0,0,20,0,0,26,24,23,26,23,26,0,26,26,0,19,19,20,21,22,23,24,25,26,19,20,21,22,23,24,25,26,19,20,21,22,23,24,25,26,19,20,21,22,23,24,25,26,19,19,20,21,22,23,24,25,26,19,20,21,22,23,24,26,23,19,20,20,21,21,22,22,23,23,24,24,25,25,26,26,19,20,21,22,23,24,25,26,22,20,21,22,23,24,25,26,21,19,20,21,22,23,24,25,26,21,19,21,25,25,25,19,20,21,22,23,24,26,19,22,19,20,21,22,23,24,25,26,20,21,22,23,24,25,26,19,20,21,22,23,24,25,26,19,20,21,22,23,24,25,26,19,20,21,22,23,24,25,26,21,25,21,25,19,43,43,0,0,0,36,0,36,36,36,36,0,36,0,0,0,33,34,35,36,37,33,34,35,36,37,33,34,35,36,37,33,34,35,36,37,36,33,34,35,36,37,35,36,36,33,33,34,35,36,36,37,33,34,35,36,37,36,36,37,36,33,34,35,36,37,34,33,34,33,34,35,36,37,34,33,34,35,36,37,36,33,34,35,36,37,33,34,35,36,37,33,34,35,36,37],"f":[0,[1,1],[1,1],[[]],[[]],[[],1],[[],1],0,0,[[2,3],4],[[]],[5,[[7,[2,6]]]],[5,[[7,[2,6]]]],0,[[],[[9,[8]]]],0,[[]],0,[[],10],0,[[],7],[[],7],[[],11],[[2,5],[[7,[6]]]],[[2,5],[[7,[6]]]],0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[12,3],4],[[12,3],4],[13,12],[[]],[14,12],[15,12],[[]],[16],[12,[[9,[17]]]],[[],18],[[],7],[[],7],[[],11],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[19,19],[20,20],[21,21],[22,22],[23,23],[24,24],[25,25],[26,26],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[[],19],[[],20],[[],21],[[],22],[[],23],[[],24],[[],25],[[],26],[27,[[7,[19]]]],[27,[[7,[20]]]],[27,[[7,[21]]]],[27,[[7,[22]]]],[27,[[7,[23]]]],[27,[[7,[24]]]],[27,[[7,[26]]]],[[23,23],28],[[19,3],4],[[20,3],4],[[20,3],4],[[21,3],4],[[21,3],4],[[22,3],4],[[22,3],4],[[23,3],4],[[23,3],4],[[24,3],4],[[24,3],4],[[25,3],4],[[25,3],4],[[26,3],4],[[26,3],4],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[29,[[7,[20]]]],[29,[[7,[21]]]],[29,[[7,[22]]]],[29,[[7,[23]]]],[29,[[7,[24]]]],[29,[[7,[25]]]],[29,[[7,[26]]]],0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,[[30,30],[[7,[21,14]]]],[[30,31],[[7,[25,14]]]],0,0,[[19,32],7],[[20,32],7],[[21,32],7],[[22,32],7],[[23,32],7],[[24,32],7],[[26,32],7],0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[21,[[7,[14]]]],[25,[[7,[14]]]],[21,[[7,[14]]]],[25,[[7,[14]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[33,33],[34,34],[35,35],[36,36],[37,37],[[]],[[]],[[]],[[]],[[]],[[],36],[27,[[7,[33]]]],[27,[[7,[34]]]],[27,[[7,[35]]]],[27,[[7,[36]]]],[27,[[7,[37]]]],0,[[36,36],28],[[],28],0,[[33,3],4],[[34,3],4],[[35,3],4],[[36,3],4],[[36,3],4],[[37,3],4],[[]],[[]],[[]],[[]],[[]],[29,[[38,[36]]]],[29,[[7,[36]]]],0,[[36,39]],[[]],[[]],[[]],[[]],[[]],0,[[[42,[[41,[40]]]]],[[7,[33,12]]]],[40,[[7,[34,12]]]],[[33,32],7],[[34,32],7],[[35,32],7],[[36,32],7],[[37,32],7],0,[[]],[[]],[[]],[[]],[[]],[[],18],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],11],[[],11],[[],11],[[],11],[[],11]],"c":[],"p":[[3,"Command"],[3,"Args"],[3,"Formatter"],[6,"Result"],[3,"ArgMatches"],[6,"Error"],[4,"Result"],[3,"Id"],[4,"Option"],[6,"Result"],[3,"TypeId"],[4,"ScheduleError"],[4,"Error"],[3,"ValidationErrors"],[3,"Error"],[3,"Demand"],[8,"Error"],[3,"String"],[3,"Group"],[4,"LessonType"],[3,"Time"],[3,"LessonTime"],[4,"Weeks"],[4,"Auditorium"],[3,"AuditoriumNumber"],[4,"Day"],[8,"Deserializer"],[15,"bool"],[15,"str"],[15,"u8"],[15,"u16"],[8,"Serializer"],[3,"Schedule"],[3,"Faculty"],[3,"Speciality"],[4,"SpecialityName"],[3,"Discipline"],[3,"Vec"],[8,"Hasher"],[3,"Path"],[8,"AsRef"],[15,"slice"],[13,"Range"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
