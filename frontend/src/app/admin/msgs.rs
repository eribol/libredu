use shared::msgs::admin::AdminDownMsgs;

pub fn get_msg(msg: AdminDownMsgs){
    match msg{
        AdminDownMsgs::LastSchools(schools)=> super::last_schools().lock_mut().replace_cloned(schools),
        AdminDownMsgs::GetSchool(school)=> super::school::school().set(Some(school)),
        AdminDownMsgs::GetTimetables(tts)=> super::school::timetables().lock_mut().replace_cloned(tts),
        AdminDownMsgs::GetClasses(clss)=> super::timetables::classes().lock_mut().replace_cloned(clss),
        AdminDownMsgs::GetClassesLimitations(clss)=> {
            for c in clss{
                super::timetables::class_limitations().lock_mut().push_cloned(c);    
            }
        },
        AdminDownMsgs::GetTeachers(clss)=> super::timetables::teachers().lock_mut().replace_cloned(clss),
        AdminDownMsgs::GetTeachersLimitations(clss)=> {

            super::timetables::teachers_limitations().lock_mut().replace_cloned(clss);
        }
        AdminDownMsgs::GetSchoolMessages(msgs) => super::messages::messages().lock_mut().replace_cloned(msgs),
        _ => ()
    }
}