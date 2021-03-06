use crate::{AppState, index, api};
use crate::api::users::{get_timetables, get_schools};

pub fn routes(state: AppState)->tide::Server<AppState>{
    use crate::views::{logout, reset_password, favico, robots};
    let mut app = tide::with_state(state.clone());
    app.at("/").get(index).post(index);
    app.at("/login").get(index);
    app.at("/admin").get(index);
    app.at("/admin/*").get(index);
    app.at("/signin").get(index);
    app.at("/logout").all(logout);
    app.at("/reset").get(reset_password);
    app.at("/help").get(index);
    app.at("/favicon.ico").get(favico);
    app.at("/robots.txt").get(robots);
    app.at("/schools").get(index);
    app.at("/schools/*").get(index);
    app.at("/users").get(index);
    app.at("/users/*").get(index);
    app.at("/api").nest({
        api_routes(state)
    });
    app
}

fn api_routes(state: AppState)->tide::Server<AppState>{
    let mut app = tide::with_state(state.clone());
    use crate::views;
    //use crate::middlewares::{school_auth};
    app.at("/city").nest({
        let mut api = tide::with_state(state.clone());
        api.at("").get(views::city);
        api.at("/:city")
            .get(views::town);
        api
    });
    app.at("/login").post(api::views::login);
    app.at("/login").get(api::views::get_user);
    app.at("/signin").post(api::views::signin);
    app.at("/school_types").all(crate::api::school::school::school_type);
    app.at("/days").get(views::days);
    app.at("/reset").all(views::post_reset);
    app.at("/send_key").post(views::send_key);
    app.at("/posts").get(views::get_posts);
    app.at("/posts/:post_id").delete(views::del_post);
    //app.at("/add_cities").get(views::add_cities);
    //app.at("/activities/:act_id").delete(views::activities);
    app.at("/schools").nest({
        schools_api(state.clone())
    });
    app.at("/users").nest({
        users_api(state.clone())
    });
    app.at("/admin").nest({
        admin_api(state)
    });
    app
}

fn users_api(state: AppState)->tide::Server<AppState>{
    use crate::api::users;
    use crate::middlewares::{user_auth};
    let mut api = tide::with_state(state);
    api.with(user_auth);
    api.at("/:user_id").get(users::get);
    api.at("/:user_id/timetables").get(get_timetables);
    api.at("/:user_id/schools").get(get_schools);
    api.at("/:user_id/reset").post(users::post_reset);
    api
}

fn schools_api(state: AppState)->tide::Server<AppState>{
    use crate::api::school::school;
    let mut api = tide::with_state(state.clone());
    api.at("").get(school::schools);
    api.at("/add").post(school::add);
    api.at("/:school").nest({
        school_api(state)
    });
    api
}

fn school_api(state: AppState)->tide::Server<AppState>{
    use crate::api::school::school;
    use crate::api::school::group;
    use crate::views;
    use crate::middlewares::{school_auth};
    let mut api = tide::with_state(state.clone());
    api.with(school_auth);
    api.at("").get(school::get_posts);
    api.at("/posts").post(views::posts);
    api.at("/library").get(school::get_library).post(school::library);
    api.at("/library/:library_id").patch(school::patch_library);
    api.at("/library/:library_id/books").get(school::get_books).post(school::books);
    api.at("/detail").get(school::school_detail);
    api.at("/unused_numbers").get(school::get_unused_numbers);
    api.at("/detail").patch(school::patch_school);
    api.at("/groups").get(school::get_groups).post(group::add_groups);
    api.at("/groups/:group_id").nest({
        group_api(state.clone())
    });
    api.at("/city").get(school::city);
    //app.at("/students").get(school::get_students);
    api.at("/students_with_file").post(school::students_with_file);
    api.at("/students").nest({
        students_api(state)
    });
    api.at("/subjects").get(school::get_subjects).post(school::subjects);
    api.at("/subjects/:subject_id").delete(school::del_subject);
    api.at("/class_rooms").get(school::get_class_rooms).post(school::class_rooms);
    api.at("/class_rooms/:class_room_id").delete(school::del_class_room);
    api.at("/teachers").with(school_auth).get(school::get_teachers).post(school::teachers);
    api
}

fn students_api(state: AppState)->tide::Server<AppState>{
    use crate::api::school::student;
    use crate::api::school::school;
    let mut api = tide::with_state(state);
    api.at("").post(school::students).get(school::get_students);
    api.at("/:student_id").get(get_timetables).delete(student::del_student);
    api
}

fn group_api(state: AppState)->tide::Server<AppState>{
    use crate::api::school::school;
    use crate::api::school::{group};
    use crate::middlewares::{group_auth};
    let mut group_api = tide::with_state(state.clone());
    group_api.with(group_auth);
    group_api.at("").patch(group::patch_group).delete(group::del_group).get(group::get_group);
    group_api.at("/schedules").get(group::group_schedules).patch(group::patch_group_schedules);
    group_api.at("/timetables").get(group::get_timetables).post(group::timetables);
    group_api.at("/class_rooms").get(school::get_class_rooms).post(school::class_rooms);
    group_api.at("/classes").get(group::get_classes);
    group_api.at("/classes/limitations").post(group::limitations);
    group_api.at("/students").get(group::get_students);
    group_api.at("/add_class").post(group::add_class);
    group_api.at("/activities").post(group::add_activity);
    group_api.at("/classes/:class_id").nest({
        class_api(state.clone())
    });
    group_api.at("/teachers/:teacher_id").nest({
        teacher_api(state)
    });
    group_api
}

fn class_api(state: AppState)->tide::Server<AppState>{
    use crate::api::school::class;
    use crate::middlewares::{class_auth};
    let mut class_api = tide::with_state(state);
    class_api.with(class_auth);
    class_api.at("").get(class::class_detail).delete(class::class_delete).patch(class::update_class);
    class_api.at("/activities").get(class::activities);
    class_api.at("/activities/:act_id").delete(class::del_act);
    class_api.at("/limitations").all(class::limitations);
    class_api.at("/timetables").get(class::timetables);
    class_api.at("/students").get(class::get_students).post(class::students);
    class_api.at("/students/:student_id").delete(class::del_student);
    class_api.at("/all_students").get(class::get_all_students);
    class_api
}

fn teacher_api(state: AppState)->tide::Server<AppState>{
    use crate::api::school::teacher;
    let mut teacher_api = tide::with_state(state);
    teacher_api.at("").get(teacher::teacher_detail).patch(teacher::patch_teacher).delete(teacher::del_teacher);
    teacher_api.at("/activities").get(teacher::get_activities);
    //teacher_api.at("/activities/:act_id").patch(teacher::patch_activities);
    teacher_api.at("/activities/:act_id").delete(teacher::del_activities);
    teacher_api.at("/limitations").get(teacher::get_limitations).post(teacher::limitations);
    teacher_api.at("/timetables").get(teacher::timetables);
    teacher_api
}

fn admin_api(state: AppState)->tide::Server<AppState>{
    let mut app = tide::with_state(state);
    use crate::api::admin::views;
    app.at("/school_types").post(views::add_school_type).get(crate::api::school::school::school_type);
    app.at("/subjects").post(views::add_subject);
    app.at("/subjects/:school_type").get(views::get_subjects);
    app
}