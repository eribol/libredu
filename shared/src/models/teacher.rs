use moonlight::*;
pub type UserId = EntityId;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AddTeacher {
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Teacher {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct TeacherLimitation {
    pub user_id: i32,
    pub school_id: i32,
    pub group_id: i32,
    pub day: i16,
    pub hours: Vec<bool>,
}
