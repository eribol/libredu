use moonlight::*;
use validator::{Validate, ValidationErrors};
pub type UserId = EntityId;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[serde(crate = "serde")]
pub struct ResetForm {
    #[validate(length(min = 2, max = 50))]
    pub token: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 25))]
    pub password: String,
    #[validate(must_match = "password")]
    pub password2: String,
}
impl ResetForm{
    pub fn is_valid(&self)-> Result<(), ValidationErrors>{
        self.validate()
    }
    pub fn has_error(&self, field: &'static str)->bool{
        ValidationErrors::has_error(&self.is_valid(), field)
    }
}
