use serde::*;
use crate::model;
use crate::page::school::detail;
use crate::page::school::group::timetable::{Activity, ClassAvailable, NewClassTimetable};
use crate::model::user::Teacher;
use crate::model::class::Class;
use crate::page::school::group::generate::put_activity;
use crate::page::school::detail::GroupContext;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tests{
    pub activity: Vec<Act>,
    pub teachers: Vec<Teacher>,
    pub classes: Vec<Class>,
    pub test4: Vec<Teacher>,
    pub test5: Vec<Class>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Act{
    pub teacher: ActTeacher,
    pub class: ActClass
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActClass{
    pub kademe: String,
    pub sube: String
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActTeacher{
    pub first_name: String,
    pub last_name: String
}

pub fn tests(
    acts: &Vec<Activity>,
    max_day_hour: i32,
    tests: &mut Tests,
    ctx_school: &mut detail::SchoolContext,
    ctx_group: &mut GroupContext,
    tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
    cat: &Vec<ClassAvailable>
){
    let _fail_acts: Vec<Activity> = acts.iter().cloned().filter(|a| a.hour > max_day_hour as i16).collect();
    tests.classes.clear();
    tests.teachers.clear();
    tests.activity.clear();
    tests.test4.clear();
    tests.test5.clear();
    for a in acts{
        if a.hour > max_day_hour as i16 && max_day_hour != 0{
            let teacher = ctx_school.teachers.iter().find(|t| Some(t.id) == a.teacher).unwrap();
            let class = ctx_group.classes.iter().find(|c| a.classes.iter().any(|a2| a2 == &c.id)).unwrap();
            let act = Act{
                teacher: ActTeacher{
                    first_name: teacher.first_name.clone(),
                    last_name: teacher.last_name.clone()
                },
                class: ActClass{ kademe: class.kademe.clone(), sube: class.sube.clone() }
            };
            tests.activity.push(act);
        }
    }
    for teacher in &ctx_school.teachers{
        let total_act_hour = acts.iter().fold(0,|a, b| if b.teacher==Some(teacher.id){a+b.hour} else{a});
        let mut total_available_hour = 0;
        for t in tat{
            if t.user_id == teacher.id{
                for h in &t.hours{
                    if *h{
                        total_available_hour += 1;
                    }
                }
            }
        }
        if total_act_hour > total_available_hour || tat.len() == 0{
            tests.teachers.push(teacher.clone())
        }
    }
    for class in &ctx_group.classes{
        let total_act_hour = acts.iter().fold(0,|a, b| if b.classes.iter().any(|b2| b2 == &class.id){a+b.hour} else{a});
        let mut total_available_hour = 0;
        for c in cat{
            if c.class_id == class.id{
                for h in &c.hours{
                    if *h{
                        total_available_hour += 1;
                    }
                }
            }
        }
        if total_act_hour > total_available_hour || cat.len() == 0{
            tests.classes.push(class.clone())
        }
    }

    for teacher in &ctx_school.teachers{
        use crate::page::school::group::generate::recursive_put;
        use crate::page::school::group::generate::find_timeslot;
        let mut tat2 = tat.clone();
        let mut cat2 = cat.clone();
        let mut timetables: Vec<NewClassTimetable> = vec![];
        let t_acts: Vec<Activity> = acts.clone().into_iter().filter(|a| a.teacher == Some(teacher.id)).collect();
        for a in t_acts{
            let available = find_timeslot(&a, acts, &tat2, &timetables, &cat2, tat, max_day_hour, false);
            match available {
                Some(slots) => {
                    put_activity(&a, acts, &mut tat2, &mut timetables, &mut cat2, slots[0].0, slots[0].1);
                },
                None => {
                    let rec_result = recursive_put(&a, acts, &mut timetables, tat, &mut tat2, &mut cat2, max_day_hour, 0, 7);
                    if rec_result {
                    }
                    else {
                        //tests.test4.push(teacher.clone());
                        break;
                    }
                }
            }
        }
    }
    for class in &ctx_group.classes{
        use crate::page::school::group::generate::recursive_put;
        use crate::page::school::group::generate::find_timeslot;
        let mut tat2 = tat.clone();
        let mut cat2 = cat.clone();
        let mut timetables: Vec<NewClassTimetable> = vec![];
        let c_acts: Vec<Activity> = acts.clone().into_iter().filter(|a| a.classes.iter().any(|aa| aa == &class.id)).collect();
        for a in c_acts{
            let available = find_timeslot(&a, acts, &tat2, &timetables, &cat2, tat, max_day_hour, false);
            match available {
                Some(slots) => {
                    put_activity(&a, acts, &mut tat2, &mut timetables, &mut cat2, slots[0].0, slots[0].1);
                },
                None => {
                    let rec_result = recursive_put(&a, acts, &mut timetables, tat, &mut tat2, &mut cat2, max_day_hour, 0, 8);
                    if rec_result {
                    }
                    else {
                        //tests.test5.push(class.clone());
                        break;
                    }
                }
            }
        }
    }
}