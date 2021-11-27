pub (in crate) mod fetch;
pub (in crate) mod text;
mod innerhtml;
mod tagnames;


impl crate::Dom {
    #[allow(dead_code)]
    pub (in crate) fn f(&self, tag_name: &str, attr_name: &str, attr_value: &str) -> crate::Dom {

        #[allow(unused_assignments)]
        let mut new = crate::Dom::new();

        if tag_name == "" && attr_name == "" && attr_value == "" {
             return self.clone();
        }


        if !self.is_parsed {
            new=crate::parse_html(&self.to_string()).unwrap();
        } else {
            new=self.clone();
        }
        
        if tag_name != "" && tag_name != "*" {
            new=new.tag(tag_name);
        }

        if attr_name != "" && attr_name != "*"  {
            new=new.attr(attr_name);
        }

        if attr_value != "" && attr_value != "*"  {
            new=new.attr_value(attr_value);
        }

        new.is_parsed=false;
        new
    }

    pub (in crate::parse) fn new() -> crate::Dom {
        let tag = crate::Tag{tagname: "".to_string(), tagcontent: "".to_string(), innerhtml: "".to_string()};
        let tags = vec![tag];
        crate::Dom{tag: tags, is_parsed: false}
    }

}
