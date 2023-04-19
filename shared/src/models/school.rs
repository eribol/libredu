use moonlight::*;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[serde(crate = "serde")]
pub struct FullSchool {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    pub manager: i32,
    #[validate(phone)]
    pub phone: String,
}

impl FullSchool{
    pub fn is_valid(&self)-> Result<(), ValidationErrors>{
        self.validate()
    }
    pub fn has_error(&self, field: &'static str)->bool{
        ValidationErrors::has_error(&self.is_valid(), field)
    }
}
