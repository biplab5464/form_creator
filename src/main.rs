//#[macro_use] extern crate rocket;   //uncomment to use server mode and vice versa

use std::fs::File;                // uncomment to use non_server mode
use std::io::Write;               //  uncomment to use non_server mode

trait Build{
    fn build(&self) -> String;
}

struct Form{
    components : Vec<Box<dyn Build>>,
}

impl Form{
    fn run(&self) -> String{
        let mut return_str = String::new();
        for ele in self.components.iter(){
            //println!("-------------{}",ele.build());
            return_str = format!("{}{}",return_str,ele.build());
        }
        return return_str;
    }
}

struct TextBox{
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
    default : Option<String>
}

impl Build for TextBox{
    fn build(&self) -> String{
        /*
            <label for="fname">First name:</label>
            <input type="text" id="fname" name="fname ">
        */
        let id = &self.id;
        let label = &self.label;
        let css_class = &self.css_class;

        //--------------------------------------------------------------------
        //**base html
        let mut return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"text\" css_class=\"{css_class}\" id=\"{id}\" name=\"{id}\""};

        //++ requried
        if self.required {
            return_str = format!("{return_str} required")
        }

        //++ disabled
        if self.disabled {
            return_str = format!("{return_str} disabled")
        }

        //++ value="Doe"  (might have potencial problem --when refreshed change to defalut )
        if let Some(str) = &self.default{
            return_str = format!("{return_str} value=\"{str}\"")
        }


        //----------------------------------------------------------------
        //retrun statment
        format!("{return_str} > </br>")
    }
}


struct RadioButton{
    id : String,
    label : String,
    option : Vec<String>,
    on_new_line : bool, //the choice will be on new line
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
    default : Option<String>
}

impl Build for RadioButton{
    fn build(&self) -> String{
        /*
            --in a loop
            <input type="radio" id="html" name="fav_language" value="HTML">
            <label for="html">HTML</label><br>
            <input type="radio" id="css" name="fav_language" value="CSS">
            <label for="css">CSS</label><br>
            <input type="radio" id="javascript" name="fav_language" value="JavaScript">
            <label for="javascript">JavaScript</label>
        */
        //println!("hello from radio");
        let id = &self.id;
        let label = &self.label;

        //------------------------------------------------------
        let mut return_str = String::new();
        return_str = format!("<label for=\"{id}\">{label}</label> </br>");
        for ele in self.option.iter(){
            //println!("log");
            return_str = format!{"{return_str}<input type=\"radio\" id=\"{ele}\" name=\"{id}\" value=\"{ele}\""};

            //--checked (might have potencial problem --when refreshed change to defalut )
            if let Some(str) = &self.default{
                if str == ele{
                    return_str = format!("{return_str} checked")
                }
            }
            //++ disabled
            if self.disabled {
                return_str = format!("{return_str} disabled")
            }

            return_str = format!("{return_str} ><label for=\"{ele}\">{ele}</label>");

            //++ on new line
            if self.on_new_line{
                return_str = format!("{return_str} </br>");
            }
        }
        //------------------------------------------------------
        format!("{return_str} </br>")
    }
}
struct Checkbox{
    id : String,
    label : String,
    option : Vec<String>,
    on_new_line : bool,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
    default : Option<Vec<String>>
}

impl Build for Checkbox{
    fn build(&self) -> String{
        /*
            --in a loop
            <input type="checkbox" id="html" name="fav_language" value="HTML">
            <label for="html">HTML</label><br>
            <input type="checkbox" id="css" name="fav_language" value="CSS">
            <label for="css">CSS</label><br>
            <input type="checkbox" id="javascript" name="fav_language" value="JavaScript">
            <label for="javascript">JavaScript</label>
        */
        let id = &self.id;
        let label = &self.label;
        //----------------------------------------------------------------------
        let mut return_str = String::new();
        return_str = format!("<label for=\"{id}\">{label}</label> </br>");
        for ele in self.option.iter(){
            //println!("log");
            return_str = format!{"{return_str}<input type=\"checkbox\" id=\"{ele}\" name=\"{id}\" value=\"{ele}\""};


            //--checked (might have potencial problem --when refreshed change to defalut ) (do this need perf encance like (removind ele already searched))
            if let Some(vec) = &self.default{
                    if vec.contains(ele){ 
                        return_str = format!("{return_str} checked")
                    }
            }

            //++ disabled
            if self.disabled {
            return_str = format!("{return_str} disabled")
            }


            return_str = format!("{return_str} ><label for=\"{ele}\">{ele}</label>");

            //++ new line
            if  self.on_new_line{
                return_str = format!("{return_str} </br>");
            }
        }
        //----------------------------------------------------------------------
        format!("{return_str}</br>")
    }
}

struct Date{
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
    default : Option<String>
}

impl Build for Date{

    fn build(&self) -> String{
        /*
             <form>
              <label for="birthday">Birthday:</label>
              <input type="date" id="birthday" name="birthday">
            </form> 
        */

        let id = &self.id;
        let label = &self.label;

        //-------------------------------------------------
        let mut return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"date\" id=\"{id}\" name=\"{id}\""};

        //++ requried
        if self.required {
            return_str = format!("{return_str} required")
        }

        //++ disabled
        if self.disabled {
            return_str = format!("{return_str} disabled")
        }

        //++ value="Doe"  (might have potencial problem --when refreshed change to defalut )
        if let Some(str) = &self.default{
            return_str = format!("{return_str} value=\"{str}\"")
        }
        //-------------------------------------------------
        format!("{return_str} > </br>")
    }

}

struct Date_time {
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
    default : Option<String>
}

impl Build for Date_time{

    fn build(&self) -> String{
        /*
             <form>
              <label for="birthday">Birthday:</label>
              <input type="datetime-local" id="birthday" name="birthday">
            </form> 
        */

        let id = &self.id;
        let label = &self.label;

        //----------------------------------------------------------------
        let mut return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"datetime-local\" id=\"{id}\" name=\"{id}\""};

        //++ requried
        if self.required {
            return_str = format!("{return_str} required")
        }

        //++ disabled
        if self.disabled {
            return_str = format!("{return_str} disabled")
        }

        //++ value="Doe"  (might have potencial problem --when refreshed change to defalut )
        if let Some(str) = &self.default{
            return_str = format!("{return_str} value=\"{str}\"")
        }
        //----------------------------------------------------------------
        format!("{return_str} > </br>")
    }

}

struct ComboBox{
    id : String,
    label : String,
    option : Vec<String>,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
    default : Option<Vec<String>>
}

impl Build for ComboBox{
    fn build(&self) -> String{
        /*
           <label for="lang">Language</label>
          <select name="languages" id="lang">
            <option value="javascript">JavaScript</option>
            <option value="php">PHP</option>
            <option value="java">Java</option>
            <option value="golang">Golang</option>
            <option value="python">Python</option>
            <option value="c#">C#</option>
            <option value="C++">C++</option>
            <option value="erlang">Erlang</option>
          </select>
        */
        let id = &self.id;
        let label = &self.label;
        //----------------------------------------------------------------------
        let mut return_str = String::new();
        return_str = format!("<label for=\"{id}\">{label}</label><select name=\"{id}\" id=\"{id}\"");


        return_str = format!("{return_str} > <option value=\"-\">_NONE_</option>");
        for ele in self.option.iter(){
            return_str = format!("{return_str}<option value=\"{ele}\">{ele}</option>");
        }
        //----------------------------------------------------------------------
        format!("{return_str}</select> </br>")
    }
}

//%%%%%%% THIS IS NON_SERVER MODE %%%%%%%%%%%%%%%
fn main() {
    /*let textbox = TextBox{
        id : String::from("name"),
        label : String::from("Enter your name :")
    };
    let radio = RadioButton{
        id : String::from("radio_btn"),
        label : String::from("Enter your prefer language :"),
        option : vec![String::from("rust"),String::from("js")],
        on_new_line : false 

    };
    let chcekbox = Checkbox{
        id : String::from("radio_btn"),
        label : String::from("Enter your prefer language :"),
        option : vec![String::from("rust"),String::from("js")],
        on_new_line : true
    }
    println!("{}  {} {}",textbox.build(), radio.build(),chcekbox.build());
    */

    let my_form = Form{
        components : vec![
            Box::new(
                TextBox{
                    id : String::from("name"),
                    label : String::from("Enter your name :"),
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : true,
                    required : true,
                    default : Some(String::from("joe")),
                }
            ),
            Box::new(
                Checkbox{
                    id : String::from("radio_btn"),
                    label : String::from("Enter your prefer ide :"),
                    option : vec![String::from("eclispe"),String::from("visual code"),String::from("sublime text"),String::from("vim")],
                    on_new_line : true,
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : true,
                    required : false,
                    default : Some(vec![String::from("vim"),String::from("visual code")]),
                }
            ),
            Box::new(
                RadioButton{
                    id : String::from("checkbox"),
                    label : String::from("Enter your prefer language :"),
                    option : vec![String::from("rust"),String::from("js")],
                    on_new_line : false,
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : true,
                    required : false,
                    default : Some(String::from("rust")),

                }
            ),
            Box::new(
                Date{
                    id : String::from("date"),
                    label : String::from("Enter the date :"),
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : false,
                    required : true,
                    default : None,
                }
            ),
            Box::new(
                Date_time{
                    id : String::from("date_time"),
                    label : String::from("Enter the date time :"),
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : false,
                    required : true,
                    default : None,
                }
            ),
            Box::new(
                ComboBox{
                    id : String::from("combo"),
                    label : String::from("Enter your prefer ide :"),
                    option : vec![String::from("eclispe"),String::from("visual code"),String::from("sublime text"),String::from("vim")],
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : true,
                    required : false,
                    default : Some(vec![String::from("vim"),String::from("visual code")]),
                }
            )
        ]
    };

    let html_form  = my_form.run();

    println!("{}",html_form);

    let mut file = File::create("static/index.html").expect("problem in opening file");
    file.write_all(html_form.as_bytes()).expect("problem in writeing of file");
}
//%%%%%%% THIS IS NON_SERVER MODE %%%%%%%%%%%%%%%


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


