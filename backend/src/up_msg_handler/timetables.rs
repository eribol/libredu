use crate::connection::sql::POSTGRES;
use moon::tokio_stream::StreamExt;
use shared::models::timetables::*;
use shared::{DownMsg, msgs::timetables::*};
use sqlx::Row;

pub async fn add_timetable(form: AddTimetable, school_id: i32) -> DownMsg {
    let db = POSTGRES.write().await;            
    let mut timetable_query = sqlx::query(r#"insert into class_groups(school, name, hour) values($1, $2, $3) returning id, name, hour"#)
        .bind(school_id)
        .bind(&form.name)
        .bind(form.hour)
        .fetch(&*db);
    if let Some(timetable) = timetable_query.try_next().await.unwrap() {
        let t = Timetable {
            id: timetable.try_get("id").unwrap(),
            name: timetable.try_get("name").unwrap(),
            hour: timetable.try_get("hour").unwrap(),
        };
        let tt_msg = TimetablesDownMsgs::AddedTimetable(t);
            return DownMsg::Timetables(tt_msg);
    }
    DownMsg::Timetables(TimetablesDownMsgs::AddTimetableError("Add timetable sql error".to_string()))
}

pub async fn get_class_groups(auth: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut groups_query = sqlx::query(
        r#"select class_groups.id, class_groups.name, class_groups.hour from class_groups
        inner join school on school.id = class_groups.school where school.id = $1"#,
    )
        .bind(auth)
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
    DownMsg::Timetables(TimetablesDownMsgs::GetTimetables(groups))
}

pub async fn del_timetable(id: i32, group_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    del_tt_acts(group_id).await;
    let mut groups_query = sqlx::query(
        r#"delete from class_groups where school = $1 and id = $2 returning id"#,
    )
        .bind(id)
        .bind(group_id)
        .fetch(&*db);
    if let Some(_g) = groups_query.try_next().await.unwrap() {
        return DownMsg::Timetables(TimetablesDownMsgs::DeletedTimetable(group_id))
    }
    DownMsg::Timetables(TimetablesDownMsgs::DeleteTimetableError("No timetable found".to_string()))
}

pub async fn get_schedules(group_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut groups_query = sqlx::query(
        r#"select * from group_schedules where group_id = $1"#,
    )
        .bind(group_id)
        .fetch(&*db);
    if let Some(_g) = groups_query.try_next().await.unwrap() {
        let schedules = TimetableSchedules{
            group_id,
            starts: _g.try_get("starts").unwrap(),
            ends: _g.try_get("ends").unwrap(),
        };
        return DownMsg::Timetables(TimetablesDownMsgs::GetSchedules(schedules))
    }
    DownMsg::Timetables(
        TimetablesDownMsgs::GetSchedules(
            TimetableSchedules{group_id, starts: vec![], ends: vec![]}
        )
    )
}
pub async fn update_schedules(group_id: i32, schedules: TimetableSchedules) -> DownMsg {
    let db = POSTGRES.read().await;
    let starts = &schedules.starts;
    let ends = &schedules.ends;
    let mut groups_query = sqlx::query(
        r#"insert into group_schedules(starts, ends, group_id) values($1, $2, $3) 
        on conflict (group_id) do update set starts=$1, ends = $2"#,
    )
        .bind(&starts)
        .bind(&ends)
        .bind(group_id)
        .fetch(&*db);
    groups_query.try_next().await.unwrap();
    DownMsg::Timetables(
        TimetablesDownMsgs::GetSchedules(
            schedules.clone()
        )
    )
}

async fn del_tt_acts(group_id: i32){
    let db = POSTGRES.read().await;
    let _groups_query = sqlx::query(
        r#"delete from activities using school_acts where school_acts.group_id = $1 and school_acts.act_id = activities.id"#,
    )
    .bind(group_id)
    .execute(&*db).await;
}