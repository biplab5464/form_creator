//#[macro_use] extern crate rocket;   //uncomment to use server mode and vice versa

use std::fs::File;                // uncomment to use non_server mode
use std::io::Write;               //  uncomment to use non_server mode

use json::{object::Object,JsonValue};

mod formJson;
mod form;
pub use formJson::FormJson;
pub use form::Form;


//%%%%%%% THIS IS NON_SERVER MODE %%%%%%%%%%%%%%%
// fn main() {
//     /*let textbox = TextBox{
//         id : String::from("name"),
//         label : String::from("Enter your name :")
//     };
//     let radio = RadioButton{
//         id : String::from("radio_btn"),
//         label : String::from("Enter your prefer language :"),
//         option : vec![String::from("rust"),String::from("js")],
//         on_new_line : false 

//     };
//     let chcekbox = Checkbox{
//         id : String::from("radio_btn"),
//         label : String::from("Enter your prefer language :"),
//         option : vec![String::from("rust"),String::from("js")],
//         on_new_line : true
//     }
//     println!("{}  {} {}",textbox.build(), radio.build(),chcekbox.build());
//     */

//     let my_form = Form{
//         components : vec![
//             Box::new(
//                 TextBox{
//                     id : String::from("name"),
//                     label : String::from("Enter your name :"),
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : true,
//                     default : Some(String::from("joe")),
//                 }
//             ),
//             Box::new(
//                 Checkbox{
//                     id : String::from("my_checkbox__test"),
//                     label : String::from("Enter your prefer ide :"),
//                     option : vec![String::from("eclispe"),String::from("visual code"),String::from("sublime text"),String::from("vim")],
//                     on_new_line : true,
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : false,
//                     default : Some(vec![String::from("vim"),String::from("visual code")]),
//                 }
//             ),
//             Box::new(
//                 RadioButton{
//                     id : String::from("my_radio_test"),
//                     label : String::from("Enter your prefer language :"),
//                     option : vec![String::from("rust"),String::from("js")],
//                     on_new_line : false,
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : false,
//                     default : Some(String::from("rust")),

//                 }
//             ),
//             Box::new(
//                 Date{
//                     id : String::from("date_test"),
//                     label : String::from("Enter the date :"),
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : false,
//                     required : true,
//                     default : None,
//                 }
//             ),
//             Box::new(
//                 Date_time{
//                     id : String::from("date_time_test"),
//                     label : String::from("Enter the date time :"),
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : false,
//                     required : true,
//                     default : None,
//                 }
//             ),
//             Box::new(
//                 ComboBox{
//                     id : String::from("combo_test"),
//                     label : String::from("Enter your prefer ide :"),
//                     option : vec![String::from("eclispe"),String::from("visual code"),String::from("sublime text"),String::from("vim")],
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : false,
//                     default : Some(vec![String::from("vim"),String::from("visual code")]),
//                 }
//             )
//         ]
//     };

//     let html_form  = my_form.run();

//     println!("{}",html_form);

//     let mut file = File::create("static/index.html").expect("problem in opening file");
//     file.write_all(html_form.as_bytes()).expect("problem in writeing of file");

//     println!("{}",my_form.get_json_id());
// }
//%%%%%%% THIS IS NON_SERVER MODE %%%%%%%%%%%%%%%


//%%%%%%% THIS IS NON_SERVER MODE WITH JSON %%%%%%%%%%%%%%%
fn main() {

    let json = json::parse(r#"
[
    {
        "widget" : "TextBox",
        "id" : "name",
        "label" : "hello",
        "css_class" : "blank",
        "disabled" : true,
        "required" : true,
        "default" : "doe"
    },
    {
        "widget":"RadioButton",
        "id" : "radio",
        "label" : "radio_button",
        "css_class" : "hemlo",
        "option" : ["js","rust"],
        "default" : "js"   
    },
    {
        "widget":"Checkbox",
        "id" : "checkbox",
        "label" : "radio_box",
        "css_class" : "hemlo",
        "option" : ["vim","vs code","fleet","eclipse"],
        "default" : ["vim","vs code"]  
    },
    {
        "widget" : "Date",
        "id" : "dt",
        "label" : "date",
        "css_class" : "blank",
        "disabled" : true,
        "required" : true
    },
    {
        "widget" : "TextBox",
        "id" : "name1",
        "label" : "hello",
        "css_class" : "blank",
        "disabled" : true,
        "required" : true,
        "default" : "doe"
    },
    {
        "widget":"RadioButton",
        "id" : "radio1",
        "label" : "radio_button",
        "css_class" : "hemlo",
        "option" : ["js","rust"],
        "default" : "js"   
    },
    {
        "widget":"Checkbox",
        "id" : "checkbox1",
        "label" : "radio_box",
        "css_class" : "hemlo",
        "option" : ["vim","vs code","fleet","eclipse"],
        "default" : ["vim","vs code"]  
    },
    {
        "widget" : "Date",
        "id" : "dt1",
        "label" : "date",
        "css_class" : "blank",
        "disabled" : true,
        "required" : true
    },
    {
        "widget" : "TextBox",
        "id" : "name2",
        "label" : "hello",
        "css_class" : "blank",
        "disabled" : true,
        "required" : true,
        "default" : "doe"
    },
    {
        "widget":"RadioButton",
        "id" : "radio2",
        "label" : "radio_button",
        "css_class" : "hemlo",
        "option" : ["js","rust"],
        "default" : "js"   
    },
    {
        "widget":"Checkbox",
        "id" : "checkbox2",
        "label" : "radio_box",
        "css_class" : "hemlo",
        "option" : ["vim","vs code","fleet","eclipse"],
        "default" : ["vim","vs code"]  
    },
    {
        "widget" : "Date",
        "id" : "dt2",
        "label" : "date",
        "css_class" : "blank",
        "disabled" : true,
        "required" : true
    }
  
]
"#).unwrap();

   
    // let my_form = FormJson{
    //     form : json
    // };

    // let html_form = my_form.run();

    let my_form = Form::from_json(json);
    let html_form = my_form.run();

    let mut file = File::create("static/index.html").expect("problem in opening file");
    file.write_all(html_form.as_bytes()).expect("problem in writeing of file");

    println!("{}",html_form);
    println!("{}",my_form.get_json_id());

}
//%%%%%%% THIS IS NON_SERVER MODE WITH JSON %%%%%%%%%%%%%%%

//%%%%%%% THIS IS SERVER MODE %%%%%%%%%%%%%% 
//  ALSO UNCOMMENT THE '#[macro_use] extern crate rocket;' *** IT IS ON THE TOP

// #[launch]
// fn rocket() -> _ {
//     let server = rocket::build();

//     server.mount("/",routes![get_form])
// }

// #[get("/")]
// fn get_form() -> String{
//     let my_form = Form{
//         components : vec![
//             Box::new(
//                 TextBox{
//                     id : String::from("name"),
//                     label : String::from("Enter your name :"),
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : true,
//                     default : Some(String::from("joe")),
//                 }
//             ),
//             Box::new(
//                 Checkbox{
//                     id : String::from("radio_btn"),
//                     label : String::from("Enter your prefer ide :"),
//                     option : vec![String::from("eclispe"),String::from("visual code"),String::from("sublime text"),String::from("vim")],
//                     on_new_line : true,
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : false,
//                     default : Some(vec![String::from("vim"),String::from("visual code")]),
//                 }
//             ),
//             Box::new(
//                 RadioButton{
//                     id : String::from("checkbox"),
//                     label : String::from("Enter your prefer language :"),
//                     option : vec![String::from("rust"),String::from("js")],
//                     on_new_line : false,
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : false,
//                     default : Some(String::from("rust")),

//                 }
//             ),
//             Box::new(
//                 Date{
//                     id : String::from("date"),
//                     label : String::from("Enter the date :"),
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : false,
//                     required : true,
//                     default : None,
//                 }
//             ),
//             Box::new(
//                 Date_time{
//                     id : String::from("date_time"),
//                     label : String::from("Enter the date time :"),
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : false,
//                     required : true,
//                     default : None,
//                 }
//             ),
//             Box::new(
//                 ComboBox{
//                     id : String::from("combo"),
//                     label : String::from("Enter your prefer ide :"),
//                     option : vec![String::from("eclispe"),String::from("visual code"),String::from("sublime text"),String::from("vim")],
//                     css_class : String::from("blank"), // class for the the Radio button
//                     disabled : true,
//                     required : false,
//                     default : Some(vec![String::from("vim"),String::from("visual code")]),
//                 }
//             )
//         ]
//     };

//     //println!("{}",my_form.run());
//     my_form.run()
// }
//%%%%%%% THIS IS SERVER MODE %%%%%%%%%%%%%%


