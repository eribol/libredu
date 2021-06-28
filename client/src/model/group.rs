use serde::*;
use chrono::NaiveTime;
use crate::model::class;
use crate::model::class::ClassContext;
use seed::Url;

#[derive(Clone,Deserialize, Serialize)]
pub struct Schedule{
    pub(crate) group_id: i32,
    hour: i32,
    start_time: NaiveTime,
    end_time: NaiveTime
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ClassGroups{
    pub id: i32,
    pub name: String,
    pub hour: i32,
    pub school: i32
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupContext{
    pub group: ClassGroups ,
    pub classes: Option<Vec<class::ClassContext>>,
}

impl GroupContext{
    pub fn get_mut_classes(&mut self) -> &mut Vec<ClassContext>{
        self.classes.get_or_insert(vec![])
    }
    pub fn get_mut_class(&mut self, url: &Url) -> &mut ClassContext{
        let classes_ctx = self.get_mut_classes();
        let class = classes_ctx.iter_mut().find(|ref mut c| c.class.id == url.path()[5].parse::<i32>().unwrap()).unwrap();
        class
    }
    pub fn get_class(&self, url: &Url) -> &ClassContext{
        let classes = self.get_classes();
        classes.iter().find(|c| c.class.id == url.path()[5].parse::<i32>().unwrap()).unwrap()
    }
    pub fn get_next_class(&self, url: &Url) -> Option<ClassContext>{
        let classes = self.get_classes();
        let id = classes.iter().enumerate().find(|c| c.1.class.id == url.path()[5].parse::<i32>().unwrap()).unwrap();
        if id.0 + 1 < classes.len(){
            Some(classes[id.0+1].clone())
        }
        else {
            None
        }
    }
    pub fn get_prev_class(&self, url: &Url) -> Option<ClassContext>{
        let classes = self.get_classes();
        let id = classes.iter().enumerate().find(|c| c.1.class.id == url.path()[5].parse::<i32>().unwrap()).unwrap();
        if id.0 > 0 {
            Some(classes[id.0-1].clone())
        }
        else {
            None
        }
    }
    pub fn get_classes(&self) -> &Vec<ClassContext>{
        self.classes.as_ref().unwrap()
    }
}