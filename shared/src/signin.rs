use moonlight::{Deserialize, Serialize, *};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(crate = "serde")]
pub struct SigninForm {
    #[validate(length(min = 2, max = 100))]
    pub first_name: String,
    #[validate(length(min = 2, max = 100))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 6))]
    pub short_name: String,
    #[validate(length(min = 6, max = 25))]
    pub password: String,
    #[validate(must_match = "password")]
    pub password2: String,
}

impl SigninForm{
    pub fn is_valid(&self)-> Result<(), ValidationErrors>{
        self.validate()
    }
}
