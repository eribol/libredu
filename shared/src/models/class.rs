use moonlight::*;
use validator::{Validate, ValidationErrors};
pub type UserId = EntityId;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(crate = "serde")]
pub struct AddClass {
    #[validate(length(min = 1, max = 5))]
    pub grade: String,
    #[validate(length(min = 1, max = 5))]
    pub branch: String,
    pub group_id: i32,
}
impl AddClass {
    pub fn is_valid(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
    pub fn has_error(&self, field: &'static str) -> bool {
        ValidationErrors::has_error(&self.is_valid(), field)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Class {
    pub id: i32,
    pub grade: String,
    pub branch: String,
    pub group_id: i32,
    pub short_name: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct ClassLimitation {
    pub class_id: i32,
    // pub day: i32,
    // pub hours: Vec<bool>,
    pub limitations: Vec<Vec<bool>>,
}

impl Class {
    pub fn label(&self) -> String {
        let label = self.grade.clone() + &self.branch;
        label
    }
}
