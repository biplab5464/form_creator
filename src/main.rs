
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

struct Text_box{
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
}

impl Build for Text_box{
    fn build(&self) -> String{
        /*
            <label for="fname">First name:</label>
            <input type="text" id="fname" name="fname">
        */
        let id = self.id.clone();
        let label = self.label.clone();

        let return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"text\" id=\"{id}\" name=\"{id}\""};


    //retrun statment
    format!("{return_str} > </br>")
    }
}


struct Radio_button{
    id : String,
    label : String,
    option : Vec<String>,
    onNewLine : bool, //the choice will be on new line
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
}

impl Build for Radio_button{
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
        let id = self.id.clone();
        let label = self.label.clone();
        let mut return_str = String::new();
        return_str = format!("<label for=\"{id}\">{label}</label> </br>");
        for ele in self.option.iter(){
            //println!("log");
            return_str = format!{"{return_str}<input type=\"radio\" id=\"{ele}\" name=\"{id}\" value=\"{ele}\">
            <label for=\"{ele}\">{ele}</label>"};
            if self.onNewLine{
                return_str = format!("{return_str} </br>");
            }
        }
        format!("{return_str} </br>")
    }
}
struct Checkbox{
    id : String,
    label : String,
    option : Vec<String>,
    onNewLine : bool,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
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
        let id = self.id.clone();
        let label = self.label.clone();
        let mut return_str = String::new();
        return_str = format!("<label for=\"{id}\">{label}</label> </br>");
        for ele in self.option.iter(){
            //println!("log");
            return_str = format!{"{return_str}<input type=\"checkbox\" id=\"{ele}\" name=\"{id}\" value=\"{ele}\">
            <label for=\"{ele}\">{ele}</label>"};

            if  self.onNewLine{
                return_str = format!("{return_str} </br>");
            }
        }
        format!("{return_str}</br>")
    }
}

struct Date{
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
}

impl Build for Date{

    fn build(&self) -> String{
        /*
             <form>
              <label for="birthday">Birthday:</label>
              <input type="date" id="birthday" name="birthday">
            </form> 
        */

        let id = self.id.clone();
        let label = self.label.clone();
        let return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"date\" id=\"{id}\" name=\"{id}\""};



        format!("{return_str} > </br>")
    }

}

struct Date_time {
    id : String,
    label : String,
    css_class : String, // class for the the Radio button
    disabled : bool,
    required : bool,
}

impl Build for Date_time{

    fn build(&self) -> String{
        /*
             <form>
              <label for="birthday">Birthday:</label>
              <input type="datetime-local" id="birthday" name="birthday">
            </form> 
        */

        let id = self.id.clone();
        let label = self.label.clone();
        let return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"datetime-local\" id=\"{id}\" name=\"{id}\""};


        format!("{return_str} > </br>")
    }

}


fn main() {
    // let textbox = Text_box{
    //     id : String::from("name"),
    //     label : String::from("Enter your name :")
    // };
    // let radio = Radio_button{
    //     id : String::from("radio_btn"),
    //     label : String::from("Enter your prefer language :"),
    //     option : vec![String::from("rust"),String::from("js")],
    //     onNewLine : false 

    // };
    // let chcekbox = Checkbox{
    //     id : String::from("radio_btn"),
    //     label : String::from("Enter your prefer language :"),
    //     option : vec![String::from("rust"),String::from("js")],
    //     onNewLine : true
    // };
    //println!("{}  {} {}",textbox.build(), radio.build(),chcekbox.build());

    let my_form = Form{
        components : vec![
            Box::new(
                Text_box{
                    id : String::from("name"),
                    label : String::from("Enter your name :"),
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : false,
                    required : false,
                }
            ),
            Box::new(
                Checkbox{
                    id : String::from("radio_btn"),
                    label : String::from("Enter your prefer ide :"),
                    option : vec![String::from("eclispe"),String::from("visual code"),String::from("sublime text"),String::from("vim")],
                    onNewLine : true,
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : false,
                    required : false,
                }
            ),
            Box::new(
                Radio_button{
                    id : String::from("checkbox"),
                    label : String::from("Enter your prefer language :"),
                    option : vec![String::from("rust"),String::from("js")],
                    onNewLine : false,
                    css_class : String::from("blank"), // class for the the Radio button
                    disabled : false,
                    required : false,
                }
            )
        ]
    };

    println!("{}",my_form.run());
    
}
