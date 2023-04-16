use shared::models::teacher::{AddTeacher, Teacher};
use shared::DownMsg;
use shared::msgs::teachers::TeacherDownMsgs;
use sqlx::Row;

use super::auth::POSTGRES;
use moon::tokio_stream::StreamExt;

pub async fn add_teacher(id: i32, form: AddTeacher) -> DownMsg {
    let db = POSTGRES.write().await;
    let mut row = sqlx::query(
        r#"insert into users(first_name, last_name, short_name) 
                        values($1, $2, $3) 
                        returning id, first_name, last_name, short_name"#,
    )
    .bind(&form.first_name)
    .bind(&form.last_name)
    .bind(&form.short_name)
    .fetch(&*db);
    if let Some(teacher) = row.try_next().await.unwrap() {
        let user_id: i32 = teacher.try_get("id").unwrap();
        let mut row2 = sqlx::query(
            r#"insert into school_users(user_id, school_id, role) 
                        values($1, $2, $3) returning user_id"#,
        )
        .bind(user_id)
        .bind(&id)
        .bind(&4)
        .fetch(&*db);
        if let Some(_) = row2.try_next().await.unwrap() {
            let t = Teacher {
                id: teacher.try_get("id").unwrap(),
                first_name: teacher.try_get("first_name").unwrap(),
                last_name: teacher.try_get("last_name").unwrap(),
                short_name: teacher.try_get("short_name").unwrap(),
            };
            let t_dmsg = TeacherDownMsgs::AddedTeacher(t);
            return DownMsg::Teachers(t_dmsg);
        }
        return DownMsg::Teachers(TeacherDownMsgs::AddTeacherError(
            "Add teacher error on creating school_users table".to_string(),
        ));
    }
    return DownMsg::Teachers(TeacherDownMsgs::AddTeacherError("Teacher add failed".to_string()));
}

pub async fn get_teachers(id: i32) -> shared::DownMsg {
    //use moon::tokio_stream::StreamExt;
    let mut teachers_query = sqlx::query(
        r#"select id, first_name, last_name, short_name from users 
        inner join school_users on user_id = id where school_users.school_id = $1"#,
    )
    .bind(&id)
    .fetch(&*POSTGRES.write().await);
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
    shared::DownMsg::Teachers(TeacherDownMsgs::GetTeachers(teachers))
}

pub async fn del_teacher(id: i32, school_id: i32) -> shared::DownMsg {
    //use moon::tokio_stream::StreamExt;
    let db = POSTGRES.write().await;
    let mut teacher_query = sqlx::query(
        r#"delete from school_users where school_id = $1 and user_id = $2 returning user_id"#,
    )
    .bind(school_id)
    .bind(&id)
    .fetch(&*db);
    if let Some(teacher) = teacher_query.try_next().await.unwrap() {
        let mut del_user = sqlx::query(
            r#"delete from users where id = $1 and is_active = false returning id"#,
        )
        .bind(school_id)
        .bind(&id)
        .fetch(&*db);
    }
    shared::DownMsg::Teachers(TeacherDownMsgs::DeletedTeacher(id))
}
