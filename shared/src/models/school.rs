use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct FullSchool {
    //pub id: i32,
    pub name: String,
    pub manager: i32,
    pub phone: String,
}
