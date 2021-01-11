use serde::*;
use chrono::NaiveTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassGroups{
    pub id: i32,
    pub name: String,
    pub hour: i32,
    pub school: i32
}

#[derive(Debug, Clone,Deserialize, Serialize)]
pub struct Schedule{
    pub(crate) group_id: i32,
    hour: i32,
    start_time: NaiveTime,
    end_time: NaiveTime
}