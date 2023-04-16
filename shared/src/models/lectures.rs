use moonlight::*;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AddLecture {
    pub name: String,
    pub kademe: String,
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Lecture {
    pub id: i32,
    pub name: String,
    pub kademe: String,
    pub short_name: String,
}
