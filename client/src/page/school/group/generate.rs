use rand::seq::SliceRandom;
use seed::{*};
use crate::model;
use crate::page::school::group::timetable::{ClassAvailable};
use crate::model::timetable::NewClassTimetable;
use crate::model::activity::Activity;
use crate::model::teacher::TeacherAvailableForTimetables;
//use async_std::prelude::*;
//use async_std::task;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
pub(crate) fn generate(
    max_day_hour: i32,
    max_depth: usize,
    _depth2: usize,
    tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
    cat: &mut Vec<ClassAvailable>,
    total_acts: &[Activity],
    timetables: &mut Vec<NewClassTimetable>,
    clean_tat: &[model::teacher::TeacherAvailableForTimetables],
    error: &mut String
)
    -> bool {
    use rand::thread_rng;
    let mut acts: Vec<Activity> = total_acts.iter().cloned()
        .filter(|a| !a.teachers.is_empty() && !timetables.iter().cloned()
            .any(|t| a.id == t.activities.unwrap())).collect();
    acts.shuffle(&mut thread_rng());
    acts.sort_by(|a, b| b.hour.cmp(&a.hour));
    /*if timetables.len() == 0{
        acts.sort_by(|a, b|
            b.timeslots(&total_acts,&tat,&timetables,&cat, &clean_tat,max_day_hour, false).cmp(
                &a.timeslots(&total_acts,&tat,&timetables,&cat, &clean_tat,max_day_hour, false)
            )
        );
        //
    }*/
    if acts.is_empty() {
        //let mut fivea: Vec<NewClassTimetable> = timetables.iter().cloned().filter(|t| t.class_id.unwrap() == 1).collect();
        //timetables.sort_by(|a, b| a.day_id.cmp(&b.day_id));
        //fivea.sort_by(|a, b| b.hour.cmp(&a.hour));
        return false
    }
    //log!(acts);
    let act2 = &acts[0].clone();
    let available = find_timeslot(act2, &total_acts, &tat, &timetables, &cat, clean_tat, max_day_hour, false);
    match available {
        Some(slots) => {
            put_activity(act2, total_acts, tat, timetables, cat, slots[0].0, slots[0].1, clean_tat);
            true
        },
        None => {
            let timetables_backup = timetables.clone();
            let tat_backup = tat.clone();
            let cat_backup = cat.clone();
            let rec_result = recursive_put(&act2, total_acts, timetables, clean_tat, tat, cat, max_day_hour, 0, max_depth, &mut vec![],&tat_backup, &cat_backup, &timetables_backup);
            if rec_result {
                true
            }
            else {
                *timetables = timetables_backup;
                *tat = tat_backup;
                *cat = cat_backup;
                //log!("oh", &act2.teacher, &act2.classes);
                let mut conflict_acts = find_conflict_activity(&act2, &total_acts, &timetables, clean_tat, &tat, &cat, max_day_hour, max_depth, &mut vec![]);
                if conflict_acts.is_empty() {
                    log!("Çakışan aktivite yok");
                    *error = "Sınıf ile öğretmenin uyumlu uygun saatleri mevcut değil. Kısıtlamaları kontrol edin.".to_string();
                    return false;
                }
                conflict_acts.shuffle(&mut thread_rng());
                let mut c_act = conflict_acts[0].clone();
                for a in &c_act {
                    delete_activity(total_acts, a, tat, timetables, cat, true);
                }
                c_act.insert(0, act2.clone());
                for a in &c_act{
                    let available2 = find_timeslot(a, &total_acts, &tat, &timetables, &cat, clean_tat, max_day_hour, false);
                    if let Some(slots2) = available2 {
                        put_activity(a, total_acts, tat, timetables, cat, slots2[0].0, slots2[0].1, clean_tat);
                    }
                }
                true
            }
        }
    }
}
//Add all timetables array to database

#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
pub(crate) fn recursive_put(
    act: &Activity,
    _acts: &[Activity],
    timetables: &mut Vec<NewClassTimetable>,
    clean_tat: &[model::teacher::TeacherAvailableForTimetables],
    tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
    cat: &mut Vec<ClassAvailable>,
    max_day_hour: i32,
    depth: usize,
    //depth2: usize,
    max_depth: usize,
    ignore_list: &mut Vec<Activity>,
    tat2: &[model::teacher::TeacherAvailableForTimetables],
    cat2: &[ClassAvailable],
    timetables2: &[NewClassTimetable]
)
    -> bool {
    let mut conflict_acts = find_conflict_activity(act, &_acts, &timetables, &clean_tat, &tat, &cat, max_day_hour, max_depth, ignore_list);
    //let start = Instant::now();
    use rand::thread_rng;
    let mut okey2 = false;
    if !ignore_list.iter().any(|i| i.id == act.id){
        ignore_list.push(act.clone());
    }
    conflict_acts.shuffle(&mut thread_rng());
    for conflict_act in &conflict_acts {
        let mut c_act = conflict_act.clone();
        for a in &c_act {
            delete_activity(_acts, a, tat, timetables, cat, true);
        }
        //let mut c_act2: Vec<Activity> = Vec::new();
        //c_act.shuffle(&mut thread_rng());
        //c_act.sort_by(|a, b| b.hour.cmp(&a.hour));
        c_act.insert(0, act.clone());
        //ignore_list.append(&mut c_act.clone());
        let mut okey = true;
        for a in &c_act {
            let available = find_timeslot(a, &_acts, &tat, &timetables, &cat, clean_tat, max_day_hour, true);
            match available {
                Some(slots) => {
                    put_activity(a, _acts, tat, timetables, cat, slots[0].0, slots[0].1, clean_tat);
                },
                None => {
                    if depth < 5 {
                        let rec_result = recursive_put(a, _acts, timetables, &clean_tat, tat, cat, max_day_hour, depth + 1, max_depth, ignore_list, &tat.clone(), &cat.clone(), &timetables.clone());
                        if !rec_result {
                            okey = false;
                            break;
                        }
                    }
                    else {
                        okey = false;
                        break;
                    }
                }
            }
        }
        if okey {
            okey2 = true;
            ignore_list.retain(|a3| a3.id != act.id);
            break;
        }
        else {
            *tat = tat2.to_owned();
            *cat = cat2.to_owned();
            *timetables = timetables2.to_owned();
            //ignore_list.retain(|a3| !c_act.iter().any(|a4| a4.id == a3.id));
            okey2 = false;
            //break;
        }

    }
    okey2
}


fn delete_activity(_acts: &[Activity],act: &Activity, tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
                   timetables: &mut Vec<NewClassTimetable>, cat: &mut Vec<ClassAvailable>, _a: bool)-> bool
{
    let tt: Vec<(usize, NewClassTimetable)> = timetables.iter().cloned()
        .enumerate()
        .filter(|t| t.1.activities.unwrap() == act.id).collect();

    for t in &tt{
        let tat_index: Vec<(usize, TeacherAvailableForTimetables)> = tat.iter().cloned()
            .enumerate()
            .filter(|ta| act.teachers.iter().any(|t| t == &ta.1.user_id) && ta.1.day == t.1.day_id.unwrap()).collect();
        for index in tat_index{
            tat[index.0].hours[t.1.hour.unwrap() as usize] = true;
        }

        let c_index: Vec<(usize, ClassAvailable)> = cat.iter().cloned()
            .enumerate()
            .filter(|c|  act.classes.iter().any(|cc| cc == &c.1.class_id) && c.1.day == t.1.day_id.unwrap() ).collect();
        for index in c_index{
            cat[index.0].hours[t.1.hour.unwrap() as usize] = true;
        }
        /*let find_class = cat.iter().cloned()
            .enumerate()
            .find(|c| c.1.class_id == act.class && c.1.day == t.1.day_id.unwrap()).unwrap();
        cat[find_class.0].hours[t.1.hour.unwrap() as usize] = true;*/
        //timetables.remove(t.0);
    }
    timetables.retain(|t| t.activities.unwrap() != act.id);
    true
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
fn find_conflict_activity(
    act: &Activity,
    _acts: &[Activity],
    timetables: &[NewClassTimetable],
    clean_tat: &[model::teacher::TeacherAvailableForTimetables],
    _tat: &[model::teacher::TeacherAvailableForTimetables],
    cat: &[ClassAvailable],
    max_day_hour: i32,
    depth: usize,
    ignore_list: &mut Vec<Activity>
)->Vec<Vec<Activity>>
{
    //let class = act.class.unwrap();
    let mut total_act: Vec<Vec<Activity>> = Vec::new();

    let activities: Vec<Activity> = _acts.to_owned().into_iter()
        .filter(|a| act.classes.iter().any(|c1| a.classes.iter().any(|c2| c1 == c2)) || a.teachers.iter().any(|t| act.teachers.iter().any(|t2| t2 == t))).collect();
    let teacher_availables: Vec<model::teacher::TeacherAvailableForTimetables> = clean_tat.to_owned().into_iter()
        .filter(|t| act.teachers.iter().any(|t2| t2 == &t.user_id)).collect();
    //teacher_availables.sort_by(|a, b| a.hours.iter().fold(0, |acc, x| if *x{ acc+1} else{acc}).cmp(&b.hours.iter().fold(0, |acc, x| if *x{acc+1}else{acc})));
    //teacher_availables.shuffle(&mut thread_rng());
    for teacher_available in &teacher_availables{
        for h in 0..teacher_available.hours.len(){
            if h + act.hour as usize <= teacher_available.hours.len() {
                let available = (h..h+act.hour as usize)
                    .all(|h| teacher_available.hours[h]);
                if available {
                    let mut less_conflict: Vec<Activity> = Vec::new();
                    for i in h..h+act.hour as usize{
                        let conflict_slot: Vec<NewClassTimetable> = timetables.to_owned().into_iter()
                            .filter(|t| t.day_id.unwrap() == teacher_available.day && t.hour.unwrap() as usize == i).collect();

                        for c in &conflict_slot{
                            let activity = activities.iter()
                                .find(|a| a.id == c.activities.unwrap() && a.id != act.id && !ignore_list.iter().any(|a2| a2.id == a.id));
                            if let Some(a) = activity {
                                less_conflict.push(a.clone());
                            }
                        }
                    }
                    let mut copy_tat = _tat.to_owned();
                    let mut copy_ch = cat.to_owned();
                    let mut copy_tt = timetables.to_owned();
                    let acts = _acts.to_owned();
                    for a in &less_conflict{
                        delete_activity(&acts, a, &mut copy_tat, &mut copy_tt, &mut copy_ch, true);
                    }
                    let available = find_timeslot(act, &acts, &copy_tat, &copy_tt, &copy_ch, &clean_tat, max_day_hour, true);
                    if available.is_some() {
                        total_act.push(less_conflict);
                    }
                }
            }
        }
    }
    //total_act.shuffle(&mut thread_rng());
    total_act.sort_by_key(|a| a.len());
    for item in &mut total_act{
        item.sort_by(|a,b| a.id.cmp(&b.id));
        item.dedup();
    }
    if total_act.len()>=depth{
        return total_act[0..depth].to_vec();
    }
    total_act
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
pub fn put_activity(
    act: &Activity,
    _total_acts: &[Activity],
    tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
    timetables: &mut Vec<NewClassTimetable>,
    cat: &mut Vec<ClassAvailable>,
    day:i32,
    hour: usize,
    _clean_tat: &[model::teacher::TeacherAvailableForTimetables]
)-> bool
{
    //delete_activity(total_acts, act, tat, timetables, cat, true);
    for timetable in hour..hour+act.hour as usize{
        let tt = NewClassTimetable{
            class_id : Some(act.classes[0]),
            day_id : Some(day),
            hour: Some(timetable as i16),
            activities: Some(act.id)
        };
        //Close the teacher available hours
        let tat_index: Vec<(usize, TeacherAvailableForTimetables)> = tat.iter().cloned()
            .enumerate()
            .filter(|t| act.teachers.iter().any(|t2| t2 == &t.1.user_id) && t.1.day==day).collect();
        for index in tat_index{
            tat[index.0].hours[timetable]= false;
        }
        //Close the class available hours
        let c_index: Vec<(usize, ClassAvailable)> = cat.iter().cloned()
            .enumerate()
            .filter(|c| act.classes.iter().any(|cc| cc == &c.1.class_id) && c.1.day == day ).collect();
        for index in c_index{
            cat[index.0].hours[timetable] = false;
        }
        //Add timetable to timetables Array
        timetables.push(tt);

        //total_acts[act.0].placed=true
    }
    //total_acts[act.0].day=Some(day as i16);
    //total_acts[act.0].hrs=Some(hour as i16);
    true
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
pub fn find_timeslot(
    act: &Activity,
    total_acts2: &[Activity],
    tat: &[model::teacher::TeacherAvailableForTimetables],
    timetables: &[NewClassTimetable],
    cat: &[ClassAvailable],
    clean_tat: &[model::teacher::TeacherAvailableForTimetables],
    mut max_day_hour: i32,
    _for_conflict: bool
)
    -> Option<Vec<(i32, usize)>> {
    use rand::thread_rng;
    let mut days = vec![1,2,3,4,5,6,7];
    days.shuffle(&mut thread_rng());
    let mut slots: Vec<(i32, usize)> = vec![];
    for day in days{
        for hour in 0..tat[0].hours.len(){
            if classes_available(act, hour, day, cat){
                if act.teachers.iter().all(|teacher| teacher_available(act, teacher, hour, day, max_day_hour, total_acts2, tat, timetables)){
                    slots.push((day, hour));
                    return Some(slots);
                }
            }
        }
    }
    None
}

fn teacher_available(
    act: &Activity,
    teacher: &i32,
    hour: usize,
    day: i32,
    max_day_hour: i32,
    total_acts2: &[Activity],
    tat: &[model::teacher::TeacherAvailableForTimetables],
    timetables: &[NewClassTimetable]) -> bool
{
    let teacher_available_day: (usize, &model::teacher::TeacherAvailableForTimetables) = tat.into_iter()
        .enumerate()
        .find(|t| t.1.user_id == *teacher && t.1.day == day).unwrap();
    let teacher_acts: Vec<Activity> = total_acts2.iter().cloned()
        .filter(|a| a.teachers.iter().any(|t| t == teacher) && a.classes.iter().any(|ac| act.classes.iter().any(|ac2| ac == ac2))).collect();
    let same_day_acts: Vec<NewClassTimetable> = timetables.iter().cloned()
        .filter(|t| t.day_id.unwrap() == day
            && teacher_acts.iter().cloned()
            .any(|a| a.id == t.activities.unwrap())).collect();
    if act.hour as usize + same_day_acts.len() > max_day_hour as usize {
        return false
    }
    if hour + act.clone().hour as usize <= teacher_available_day.1.hours.len() {
        let available = (hour..hour + act.hour as usize)
            .all(|h| teacher_available_day.1.hours[h]);
        if available{
            let _other_same_subject: Vec<NewClassTimetable> = timetables.to_owned().into_iter()
                .filter(|t| t.day_id.unwrap() == day
                    && teacher_acts.iter().cloned()
                    .any(|a| a.id == t.activities.unwrap() && a.subject == act.subject)).collect();
            if _other_same_subject.is_empty() {
                return true
            }
            else {
                let hours = _other_same_subject.iter().cloned()
                    .find(|t| t.hour.unwrap() == (hour - 1) as i16 || t.hour.unwrap() == hour as i16 + act.hour);
                if hours.is_some() {
                    return true
                }
            }
        }
    }
    false
}

fn classes_available(
    act: &Activity,
    hour: usize,
    day: i32,
    cat: &[ClassAvailable]) -> bool {
    let classes_availables: Vec<(usize, ClassAvailable)> = cat.iter().cloned()
        .enumerate()
        .filter(|c| act.classes.iter().any(|cc| cc == &c.1.class_id) && c.1.day == day).collect();
    if hour + act.clone().hour as usize <= classes_availables[0].1.hours.len() {
        //Look for activity hours. If same place/places is/are available for teacher and class
        return (hour..hour + act.hour as usize)
            .all(|h|classes_availables.iter().all(|ca| ca.1.hours[h]))
    }
    false
}

