use shared::msgs::admin::AdminDownMsgs;

pub fn get_msg(msg: AdminDownMsgs){
    match msg{
        AdminDownMsgs::LastSchools(schools)=> super::last_schools().lock_mut().replace_cloned(schools),
        AdminDownMsgs::GetSchool(school)=> super::selected_school().set(true)
    }
}