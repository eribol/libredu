use rand::seq::SliceRandom;
use seed::{*};
use crate::model;
use crate::page::school::group::timetable::{ClassAvailable, Activity, NewClassTimetable};
//use async_std::prelude::*;
//use async_std::task;

pub(crate) fn generate(
    max_day_hour: i32,
    max_depth: usize,
    depth2: usize,
    tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
    cat: &mut Vec<ClassAvailable>,
    total_acts: &Vec<Activity>,
    timetables: &mut Vec<NewClassTimetable>,
    clean_tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
    error: &mut String
)
    -> bool {
    use rand::thread_rng;
    let mut acts: Vec<Activity> = total_acts.iter().cloned()
        .filter(|a| a.teacher.is_some() && !timetables.iter().cloned()
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
    if acts.len() == 0 {
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
            return true
        },
        None => {
            let timetables_backup = timetables.clone();
            let tat_backup = tat.clone();
            let cat_backup = cat.clone();
            let rec_result = recursive_put(&act2, total_acts, timetables, clean_tat, tat, cat, max_day_hour, 0, max_depth, &mut vec![],&tat_backup, &cat_backup, &timetables_backup);
            if rec_result {
                return true;
            }
            else {
                *timetables = timetables_backup;
                *tat = tat_backup;
                *cat = cat_backup;
                //log!("oh", &act2.teacher, &act2.classes);
                let mut conflict_acts = find_conflict_activity(&act2, &total_acts, &timetables, clean_tat, &tat, &cat, max_day_hour, max_depth, &mut vec![]);
                if conflict_acts.len() == 0 {
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
                    match available2 {
                        Some(slots2) => {
                            put_activity(a, total_acts, tat, timetables, cat, slots2[0].0, slots2[0].1, clean_tat);
                            //return true
                        },
                        None => {
                            //return false
                        }
                    }
                }
                return true
            }
        }
    }
}
//Add all timetables array to database

pub(crate) fn recursive_put(
    act: &Activity,
    _acts: &Vec<Activity>,
    timetables: &mut Vec<NewClassTimetable>,
    clean_tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
    tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
    cat: &mut Vec<ClassAvailable>,
    max_day_hour: i32,
    depth: usize,
    //depth2: usize,
    max_depth: usize,
    ignore_list: &mut Vec<Activity>,
    tat2: &Vec<model::teacher::TeacherAvailableForTimetables>,
    cat2: &Vec<ClassAvailable>,
    timetables2: &Vec<NewClassTimetable>
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
            *tat = tat2.clone();
            *cat = cat2.clone();
            *timetables = timetables2.clone();
            //ignore_list.retain(|a3| !c_act.iter().any(|a4| a4.id == a3.id));
            okey2 = false;
            //break;
        }

    }
    return okey2;
}


fn delete_activity(_acts: &Vec<Activity>,act: &Activity, tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
                   timetables: &mut Vec<NewClassTimetable>, cat: &mut Vec<ClassAvailable>, _a: bool)-> bool
{
    let tt: Vec<(usize, NewClassTimetable)> = timetables.iter().cloned()
        .enumerate()
        .filter(|t| t.1.activities.unwrap() == act.id).collect();

    for t in &tt{
        let find_tat = tat.iter().cloned()
            .enumerate()
            .find(|ta| ta.1.user_id == act.teacher.unwrap() && ta.1.day == t.1.day_id.unwrap()).unwrap();
        tat[find_tat.0].hours[t.1.hour.unwrap() as usize] = true;

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
    return true;
}

fn find_conflict_activity(
    act: &Activity,
    _acts: &Vec<Activity>,
    timetables: &Vec<NewClassTimetable>,
    clean_tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
    _tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
    cat: &Vec<ClassAvailable>,
    max_day_hour: i32,
    depth: usize,
    ignore_list: &mut Vec<Activity>
)->Vec<Vec<Activity>>
{
    use rand::thread_rng;
    //let class = act.class.unwrap();
    let mut total_act: Vec<Vec<Activity>> = Vec::new();

    let activities: Vec<Activity> = _acts.iter().cloned()
        .filter(|a| act.classes.iter().any(|c1| a.classes.iter().any(|c2| c1 == c2)) || a.teacher==act.teacher).collect();
    let mut teacher_availables: Vec<model::teacher::TeacherAvailableForTimetables> = clean_tat.iter().cloned()
        .filter(|t| t.user_id == act.teacher.unwrap() && t.hours.iter().any(|h| *h)).collect();
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
                        let conflict_slot: Vec<NewClassTimetable> = timetables.iter().cloned()
                            .filter(|t| t.day_id.unwrap() == teacher_available.day && t.hour.unwrap() as usize == i).collect();

                        for c in &conflict_slot{
                            let activity = activities.iter()
                                .find(|a| a.id == c.activities.unwrap() && a.id != act.id && !ignore_list.iter().any(|a2| a2.id == a.id));
                            match activity{
                                Some(a)=>{
                                    less_conflict.push(a.clone());
                                },
                                None=>{}
                            }
                        }
                    }
                    let mut copy_tat = _tat.clone();
                    let mut copy_ch = cat.clone();
                    let mut copy_tt = timetables.clone();
                    let mut acts = _acts.clone();
                    for a in &less_conflict{
                        delete_activity(&mut acts, a, &mut copy_tat, &mut copy_tt, &mut copy_ch, true);
                    }
                    let available = find_timeslot(act, &acts, &copy_tat, &copy_tt, &copy_ch, &clean_tat, max_day_hour, true);
                    match available {
                        Some(_slots) => {
                            total_act.push(less_conflict);
                        },
                        None => {}
                    }
                }
            }
        }
    }
    //total_act.shuffle(&mut thread_rng());
    total_act.sort_by(|a,b| a.len().cmp(&b.len()));
    for i in 0..total_act.len(){
        total_act[i].sort_by(|a,b| a.id.cmp(&b.id));
        total_act[i].dedup();
    }
    if total_act.len()>=depth{
        return total_act[0..depth].to_vec();
    }
    else {
        return total_act
    }
    /*if depth < 1{
        if total_act.len()>=10{
            return total_act[0..10].to_vec();
        }
        else {
            return total_act
        }
    }
    else{
        if total_act.len()>=3{
            return total_act[0..3].to_vec();
        }
        else {
            return total_act
        }
    }*/
}

pub fn put_activity(
    act: &Activity,
    _total_acts: &Vec<Activity>,
    tat: &mut Vec<model::teacher::TeacherAvailableForTimetables>,
    timetables: &mut Vec<NewClassTimetable>,
    cat: &mut Vec<ClassAvailable>,
    day:i32,
    hour: usize,
    clean_tat: &Vec<model::teacher::TeacherAvailableForTimetables>
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
        let tat_index = tat.iter().cloned()
            .enumerate()
            .find(|t| t.1.user_id == act.teacher.unwrap() && t.1.day==day).unwrap();
        tat[tat_index.0].hours[timetable]= false;
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
    return true;
}

pub fn find_timeslot(
    act: &Activity,
    total_acts2: &Vec<Activity>,
    tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
    timetables: &Vec<NewClassTimetable>,
    cat: &Vec<ClassAvailable>,
    clean_tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
    mut max_day_hour: i32,
    _for_conflict: bool
)
    -> Option<Vec<(i32, usize)>> {
    use rand::thread_rng;
    let mut teacher_availables: Vec<(usize, model::teacher::TeacherAvailableForTimetables)> = tat.iter().cloned()
        .enumerate()
        .filter(|t| t.1.user_id == act.teacher.unwrap() && t.1.hours.iter()
            .any(|h| *h)).collect();
    teacher_availables.shuffle(&mut thread_rng());
    let mut _teacher_total_day_availables: Vec<(usize, model::teacher::TeacherAvailableForTimetables)> = clean_tat.iter().cloned()
        .enumerate()
        .filter(|t| t.1.user_id == act.teacher.unwrap() && t.1.hours.iter()
            .any(|h| *h)).collect();
    //let available2 = false;
    let mut slots: Vec<(i32, usize)> = Vec::new();
    let mut day: i32;
    let mut hour: usize;
    let mut max_total_hour: usize = 0;
    let teacher_acts: Vec<Activity> = total_acts2.iter().cloned()
        .filter(|a| a.teacher == act.teacher && a.classes.iter().any(|ac| act.classes.iter().any(|ac2| ac == ac2))).collect();
    for t in &teacher_acts {
        max_total_hour += t.hour as usize;
    }
    if max_total_hour > 8 {
        max_day_hour = 4;
    }
    //teacher_availables.shuffle(&mut thread_rng());
    for teacher_available in &teacher_availables {
        //let same_subject_acts: Vec<Activity> = teacher_acts.iter().cloned()
        //    .filter(|a| a.class == act.1.class && a.subject == act.1.subject ).collect();
        let same_day_acts: Vec<NewClassTimetable> = timetables.iter().cloned()
            .filter(|t| t.day_id.unwrap() == teacher_available.1.day
                && teacher_acts.iter().cloned()
                .any(|a| a.id == t.activities.unwrap())).collect();
        //if same_subject_acts.len() < 8{
        if act.hour as usize + same_day_acts.len() > max_day_hour as usize {
            continue;
        }
        let classes_availables: Vec<(usize, ClassAvailable)> = cat.iter().cloned()
            .enumerate()
            .filter(|c| act.classes.iter().any(|cc| cc == &c.1.class_id) && c.1.day == teacher_available.1.day).collect();
        for i in 0..teacher_available.1.hours.len() {
            if i + act.clone().hour as usize <= teacher_available.1.hours.len() {
                //Look for activity hours. If same place/places is/are available for teacher and class
                let available = (i..i + act.hour as usize)
                    .all(|h| teacher_available.1.hours[h] && classes_availables.iter().all(|ca| ca.1.hours[h]));
                if available {
                    day = teacher_available.1.day;
                    hour = i;
                    let _other_same_subject: Vec<NewClassTimetable> = timetables.iter().cloned()
                        .filter(|t| t.day_id.unwrap() == day
                            && teacher_acts.iter().cloned()
                            .any(|a| a.id == t.activities.unwrap() && a.subject == act.subject)).collect();
                    if _other_same_subject.len() == 0 {
                        slots.push((day, hour));
                        return Some(slots)
                    } else {
                        //slots.push((day, hour));
                        //return Some(slots)

                        let hours = _other_same_subject.iter().cloned()
                            .find(|t| t.hour.unwrap() == (hour - 1) as i16 || t.hour.unwrap() == hour as i16 + act.hour);
                        match hours {
                            Some(_) => {
                                slots.push((day, hour));
                                return Some(slots)
                            }
                            None => {}
                        }

                    }
                }
            }
        }
        //if for_conflict && slots.len() >= 1 || slots.len()>=25{
        //    break;
        //}
    }

    return None;
}

impl Activity{
    fn timeslots(&self,
                     total_acts2: &Vec<Activity>,
                     tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
                     timetables: &Vec<NewClassTimetable>,
                     cat: &Vec<ClassAvailable>,
                     clean_tat: &Vec<model::teacher::TeacherAvailableForTimetables>,
                     mut max_day_hour: i32,
                     _for_conflict: bool) -> usize {
        {
            use rand::thread_rng;
            let mut teacher_availables: Vec<(usize, model::teacher::TeacherAvailableForTimetables)> = tat.iter().cloned()
                .enumerate()
                .filter(|t| t.1.user_id == self.teacher.unwrap() && t.1.hours.iter()
                    .any(|h| *h)).collect();
            teacher_availables.shuffle(&mut thread_rng());
            let mut _teacher_total_day_availables: Vec<(usize, model::teacher::TeacherAvailableForTimetables)> = clean_tat.iter().cloned()
                .enumerate()
                .filter(|t| t.1.user_id == self.teacher.unwrap() && t.1.hours.iter()
                    .any(|h| *h)).collect();
            //let available2 = false;
            let mut slots: Vec<(i32, usize)> = Vec::new();
            let mut day: i32;
            let mut hour: usize;
            let mut max_total_hour: usize = 0;
            let teacher_acts: Vec<Activity> = total_acts2.iter().cloned()
                .filter(|a| a.teacher == self.teacher && a.classes.iter().any(|ac| self.classes.iter().any(|ac2| ac == ac2))).collect();
            for t in &teacher_acts {
                max_total_hour += t.hour as usize;
            }
            if max_total_hour > 8 {
                max_day_hour = 4;
            }
            //teacher_availables.shuffle(&mut thread_rng());
            for teacher_available in &teacher_availables {
                //let same_subject_acts: Vec<Activity> = teacher_acts.iter().cloned()
                //    .filter(|a| a.class == act.1.class && a.subject == act.1.subject ).collect();
                let same_day_acts: Vec<NewClassTimetable> = timetables.iter().cloned()
                    .filter(|t| t.day_id.unwrap() == teacher_available.1.day
                        && teacher_acts.iter().cloned()
                        .any(|a| a.id == t.activities.unwrap())).collect();
                //if same_subject_acts.len() < 8{
                if self.hour as usize + same_day_acts.len() > max_day_hour as usize {
                    continue;
                }
                let classes_availables: Vec<(usize, ClassAvailable)> = cat.iter().cloned()
                    .enumerate()
                    .filter(|c| self.classes.iter().any(|cc| cc == &c.1.class_id) && c.1.day == teacher_available.1.day).collect();
                for i in 0..teacher_available.1.hours.len() {
                    if i + self.clone().hour as usize <= teacher_available.1.hours.len() {
                        //Look for activity hours. If same place/places is/are available for teacher and class
                        let available = (i..i + self.hour as usize)
                            .all(|h| teacher_available.1.hours[h] && classes_availables.iter().all(|ca| ca.1.hours[h]));
                        if available {
                            day = teacher_available.1.day;
                            hour = i;
                            let _other_same_subject: Vec<NewClassTimetable> = timetables.iter().cloned()
                                .filter(|t| t.day_id.unwrap() == day
                                    && teacher_acts.iter().cloned()
                                    .any(|a| a.id == t.activities.unwrap() && a.subject == self.subject)).collect();
                            if _other_same_subject.len() == 0 {
                                slots.push((day, hour));
                                //return Some(slots)
                            } else {
                                let hours = _other_same_subject.iter().cloned()
                                    .find(|t| t.hour.unwrap() == (hour - 1) as i16 || t.hour.unwrap() == hour as i16 + self.hour);
                                match hours {
                                    Some(_) => {
                                        slots.push((day, hour));
                                        //return Some(slots)
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                }
                //if for_conflict && slots.len() >= 1 || slots.len()>=25{
                //    break;
                //}
            }

            return slots.len();
        }
    }
}