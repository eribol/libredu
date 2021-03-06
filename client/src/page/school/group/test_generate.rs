use serde::*;
use crate::model;
use crate::page::school::detail;
use crate::page::school::group::timetable::{ClassAvailable};
use crate::model::teacher::Teacher;
use crate::model::class::Class;
use seed::Url;
use crate::model::activity::Activity;
use crate::model::teacher;


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Tests{
    pub activity: Vec<Act>,
    pub teachers: Vec<Teacher>,
    pub classes: Vec<Class>,
    pub test4: Vec<Teacher>,
    pub test5: Vec<Class>
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Act{
    pub teacher: Vec<ActTeacher>,
    pub class: Vec<ActClass>
}

#[derive(Clone, Default, Serialize, Deserialize)]
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
    acts: &[Activity],
    max_day_hour: i32,
    tests: &mut Tests,
    school_ctx: &mut detail::SchoolContext,
    tat: &[model::teacher::TeacherAvailableForTimetables],
    cat: &[ClassAvailable],
    url: Url
){
    let _fail_acts: Vec<Activity> = acts.iter().cloned().filter(|a| a.hour > max_day_hour as i16).collect();
    tests.classes.clear();
    tests.teachers.clear();
    tests.activity.clear();
    tests.test4.clear();
    tests.test5.clear();
    for a in acts{
        if a.hour > max_day_hour as i16 && max_day_hour != 0{
            let group_ctx = school_ctx.get_group(&url);
            if let Some(teachers) = &school_ctx.teachers{
                if let Some(classes) = &group_ctx.classes{
                    let teacher = teachers.iter().filter(|t| a.teachers.iter().any(|t2| t2 == t.teacher.id)).collect::<Vec<teacher::TeacherContext>>();
                    let class = classes.iter().find(|c| a.classes.iter().any(|a2| a2 == &c.class.id)).unwrap();
                    let act = Act{
                        teacher: ActTeacher{
                            first_name: teacher.teacher.first_name.clone(),
                            last_name: teacher.teacher.last_name.clone()
                        },
                        class: ActClass{ kademe: class.class.kademe.clone(), sube: class.class.sube.clone() }
                    };
                    tests.activity.push(act);
                }
            }

        }
    }
    if let Some(teachers ) = &school_ctx.teachers{
        for teacher in teachers{
            let total_act_hour = acts.iter().fold(0,|a, b| if !b.teachers.is_empty(){a+b.hour} else{a});
            let mut total_available_hour = 0;
            for t in tat{
                if t.user_id == teacher.teacher.id{
                    for h in &t.hours{
                        if *h{
                            total_available_hour += 1;
                        }
                    }
                }
            }
            if total_act_hour > total_available_hour || tat.is_empty(){
                tests.teachers.push(teacher.teacher.clone())
            }
        }
    }
    let group_ctx = school_ctx.get_group(&url);
    if let Some(classes) = &group_ctx.classes{
        for class in classes{
            let total_act_hour = acts.iter().fold(0,|a, b| if b.classes.iter().any(|b2| b2 == &class.class.id){a+b.hour} else{a});
            let mut total_available_hour = 0;
            for c in cat{
                if c.class_id == class.class.id{
                    for h in &c.hours{
                        if *h{
                            total_available_hour += 1;
                        }
                    }
                }
            }
            if total_act_hour > total_available_hour || cat.is_empty(){
                tests.classes.push(class.class.clone())
            }
        }
    }

    /*
    for teacher in &school_ctx.teachers{
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
                    let rec_result = recursive_put(&a, acts, &mut timetables, tat, &mut tat2, &mut cat2, max_day_hour, 0, 4, &mut vec![]);
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
                    let rec_result = recursive_put(&a, acts, &mut timetables, tat, &mut tat2, &mut cat2, max_day_hour, 0, 4, &mut vec![]);
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
    */
}