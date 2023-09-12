use moon::{*, tokio_stream::StreamExt};
use shared::{models::{school::FullSchool, timetables::{AddTimetable, Timetable, Activity}, class::{Class, ClassLimitation}, teacher::{Teacher, TeacherLimitation}}, DownMsg, msgs::{admin::{AdminDownMsgs, SchoolManager, AdminSchool}, messages::{Message, NewMessage}}, School};
use sqlx::{FromRow, Row};

use super::{sql::POSTGRES, school::get_school};

pub async fn get_classes(group_id: i32)-> AdminDownMsgs{
    let db = POSTGRES.read().await;
    let mut classes =
        sqlx::query(r#"select * from classes where group_id = $1"#)
            .bind(group_id)
            .fetch(&*db);
    let mut clss = vec![];
    while let Some(class) = classes.try_next().await.unwrap(){
        let c = Class{
            id: class.try_get("id").unwrap(),
            kademe: class.try_get("kademe").unwrap(),
            sube: class.try_get("sube").unwrap(),
            group_id
        };
        clss.push(c);
    }
    AdminDownMsgs::GetClasses(clss)
}

pub async fn get_groups(school_id: i32)-> AdminDownMsgs{
    let db = POSTGRES.read().await;
    let mut timetables =
        sqlx::query(r#"select * from class_groups where school_id = $1"#)
            .bind(school_id)
            .fetch(&*db);
    let mut tts = vec![];
    while let Some(timetable) = timetables.try_next().await.unwrap(){
        let tt = Timetable{
            id: timetable.try_get("id").unwrap(),
            name: timetable.try_get("name").unwrap(),
            hour: timetable.try_get("hour").unwrap(),
        };
        tts.push(tt);
    }
    AdminDownMsgs::GetTimetables(tts)
}

pub async fn get_teachers(id: i32) -> AdminDownMsgs {
    //use moon::tokio_stream::StreamExt;
    let mut teachers_query = sqlx::query(
        r#"select id, first_name, last_name, short_name from users 
        inner join school_users on user_id = id where school_users.school_id = $1"#,
    )
    .bind(id)
    .fetch(&*POSTGRES.read().await);
    let mut teachers = vec![];
    while let Some(teacher) = teachers_query.try_next().await.unwrap() {
        let t = Teacher {
            id: teacher.try_get("id").unwrap(),
            first_name: teacher.try_get("first_name").unwrap(),
            last_name: teacher.try_get("last_name").unwrap(),
            short_name: teacher.try_get("short_name").unwrap(),
        };
        teachers.push(t);
    }
    AdminDownMsgs::GetTeachers(teachers)
}

pub async fn get_schools() -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    let mut schools =
        sqlx::query(r#"select school.id as school_id, school.name, users.id as user_id, users.first_name, users.last_name, users.last_login from school 
            inner join users on users.id = school.manager
            order by users.last_login desc limit 10"#)
            .fetch(&*db);
    let mut schs = vec![];
    while let Some(row) = schools.try_next().await.unwrap() {
        let s = School{
            id: row.try_get("school_id").unwrap(),
            name: row.try_get("name").unwrap(),
        };
        let u = SchoolManager{
            id: row.try_get("user_id").unwrap(),
            first_name: row.try_get("first_name").unwrap(),
            last_name: row.try_get("last_name").unwrap(),
            last_login: row.try_get("last_login").unwrap_or(Utc::now().naive_utc()),
        };
        let s_admin = AdminSchool{
            school: s,
            principle: u,
        };
        schs.push(s_admin);
    }
    AdminDownMsgs::LastSchools(schs)
}
pub async fn get_timetables(school_id: i32)-> AdminDownMsgs{
    let db = POSTGRES.read().await;
    let mut groups_query = sqlx::query(
        r#"select class_groups.id, class_groups.name, class_groups.hour from class_groups
        inner join school on school.id = class_groups.school where school.id = $1"#,
    )
    .bind(school_id)
    .fetch(&*db);
    let mut groups = vec![];
    while let Some(g) = groups_query.try_next().await.unwrap() {
        let group = shared::models::timetables::Timetable {
            id: g.try_get("id").unwrap(),
            name: g.try_get("name").unwrap(),
            hour: g.try_get("hour").unwrap(),
        };
        groups.push(group)
    }
    AdminDownMsgs::GetTimetables(groups)
}

pub async fn get_activities(group_id: i32) -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    let mut groups_query = 
    sqlx::query(r#"select activities.id, activities.subject, activities.hour, activities.teachers, activities.partner_activity, activities.classes
        from activities inner join school_acts on school_acts.act_id = activities.id where school_acts.group_id = $1"#)
    .bind(&group_id)
    .fetch(&*db);
    let mut activities = vec![];
    while let Some(g) = groups_query.try_next().await.unwrap() {
        let act = Activity {
            id: g.try_get("id").unwrap(),
            subject: g.try_get("subject").unwrap(),
            hour: g.try_get("hour").unwrap(),
            classes: g.try_get("classes").unwrap(),
            teachers: g.try_get("teachers").unwrap(),
            blocks: None,
            partner_activity: None
        };
        activities.push(act)
    }
    AdminDownMsgs::GetActivities(activities)
}

pub async fn get_classes_limitations(group_id: i32) -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(r#"select * from class_available inner join classes on class_available.class_id = classes.id
                        where classes.group_id = $1"#)
        .bind(&group_id)
        .fetch(&*db);
    let mut limitations = vec![];
    while let Some(class) = row.try_next().await.unwrap() {
        let c = ClassLimitation {
            class_id: class.try_get("class_id").unwrap(),
            day: class.try_get("day").unwrap(),
            hours: class.try_get("hours").unwrap(),
        };
        limitations.push(c);
    }
    AdminDownMsgs::GetClassesLimitations(limitations)
}

pub async fn get_teachers_limitations(group_id: i32) -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(r#"select * from teacher_available
                        where group_id = $1"#)
        .bind(&group_id)
        .fetch(&*db);
    let mut limitations = vec![];
    while let Some(class) = row.try_next().await.unwrap() {
        let day: i32 = class.try_get("day").unwrap();
        let c = TeacherLimitation{
            user_id: class.try_get("user_id").unwrap(),
            school_id: class.try_get("school_id").unwrap(),
            group_id: class.try_get("group_id").unwrap(),
            day: day as i16,
            hours: class.try_get("hours").unwrap(),
        };
        limitations.push(c);
    }
    AdminDownMsgs::GetTeachersLimitations(limitations)
}

pub async fn update_class_limitations(mut form: Vec<ClassLimitation>) -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    form.sort_by(|a,b| a.day.cmp(&b.day));
    let class_id = form[0].class_id;
    if form.len() != 7{
        let c_msg = AdminDownMsgs::UpdateClassLimitationsError("No valid form".to_string());
        return c_msg
    }
    if !form.iter().enumerate().all(|l| l.1.class_id == class_id && l.0+1==l.1.day as usize){
        let c_msg = AdminDownMsgs::UpdateClassLimitationsError("No valid form".to_string());
        return c_msg
    }
    let mut lims: Vec<ClassLimitation> = Vec::new();
    for l in form{
        let mut insert = sqlx::query(
            r#"insert into class_available(class_id, day, hours) values($1, $2, $3) 
                on conflict(class_id, day) where class_id = $1 and  day = $2 do update set hours = $3
                returning class_id, day, hours"#)
            .bind(&class_id)
            .bind(&l.day)
            .bind(&l.hours)
            .fetch(&*db);
        while let Some(row) = insert.try_next().await.unwrap(){
            let new_lim = ClassLimitation{
                class_id: row.try_get("class_id").unwrap(),
                day: row.try_get("day").unwrap(),
                hours: row.try_get("hours").unwrap()
            };
            lims.push(new_lim);
        }

    }
    let c_msg = AdminDownMsgs::UpdatedClassLimitations;
    c_msg
}

pub async fn update_teacher_limitations(mut form: Vec<TeacherLimitation>) -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    form.sort_by(|a,b| a.day.cmp(&b.day));
    let teacher_id = form[0].user_id;
    let school_id = form[0].school_id;
    let group_id = form[0].group_id;
    if form.len() != 7{
        let c_msg = AdminDownMsgs::UpdateTeacherLimitationsError("No valid form".to_string());
        return c_msg
    }
    let user_id = form[0].user_id;
    if !form.iter().enumerate().all(|l| l.1.user_id == user_id && l.0+1==l.1.day as usize){
        let c_msg = AdminDownMsgs::UpdateTeacherLimitationsError("No valid form".to_string());
        return c_msg
    }
    let mut lims: Vec<TeacherLimitation> = Vec::new();
    for l in form{
        let mut insert = sqlx::query(
            r#"insert into teacher_available(user_id, school_id, group_id, day, hours) values($1, $2, $3, $4, $5) 
                on conflict(user_id, group_id, day, school_id) where user_id = $1 and  day = $4 and group_id = $3 do update set hours = $5
                returning user_id, school_id, group_id, day, hours"#)
            .bind(&user_id)
            .bind(&school_id)
            .bind(&group_id)
            .bind(&l.day)
            .bind(&l.hours)
            .fetch(&*db);
        while let Some(row) = insert.try_next().await.unwrap(){
            let new_lim = TeacherLimitation{
                user_id: row.try_get("user_id").unwrap(),
                group_id,
                school_id,
                day: row.try_get("day").unwrap(),
                hours: row.try_get("hours").unwrap()
            };
            lims.push(new_lim);
        }

    }
    let teacher_msg = AdminDownMsgs::UpdatedTeacherLimitations;
    teacher_msg
}

pub async fn del_act(act_id: i32) -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    let _ = sqlx::query(
        r#"delete from activities where id = $1"#,
    )
    .bind(act_id)
    .execute(&*db).await;
    AdminDownMsgs::DeletedAct
}

pub async fn get_school_messages(id: i32)-> AdminDownMsgs{
    let db = POSTGRES.read().await;
    let mut query = sqlx::query(r#"select * from help_messages where school_id = $1"#,
    )
    .bind(&id)
    .fetch(&*db);
    let mut msgs = vec![];
    while let Some(row) = query.try_next().await.unwrap(){
        let m = Message{
            id: row.try_get("id").unwrap(),
            sender_id: row.try_get("sender_id").unwrap(),
            school_id: row.try_get("school_id").unwrap(),
            school_name: row.try_get("school_name").unwrap(),
            body: row.try_get("body").unwrap(),
            send_time: row.try_get("send_time").unwrap(),
            to_school: row.try_get("to_school").unwrap(),
            read: row.try_get("read").unwrap()
        };
        msgs.push(m);
    }
    AdminDownMsgs::GetSchoolMessages(msgs)
}

pub async fn new_message(form: NewMessage)-> AdminDownMsgs{
    let db = POSTGRES.read().await;
    let mut a = sqlx::query(r#"insert into help_messages(sender_id, school_name,school_id, body, send_time, to_school, read) 
        values($1, $2, $3, $4, $5, $6, $7) returning sender_id, school_name,school_id, body, send_time, to_school, read, id"#,
    )
    .bind(&1)
    .bind(&form.school_name)
    .bind(&form.school_id)
    .bind(&form.body)
    .bind(&form.send_time)
    .bind(&form.to_school)
    .bind(&form.read)
    .fetch(&*db);
    if let Some(row) = a.try_next().await.unwrap(){
        println!("messages");
        let m = Message{
            id: row.try_get("id").unwrap(),
            sender_id: row.try_get("sender_id").unwrap(),
            school_id: row.try_get("school_id").unwrap(),
            school_name: row.try_get("school_name").unwrap(),
            body: row.try_get("body").unwrap(),
            send_time: row.try_get("send_time").unwrap(),
            to_school: row.try_get("to_school").unwrap(),
            read: row.try_get("read").unwrap(),
        };
        return AdminDownMsgs::GetMessage(m);
    }
    return AdminDownMsgs::Empty
}