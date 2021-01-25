use serde::*;
use seed::{*, prelude::*};
use crate::{Context, print_class_rooms};
use crate::page::school::detail::{SchoolContext, GroupContext};
use crate::model::class_room::Classroom;
use web_sys::{HtmlSelectElement, HtmlOptionElement};
use crate::model::class::Class;
use crate::model::student::SimpleStudent;

pub fn init(_url: Url, ctx_school: &mut SchoolContext, orders: &mut impl Orders<Msg>, ctx_group: &mut GroupContext) -> Model{
    let model = Model::default();
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/class_rooms", ctx_school.school.id, &ctx_group.group.id);
        let request = Request::new(url)
            .method(Method::Get);
        async {
            Msg::FetchClassroom(async {
                request
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }.await)
        }
    });
    model
}
#[derive(Debug, Default, Clone)]
pub struct Model{
    class_rooms: Vec<Classroom>,
    test: String,
    select1: ElRef<HtmlSelectElement>,
    select2: ElRef<HtmlSelectElement>,
    select3: ElRef<HtmlSelectElement>,
    select4: ElRef<HtmlSelectElement>,
    select_class_rooms: ElRef<HtmlSelectElement>,
    group1_pool: Vec<SimpleStudent>,
    group2_pool: Vec<SimpleStudent>,
    group3_pool: Vec<SimpleStudent>,
    group4_pool: Vec<SimpleStudent>,
    group1: Vec<i32>,
    group2: Vec<i32>,
    group3: Vec<i32>,
    group4: Vec<i32>,
    selected_class_rooms: Vec<Classroom>,
    students: Vec<(i32, Vec<SimpleStudent>)>,
    design: Vec<Vec<ClassRoomDesing>>
}

#[derive(Debug)]
pub enum Msg {
    FetchClassroom(fetch::Result<Vec<Classroom>>),
    ChangeGroup1(String),
    ChangeGroup2(String),
    ChangeGroup3(String),
    ChangeGroup4(String),
    ChangeClassRooms(String),
    FetchStudents(fetch::Result<Vec<(i32, Vec<SimpleStudent>)>>),
    Submit,
    Start,
    Print
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClassRoomDesing {
    class_room: Option<Classroom>,
    rw: i16,
    cl: i16,
    width: Vec<Option<StudentClass>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StudentClass {
    class: Class,
    student: SimpleStudent
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group:&mut GroupContext) {
    match msg {
        Msg::FetchClassroom(class_rooms) => {
            match class_rooms{
                Ok(cr) => {
                    model.class_rooms = cr;
                }
                Err(_) => {
                    model.test = "Derslikler indirilemedi".to_string()
                }
            }
        }
        Msg::ChangeGroup1(_) => {
            model.group1 = vec![];
            let selected_options = model.select1.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                model.group1.push(item.value().parse::<i32>().unwrap());
                let select2 = model.select2.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup2("".to_string()));
                                break;
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select3.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup3("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select4.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup4("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
            }
            //log!(&model.group1);
        }
        Msg::ChangeGroup2(_) => {
            model.group2 = vec![];
            let selected_options = model.select2.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                model.group2.push(item.value().parse::<i32>().unwrap());
                let select2 = model.select1.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup1("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select3.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup3("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select4.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup4("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
            }
        }
        Msg::ChangeGroup3(_) => {
            model.group3 = vec![];
            let selected_options = model.select3.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                model.group3.push(item.value().parse::<i32>().unwrap());
                let select2 = model.select1.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup1("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select2.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup2("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select4.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup4("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
            }
        }
        Msg::ChangeGroup4(_) => {
            model.group4 = vec![];
            let selected_options = model.select4.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                model.group4.push(item.value().parse::<i32>().unwrap());
                let select2 = model.select1.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup1("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select2.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup2("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
                let select2 = model.select3.get().unwrap().selected_options();
                for s2 in 0..select2.length() {
                    let item2 = select2.item(s2);//.unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                    match item2 {
                        Some(i2) => {
                            if item.value() == i2.clone().dyn_into::<HtmlOptionElement>().unwrap().value() {
                                i2.dyn_into::<HtmlOptionElement>().unwrap().set_selected(false);
                                orders.send_msg(Msg::ChangeGroup3("".to_string()));
                                break
                            }
                        }
                        None => {}
                    }
                }
            }
        }
        Msg::ChangeClassRooms(_) => {
            model.selected_class_rooms = vec![];
            let selected_options = model.select_class_rooms.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                let room = model.class_rooms.iter().find(|c| c.id == item.value().parse::<i32>().unwrap()).unwrap();
                model.selected_class_rooms.push(room.clone());
            }
            if model.selected_class_rooms.len() == 0 {
                model.test = "Derslik seçmediniz".to_string()
            }
        }
        Msg::FetchStudents(students) => {
            //log!(&students);
            match students{
                Ok(stdnts) => {
                    model.students = stdnts.clone();
                    model.group4_pool.clear();
                    model.group3_pool.clear();
                    model.group2_pool.clear();
                    model.group1_pool.clear();
                    for s in stdnts{
                        let group1 = model.group1.iter().find(|g| g == &&s.0);
                        match group1{
                            None => {
                                let group2 = model.group2.iter().find(|g| g == &&s.0);
                                match group2{
                                    None => {
                                        let group3 = model.group3.iter().find(|g| g == &&s.0);
                                        match group3{
                                            None => {
                                                let group4 = model.group4.iter().find(|g| g == &&s.0);
                                                match group4{
                                                    None => {}
                                                    Some(_) => {
                                                        model.group4_pool.append(&mut s.1.clone());
                                                    }
                                                }
                                            }
                                            Some(_) => {
                                                model.group3_pool.append(&mut s.1.clone());
                                            }
                                        }
                                    }
                                    Some(_) => {
                                        model.group2_pool.append(&mut s.1.clone());
                                    }
                                }
                            }
                            Some(_) => {
                                model.group1_pool.append(&mut s.1.clone());
                            }
                        }
                    }
                    orders.send_msg(Msg::Start);
                }
                Err(_) => {
                    model.test = "Öğrenciler indirilemedi".to_string()
                }
            }
        }
        Msg::Submit => {
            //del.selected_class_rooms[0];
            //if model.selected_class_rooms.iter().fold(|a, b| b)
            let mut t = 0;
            for cls in &model.selected_class_rooms{
                t += cls.rw*cls.cl;
            }
            if model.selected_class_rooms.len() == 0{
                model.test = "Herhangi bir derslik seçmediniz".to_string()
            }
            else if model.group1_pool.len() > t as usize{
                model.test = "1. grup öğrenci sayısı derslik sıra sayısından büyük. Lütfen ya 1. gruptan öğrenci silin veya daha fazla derslik seçin".to_string()
            }
            else if model.group2_pool.len() > t as usize{
                model.test = "2. grup öğrenci sayısı derslik sıra sayısından büyük. Lütfen ya 2. gruptan öğrenci silin veya daha fazla derslik seçin".to_string()
            }
            else if model.group3_pool.len() > t as usize{
                model.test = "3. grup öğrenci sayısı derslik sıra sayısından büyük. Lütfen ya 3. gruptan öğrenci silin veya daha fazla derslik seçin".to_string()
            }
            else if model.group4_pool.len() > t as usize{
                model.test = "4. grup öğrenci sayısı derslik sıra sayısından büyük. Lütfen ya 4. gruptan öğrenci silin veya daha fazla derslik seçin".to_string()
            }
            else {
                model.test = "".to_string();
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/students", ctx_school.school.id, &ctx_group.group.id);
                    let request = Request::new(url)
                        .method(Method::Get);
                    async {
                        Msg::FetchStudents(async {
                            request
                                .fetch()
                                .await?
                                .check_status()?
                                .json()
                                .await
                        }.await)
                    }
                });
            }

        }
        Msg::Start => {
            let mut design: Vec<ClassRoomDesing> = vec![];
            for selected_room in &model.selected_class_rooms{
                for rw in 0..selected_room.rw{
                    for cl in 0..selected_room.cl {
                        let d = ClassRoomDesing {
                            class_room: Some(selected_room.clone()),
                            rw,
                            cl,
                            width: vec![None; selected_room.width as usize]
                        };
                        design.push(d);
                    }
                }
            }
            for student in &model.group1_pool{
                let placed = find_desk(&design, &model.group1_pool);
                //log!("placed1:", &placed);
                if placed.0{
                    let student_class_id = model.students.iter().find(|s| s.1.iter().any(|s2| s2.id == student.id)).unwrap();
                    let class = ctx_group.classes.iter().find(|c| c.id == student_class_id.0).unwrap();
                    let st_class = StudentClass{ class: class.clone(), student: student.clone() };
                    design[placed.1].width[placed.2] = Some(st_class);
                }
            }
            for student in &model.group2_pool{
                let placed = find_desk(&design, &model.group2_pool);
                //log!("placed2:", &placed);
                if placed.0{
                    let student_class_id = model.students.iter().find(|s| s.1.iter().any(|s2| s2.id == student.id)).unwrap();
                    let class = ctx_group.classes.iter().find(|c| c.id == student_class_id.0).unwrap();
                    let st_class = StudentClass{ class: class.clone(), student: student.clone() };
                    design[placed.1].width[placed.2] = Some(st_class);
                }
            }
            for student in &model.group3_pool{
                let placed = find_desk(&design, &model.group3_pool);
                //log!("placed2:", &placed);
                if placed.0{
                    let student_class_id = model.students.iter().find(|s| s.1.iter().any(|s2| s2.id == student.id)).unwrap();
                    let class = ctx_group.classes.iter().find(|c| c.id == student_class_id.0).unwrap();
                    let st_class = StudentClass{ class: class.clone(), student: student.clone() };
                    design[placed.1].width[placed.2] = Some(st_class);
                }
            }
            for student in &model.group4_pool{
                let placed = find_desk(&design, &model.group4_pool);
                //log!("placed2:", &placed);
                if placed.0{
                    let student_class_id = model.students.iter().find(|s| s.1.iter().any(|s2| s2.id == student.id)).unwrap();
                    let class = ctx_group.classes.iter().find(|c| c.id == student_class_id.0).unwrap();
                    let st_class = StudentClass{ class: class.clone(), student: student.clone() };
                    design[placed.1].width[placed.2] = Some(st_class);
                }
            }
            for scr in &model.selected_class_rooms{
                //design.sort_by(|a, b| a.class_room.as_ref().unwrap().id.cmp(&b.class_room.as_ref().unwrap().id).cmp(&a.class_room.as_ref().unwrap().rw.cmp(&b.class_room.as_ref().unwrap().rw)));
                let f = design.clone().into_iter().filter(|d| d.class_room.as_ref().unwrap().id==scr.id).collect::<Vec<ClassRoomDesing>>();
                model.design.push(f.clone());
            }

            //log!(&design);
        }
        Msg::Print => {
            print_class_rooms(&serde_json::to_string(&model.design).unwrap());
        }
    }
}

fn find_desk(class_room_design: &Vec<ClassRoomDesing>, group1: &Vec<SimpleStudent>) -> (bool, usize, usize){
    let mut id = 0;
    let mut wdt = 0;
    let mut placed = false;
    for crd in 0..class_room_design.len(){
        //placed = false;
        if class_room_design[crd].width.iter().all(|w| w.is_none()){
            //log!("group1");
            id = crd;
            wdt = 0;
            placed = true;
            break;
        }
        else if class_room_design[crd].width.iter().all(|w| w.is_some()) {
            continue;
        }
        else {
            for i in 1..class_room_design[crd].width.len() {
                //let prev_student_desk = class_room_design.iter().find(|c| c.rw == class_room_design[crd].rw && c.cl == class_room_design[crd].cl).unwrap();
                let prev_student_id = class_room_design[crd].width[i - 1].as_ref().unwrap().student.id;
                if group1.iter().any(|g| g.id == prev_student_id) {
                    continue;
                } else {
                    id = crd;
                    wdt = i;
                    placed = true;
                    break
                }
            }
            if placed{
                //id = crd;
                break;
            }
        }
    }
    (placed, id, wdt)
}
pub fn view(model: &Model, ctx_group: &GroupContext) -> Node<Msg>{
    div![
        //C!{"columns"},
        div![
            C!{"columns"},
            div![
                C!{"column is-full"},
                h2![
                    C!{"title is-2"},
                    "Sınıf Öğrencilerini Gruplandırın"
                ],
                hr![],
            ]
        ],
        div![
            C!{"columns"},
            div![
                C!{"column"},
                h3![
                    C!{"title is-3"},
                    "1. Grup"
                ],
                hr![],
                select![
                    el_ref(&model.select1),
                    attrs!{At::Multiple => true.as_at_value(), At::Size => "8"},
                    ctx_group.classes.iter().map(|c|
                        option![
                            attrs!{At::Value=>&c.id},
                            &c.kademe.to_string(), "/", &c.sube," Sınıfı"
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeGroup1)
                ]
            ],
            div![
                C!{"column"},
                h3![
                    C!{"title is-3"},
                    "2. Grup"
                ],
                hr![],
                select![
                    el_ref(&model.select2),
                    attrs!{At::Multiple => true.as_at_value(), At::Size => "8"},
                    ctx_group.classes.iter().map(|c|
                        option![
                            attrs!{At::Value=>&c.id},
                            &c.kademe.to_string(), "/", &c.sube," Sınıfı"
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeGroup2)
                ]
            ],
            div![
                C!{"column"},
                h3![
                    C!{"title is-3"},
                    "3. Grup"
                ],
                hr![],
                select![
                    el_ref(&model.select3),
                    attrs!{At::Multiple => true.as_at_value(), At::Size => "8"},
                    ctx_group.classes.iter().map(|c|
                        option![
                            attrs!{At::Value=>&c.id},
                            &c.kademe.to_string(), "/", &c.sube," Sınıfı"
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeGroup3)
                ]
            ],
            div![
                C!{"column"},
                h3![
                    C!{"title is-3"},
                    "4. Grup"
                ],
                hr![],
                select![
                    el_ref(&model.select4),
                    attrs!{At::Multiple => true.as_at_value(), At::Size => "8"},
                    ctx_group.classes.iter().map(|c|
                        option![
                            attrs!{At::Value=>&c.id},
                            &c.kademe.to_string(), "/", &c.sube," Sınıfı"
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeGroup4)
                ]
            ]
        ],
        div![
            C!{"columns"},
            div![
                C!{"column is-full"},
                h2![
                    C!{"title is-2"},
                    "Derslikleri Seçin"
                ],
                hr![],
                select![
                    el_ref(&model.select_class_rooms),
                    attrs!{At::Multiple => true.as_at_value(), At::Size => "8"},
                    model.class_rooms.iter().map(|c|
                        option![
                            attrs!{At::Value=>&c.id},
                            &c.name," Dersliği"
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeClassRooms)
                ]
            ]
        ],
        div![
            C!{"columns"},
            div![
                C!{"column"},
                hr![],
                input![C!{"button is-primary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Oluştur"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::Submit
                    })
                ]
            ],
            div![
                C!{"column"},
                hr![],
                input![C!{"button is-primary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Yazdır"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::Print
                    })
                ]
            ]
        ],
        div![
            C!{"columns"},
            div![
                C!{"column is-full"},
                hr![],
                label![
                    C!{"is-dangerous"},
                    &model.test
                ]
            ]
        ]
    ]
}