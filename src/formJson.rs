use json::{*,JsonValue,object::Object};


pub struct FormJson{
	pub form : JsonValue
}

impl FormJson{

	pub fn run(&self)-> String {
		let mut return_str = String::new();
	    for (index,ele) in self.form.members().enumerate(){
	        match ele["widget"].as_str() {
	            Some("TextBox") => {
	               return_str = format!("{return_str}{}",self.textbox(index));
	            },
	            Some("RadioButton") =>{
	               return_str = format!("{return_str}{}",self.radio_button(index))
	            },
	            Some("Checkbox") =>{
	               return_str = format!("{return_str}{}",self.check_box(index))
	            },
	            Some("Date") =>{
	               return_str = format!("{return_str}{}",self.date(index))
	            },
	            Some("Date_time") =>{
	               return_str = format!("{return_str}{}",self.date_time(index))
	            },
	            _ => {}
	        }
	    }
	    return_str
	}

	pub fn get_json_id(&self)-> String {
		let mut json_id = Object::new();
        for ele in self.form.members(){
        	let id : &str = ele["id"].as_str().unwrap();
            match ele["widget"].as_str() {
                Some("TextBox") | Some("RadioButton") | Some("Date") | Some("Date_time")=>
                    json_id.insert(id,JsonValue::String(String::new())),
                Some("Checkbox") => 
                    json_id.insert(id,JsonValue::Array(Vec::new())),
                _ => {}
            }
        }

        json_id.dump()
	}

	fn textbox(&self,index : usize) -> String{

		println!{"{}",index}
		let id = &self.form[index]["id"];
        let label = &self.form[index]["label"];
        let css_class = &self.form[index]["css_class"];

        //--------------------------------------------------------------------
        //**base html
        let mut return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"text\" css_class=\"{css_class}\" id=\"{id}\" name=\"{id}\""};

        //++ requried
        if self.form[index]["required"] == true {
            return_str = format!("{return_str} required")
        }

        //++ disabled
        if self.form[index]["disabled"] == true{
            return_str = format!("{return_str} disabled")
        }

        //++ value="Doe"  (might have potencial problem --when refreshed change to defalut )
        if !self.form[index]["default"].is_null(){
            let str = &self.form[index]["default"];
            return_str = format!("{return_str} value=\"{str}\"")
        }


        //----------------------------------------------------------------
        //retrun statment
        format!("{return_str} > </br>")
	}

	fn radio_button(&self,index : usize) -> String{
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
        println!{"{}",index}
        let id = &self.form["id"];
        let label = &self.form[index]["label"];

        //------------------------------------------------------
        let mut return_str = String::new();
        return_str = format!("<label for=\"{id}\">{label}</label> </br>");
        for ele in self.form[index]["option"].members(){
            //println!("log");
            return_str = format!{"{return_str}<input type=\"radio\" id=\"{ele}\" name=\"{id}\" value=\"{ele}\""};

            //--checked (might have potencial problem --when refreshed change to defalut )
            if self.form[index]["default"] == *ele{
                return_str = format!("{return_str} checked")
            }
            //++ disabled
            if self.form[index]["disabled"] == true{
                return_str = format!("{return_str} disabled")
            }

            return_str = format!("{return_str} ><label for=\"{ele}\">{ele}</label>");

            //++ on new line
            if self.form[index]["checked"] == true{
                return_str = format!("{return_str} </br>");
            }
        }
        //------------------------------------------------------
        format!("{return_str} </br>")
    }

     fn check_box(&self,index : usize) -> String{
        /*
            --in a loop
            <input type="checkbox" id="html" name="fav_language" value="HTML">
            <label for="html">HTML</label><br>
            <input type="checkbox" id="css" name="fav_language" value="CSS">
            <label for="css">CSS</label><br>
            <input type="checkbox" id="javascript" name="fav_language" value="JavaScript">
            <label for="javascript">JavaScript</label>
        */
        println!{"{}",index}
        let id = &self.form[index]["id"];
        let label = &self.form[index]["label"];
        //----------------------------------------------------------------------
        let mut return_str = String::new();
        return_str = format!("<label for=\"{id}\">{label}</label> </br>");
        for ele in self.form[index]["option"].members(){
            //println!("log");
            return_str = format!{"{return_str}<input type=\"checkbox\" id=\"{ele}\" name=\"{id}\" value=\"{ele}\""};

            // --checked (might have potencial problem --when refreshed change to defalut ) (do this need perf encance like (removind ele already searched))
            for var in self.form[index]["default"].members(){
                if ele == var{ 
                    return_str = format!("{return_str} checked")
                }
            }

            //++ disabled
            if self.form[index]["disabled"] == true{
            return_str = format!("{return_str} disabled")
            }


            return_str = format!("{return_str} ><label for=\"{ele}\">{ele}</label>");

            //++ new line
            if self.form[index]["checked"] == true{
                return_str = format!("{return_str} </br>");
            }
        }
        //----------------------------------------------------------------------
        format!("{return_str}</br>")
    }

    fn date(&self,index : usize) -> String{
        /*
             <form>
              <label for="birthday">Birthday:</label>
              <input type="date" id="birthday" name="birthday">
            </form> 
        */
        println!{"{}",index}
        let id = &self.form[index]["id"];;
        let label = &self.form[index]["label"];

        //-------------------------------------------------
        let mut return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"date\" id=\"{id}\" name=\"{id}\""};

        //++ requried
        if self.form[index]["required"] == true {
            return_str = format!("{return_str} required")
        }

        //++ disabled
        if self.form[index]["disabled"] == true {
            return_str = format!("{return_str} disabled")
        }

        //++ value="Doe"  (might have potencial problem --when refreshed change to defalut )
        if !self.form[index]["default"].is_null(){
            let str = &self.form[index]["default"];
            return_str = format!("{return_str} value=\"{str}\"")
        }
        //-------------------------------------------------
        format!("{return_str} > </br>")
    }

    fn date_time(&self,index : usize) -> String{
        /*
             <form>
              <label for="birthday">Birthday:</label>
              <input type="date" id="birthday" name="birthday">
            </form> 
        */
        println!{"{}",index}
        let id = &self.form[index]["id"];;
        let label = &self.form[index]["label"];

        //-------------------------------------------------
        let mut return_str : String = format!{"<label for=\"{id}\">{label}</label>
            <input type=\"datetime-local\" id=\"{id}\" name=\"{id}\""};

        //++ requried
        if self.form[index]["required"] == true {
            return_str = format!("{return_str} required")
        }

        //++ disabled
        if self.form[index]["disabled"] == true {
            return_str = format!("{return_str} disabled")
        }

        //++ value="Doe"  (might have potencial problem --when refreshed change to defalut )
        if !self.form[index]["default"].is_null(){
            let str = &self.form[index]["default"];
            return_str = format!("{return_str} value=\"{str}\"")
        }
        //-------------------------------------------------
        format!("{return_str} > </br>")
    }

}


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

   
    let my_form = FormJson{
    	form : json
    };

    println!("{}",my_form.run());
    println!("{}",my_form.get_json_id());


  //   for ele in json.members(){
  //       println!("---{}",ele);
  //   }
  // //println!("---{}",json[0]);
  //println!("{}",run(json));


}