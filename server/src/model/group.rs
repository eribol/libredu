use chrono::NaiveTime;
use serde::*;
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClassGroups{
    pub id: i32,
    pub name: String,
    pub hour: i32,
    pub school: i32
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Schedules{
    group_id: i32,
    hour: i32,
    start_time: NaiveTime,
    end_time: NaiveTime
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct AddGroup{
    name: String,
    hour: i32
}