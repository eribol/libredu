use moonlight::*;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AddTimetable {
    pub name: String,
    pub hour: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Timetable {
    pub id: i32,
    pub name: String,
    pub hour: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Activity {
    pub id: i32,
    pub subject: i32,
    pub hour: i16,
    pub classes: Vec<i32>,
    pub teachers: Vec<i32>,
    pub blocks: Option<String>,
    pub partner_activity: Option<i32>,
}
