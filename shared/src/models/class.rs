use moonlight::*;
pub type UserId = EntityId;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AddClass {
    pub kademe: String,
    pub sube: String,
    pub group_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Class {
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub group_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct ClassLimitation {
    pub class_id: i32,
    pub day: i32,
    pub hours: Vec<bool>,
}
