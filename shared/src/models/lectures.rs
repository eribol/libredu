use moonlight::*;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(crate = "serde")]
pub struct AddLecture {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 2, max = 8))]
    pub short_name: String,
}
impl AddLecture{
    pub fn is_valid(&self)-> Result<(), ValidationErrors>{
        self.validate()
    }
    pub fn has_error(&self, field: &'static str)->bool{
        ValidationErrors::has_error(&self.is_valid(), field)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Lecture {
    pub id: i32,
    pub name: String,
    pub short_name: String,
}
