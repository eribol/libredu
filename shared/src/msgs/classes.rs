use crate::models::class::*;
use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum ClassUpMsgs {
    GetClasses,
    AddClass(AddClass),
    DelClass(i32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum ClassDownMsgs {
    GetClasses(Vec<Class>),
    GetClassesError(String),
    AddedClass(Class),
    AddClassError(String),
    DeletedClass(i32),
    DeleteClassError(String),
}
