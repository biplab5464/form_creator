use json::{object::Object,JsonValue};
use serde::{Deserialize, Serialize};
use serde_json;

trait Build{
    fn build(&self) -> String;
    fn get_id(&self) -> (u8,String);
}

pub struct Form{
    components : Vec<Box<dyn Build>>,
}

impl Form{
    pub fn run(&self) -> String{
        let mut return_str = String::new();
        for ele in self.components.iter(){
            //println!("-------------{}",ele.build());
            return_str = format!("{}{}",return_str,ele.build());
        }
        return return_str;
    }

    pub fn get_json_id(&self) -> String{

        let mut json_id = Object::new();

        for ele in self.components.iter(){
            let id = ele.get_id();
            match id.0 {
                10 => {

                    json_id.insert(id.1.as_str(),JsonValue::String(String::new()))
                },
                11 => {

                    json_id.insert(id.1.as_str(),JsonValue::Array(Vec::new()))
                },
                _ => {}
            }
        }

        json_id.dump()
    }

     pub fn from_json(json : JsonValue) -> Form {

        let mut form = Form{
            components : vec![]
        };

        for ele in json.members(){
            match ele["widget"].as_str() {
                Some("TextBox") => {
                    let tmp : TextBox = serde_json::from_str(&ele.dump()).expect("problem in coverting to textbox");
                    form.components.push(Box::new(tmp));
                },
                Some("RadioButton") =>{
                    let tmp : RadioButton = serde_json::from_str(&ele.dump()).expect("problem in coverting to Radio");
                    form.components.push(Box::new(tmp));
                },
                Some("Checkbox") =>{
                    let tmp : Checkbox = serde_json::from_str(&ele.dump()).expect("problem in coverting to Radio");
                    form.components.push(Box::new(tmp));
                },
                Some("Date") =>{
                    let tmp : Date = serde_json::from_str(&ele.dump()).expect("problem in coverting to Radio");
                    form.components.push(Box::new(tmp));
                },
                Some("Date_time") =>{
                    let tmp : Date_time = serde_json::from_str(&ele.dump()).expect("problem in coverting to Radio");
                    form.components.push(Box::new(tmp));
                },
                 Some("ComboBox") =>{
                    let tmp : ComboBox = serde_json::from_str(&ele.dump()).expect("problem in coverting to Radio");
                    form.components.push(Box::new(tmp));
                },
                _ => {}
            }
        }

        return form;
    }

    pub fn from_str(json_str : String) -> Form {

        let json = json::parse(&json_str).expect("hello, you haven't pass json check the json again"); 

        return Form::from_json(json);
    }
}


#[derive(Debug,Serialize, Deserialize)]
struct TextBox{
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    #[serde(default)]   disabled : bool,
    #[serde(default)]   required : bool,
    #[serde(default)]   default : Option<String>
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

    fn get_id(&self) -> (u8,String){
        (10,self.id.clone())
    }
}


#[derive(Debug,Serialize, Deserialize)]
struct RadioButton{
    id : String,
    label : String,
    option : Vec<String>,
    #[serde(default)]   on_new_line : bool, //the choice will be on new line
    #[serde(default)]   css_class : String, // class for the the Radio button
    #[serde(default)]   disabled : bool,
    #[serde(default)]   required : bool,
    #[serde(default)]   default : Option<String>
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

    fn get_id(&self) -> (u8,String){
        (10,self.id.clone())
    }
}


#[derive(Debug,Serialize, Deserialize)]
struct Checkbox{
    id : String,
    label : String,
    option : Vec<String>,
    #[serde(default)]   on_new_line : bool,
    #[serde(default)]   css_class : String, // class for the the Radio button
    #[serde(default)]   disabled : bool,
    #[serde(default)]   required : bool,
    #[serde(default)]   default : Option<Vec<String>>
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

    fn get_id(&self) -> (u8,String){
        (11,self.id.clone())
    }
}

#[derive(Debug,Serialize, Deserialize)]
struct Date{
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    #[serde(default)]   disabled : bool,
    #[serde(default)]   required : bool,
    #[serde(default)]    default : Option<String>
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

    fn get_id(&self) -> (u8,String){
        (10,self.id.clone())
    }

}

#[derive(Debug,Serialize, Deserialize)]
struct Date_time {
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    #[serde(default)]   disabled : bool,
    #[serde(default)]   required : bool,
   #[serde(default)]    default : Option<String>
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

    fn get_id(&self) -> (u8,String){
        (10,self.id.clone())
    }

}

#[derive(Debug,Serialize, Deserialize)]
struct ComboBox{
    id : String,
    label : String,
    option : Vec<String>,
    css_class : String, // class for the the Radio button
    #[serde(default)]   disabled : bool,
    #[serde(default)]   required : bool,
    #[serde(default)]   default : Option<Vec<String>>
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

    fn get_id(&self) -> (u8,String){
        (11,self.id.clone())
    }
}

// fn main() {

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
//     println!("{}",my_form.get_json_id());
// }
fn main() {

    let my_form = Form::from_str(r#"
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
"#.to_string());
    println!("{}",my_form.run());

}