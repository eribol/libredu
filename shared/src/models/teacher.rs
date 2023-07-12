use moonlight::*;
use validator::{Validate, ValidationErrors};
pub type UserId = EntityId;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(crate = "serde")]
pub struct AddTeacher {
    #[validate(length(min = 2, max = 100))]
    pub first_name: String,
    #[validate(length(min = 2, max = 100))]
    pub last_name: String,
    #[validate(length(min = 2, max = 6))]
    pub short_name: String,
}
impl AddTeacher{
    pub fn is_valid(&self)-> Result<(), ValidationErrors>{
        self.validate()
    }
    pub fn has_error(&self, field: &'static str)->bool{
        ValidationErrors::has_error(&self.is_valid(), field)
    }
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

