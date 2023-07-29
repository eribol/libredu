
use shared::models::lectures::{AddLecture, Lecture};
use shared::DownMsg;
use shared::msgs::lectures::LecturesDownMsg;
use sqlx::Row;

use super::auth::POSTGRES;
use moon::tokio_stream::StreamExt;

pub async fn add_lecture(id: i32, lecture_form: AddLecture) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(
        r#"insert into subjects(kademe, name, school, short_name) 
        values($1, $2, $3, $4) returning id, kademe, name, short_name"#,
    )
        .bind(&lecture_form.kademe)
        .bind(&lecture_form.name)
        .bind(id)
        .bind(&lecture_form.short_name)
        .fetch(&*db);
        if let Some(lec) = row.try_next().await.unwrap() {
            let l = Lecture {
                id: lec.try_get("id").unwrap(),
                kademe: lec.try_get("kademe").unwrap(),
                name: lec.try_get("name").unwrap(),
                short_name: lec.try_get("short_name").unwrap(),
            };
            let l_msg = LecturesDownMsg::AddedLecture(l);
                DownMsg::Lectures(l_msg)
            } 
    else {
        let l_msg = LecturesDownMsg::AddLectureError("Lecture Add error".to_string());
        DownMsg::Lectures(l_msg)
    }
}

pub async fn update_lecture(school_id: i32,lecture_form: Lecture) -> DownMsg {
    let db = POSTGRES.read().await;
    let _row = sqlx::query(
        r#"update subjects set kademe = $1, name = $2, short_name = $3
        where id = $4 and school = $5
        returning id, kademe, name, short_name"#,
    )
    .bind(&lecture_form.kademe)
    .bind(&lecture_form.name)
    .bind(&lecture_form.short_name)
    .bind(lecture_form.id)
    .bind(school_id)
    .execute(&*db).await;
    let l_msg = LecturesDownMsg::AddedLecture(lecture_form);
    DownMsg::Lectures(l_msg)
}

pub async fn get_lectures(id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut lectures: Vec<Lecture> = vec![];
    let mut row = sqlx::query(
        r#"select id, kademe, name, short_name from subjects
        where school = $1"#,
    )
        .bind(id)
        .fetch(&*db);
    while let Some(lec) = row.try_next().await.unwrap() {
        lectures.push(Lecture {
            id: lec.try_get("id").unwrap(),
            kademe: lec.try_get("kademe").unwrap(),
            name: lec.try_get("name").unwrap(),
            short_name: lec.try_get("short_name").unwrap(),
        })
    }
    DownMsg::Lectures(LecturesDownMsg::GetLectures(lectures))
}

pub async fn del_lecture(id: i32, school_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(
        r#"delete from subjects where id = $1 and school = $2 returning id"#,
    )
        .bind(id)
        .bind(school_id)
        .fetch(&*db);
    if let Some(_) = row.try_next().await.unwrap() {
        return DownMsg::Lectures(LecturesDownMsg::DeletedLecture(id))
    }
    DownMsg::Lectures(LecturesDownMsg::DelLectureError("No subject".to_string()))
}
