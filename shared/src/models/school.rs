use moonlight::*;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[serde(crate = "serde")]
pub struct FullSchool {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    pub manager: i32,
    #[validate(phone)]
    pub phone: String,
}
