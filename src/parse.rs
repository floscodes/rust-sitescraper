pub mod structs;
pub (in crate) mod fetch;
mod text;
mod innerhtml;
mod tagnames;

impl structs::Tag {

    /// Returns InnerHTML inside a [`Tag`] as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "body");
    /// 
    /// println!("{}", filtered_dom.tag[0].get_inner_html());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Tag`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Tag.html#
    pub fn get_inner_html(&self) -> String {
        self.innerhtml.clone()
    }

    /// Returns the name of the [`Tag`] as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.tag[0].get_tagname());
    /// //Output: div
    /// ```
    /// [`Tag`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Tag.html#
    pub fn get_tagname(&self) -> String {
        self.tagname.clone()
    }

    /// Returns pure text inside a [`Tag`] as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.tag[0].get_text());
    /// //Output: Hello World!
    /// ```
    /// [`Tag`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Tag.html#
    pub fn get_text(&self) -> String {
        text::get(&self.tagname, self.innerhtml.clone())
    }


    /// Returns the [`Tag`] and its contents as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.tag[0].to_string());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Tag`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Tag.html#
    pub fn to_string(&self) -> String {
        format!("{}{}</{}>", self.tagcontent, self.innerhtml, self.tagname)
    }


    /// Returns the value of the given attribute
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div id="hello">Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.tag[0].get_attr_value("id"));
    /// //Output: hello
    /// ```
    pub fn get_attr_value(&self, attr: &str) -> String {

        let mut out = String::new();
        
        if self.tagcontent.contains(&format!("{}=", attr)) {
            out = self.tagcontent[self.tagcontent.find(&format!("{}=", attr)).unwrap()+format!("{}=", attr).len()..].to_string();

            if out.chars().nth(0).unwrap() == '"' {
                out=out[1..].to_string();
                out=out[..out.find('"').unwrap()].to_string();
            } else {
                out=out[..out.find(" ").unwrap()].to_string();
            }
        }

        out
    }

}


impl structs::Dom {
    #[allow(dead_code)]

    fn tag(&self, tagname: &str) -> structs::Dom {

        if tagname.len() < 1 {
            return self.clone();
        }

        let mut tags: Vec<structs::Tag> = vec![];

        for n in &self.tag {
            if n.tagname == tagname {
                tags.push(n.clone());
            }
        }

        structs::Dom{tag: tags, is_parsed: false}

    }
    #[allow(dead_code)]
    fn attr(&self, attr: &str) -> structs::Dom {

        if attr.len() < 1 {
            return self.clone();
        }

        let mut tags: Vec<structs::Tag> = vec![];

        for n in &self.tag {
            if n.tagcontent.contains(&format!(r#"{}=""#, attr)) || n.tagcontent.contains(&format!("{}=", attr)) {
                tags.push(n.clone());
            }
        }
        structs::Dom{tag: tags, is_parsed: false}
    }
    #[allow(dead_code)]
    fn attr_value(&self, attrvalue: &str) -> structs::Dom {

        if attrvalue.len() < 1 {
            return self.clone();
        }

        let mut tags: Vec<structs::Tag> = vec![];

        for n in &self.tag {
            if n.tagcontent.contains(&format!(r#"="{}""#, attrvalue)) || n.tagcontent.contains(&format!(r#"={} "#, attrvalue)) || n.tagcontent.contains(&format!(r#"={}>"#, attrvalue)) {
                tags.push(n.clone());
            }
        }

        structs::Dom{tag: tags, is_parsed: false}
    }


    /// Returns the [`Dom`] or a filtered [`Dom`] and its contents as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.to_string());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Dom`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Dom.html#
    pub fn to_string(&self) -> String {

        if self.is_parsed {
            let mut x = 0;
            loop {
                if self.tag[x].tagname != "" && self.tag[x].tagname != " " {
                    return self.tag[x].to_string();
                }
            x=x+1;
            }
        }

        let mut s: Vec<String> = vec![];

        for x in 0..self.tag.len() as usize {

        if &self.tag[x].tagname != "" && &self.tag[x].tagname != " " {
            s.push(self.tag[x].to_string());
        }

    
        }

        let mut cleared: Vec<String> = vec![];

        for old in s {
            let mut exists = false;
            for new in &cleared {
                if &old==new {
                    exists=true;
                }
            }

            if !exists {
                cleared.push(old);
            }
        }
       
        cleared.concat()
    }



    /// Returns InnerHTML inside a [`Dom`] or a filtered [`Dom`] as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "body");
    /// 
    /// println!("{}", filtered_dom.get_inner_html());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Dom`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Dom.html#
    pub fn get_inner_html(&self) -> String {

        if self.is_parsed {
            let mut x = 0;
            loop {
                if self.tag[x].tagname != "" && self.tag[x].tagname != " " {
                    return self.tag[x].get_inner_html();
                }
            x=x+1;
            }
        }

        let mut s: Vec<String> = vec![];

        for x in 0..self.tag.len() as usize {

        if &self.tag[x].tagname != "" && &self.tag[x].tagname != " " {
            s.push(self.tag[x].get_inner_html());
        }

    
        }

        let mut cleared: Vec<String> = vec![];

        for old in s {
            let mut exists = false;
            for new in &cleared {
                if &old==new {
                    exists=true;
                }
            }

            if !exists {
                cleared.push(old);
            }
        }
       
        cleared.concat()
    }



    /// Returns pure text inside a [`Dom`] or a filtered [`Dom`] as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "body");
    /// 
    /// println!("{}", filtered_dom.get_text());
    /// //Output: Hello World!
    /// ```
    /// [`Dom`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Dom.html#
    pub fn get_text(&self) -> String {

        if self.is_parsed {
            let mut x = 0;
            loop {
                if self.tag[x].tagname != "" && self.tag[x].tagname != " " {
                    return self.tag[x].get_text();
                }
            x=x+1;
            }
        }

        let mut s: Vec<String> = vec![];

        for x in 0..self.tag.len() as usize {

        if &self.tag[x].tagname != "" && &self.tag[x].tagname != " " {
            s.push(text::get(&self.tag[x].tagname, self.tag[x].innerhtml.clone()));
        }

    
        }

        let mut cleared: Vec<String> = vec![];

        for old in s {
            let mut exists = false;
            for new in &cleared {
                if &old==new {
                    exists=true;
                }
            }

            if !exists {
                cleared.push(old);
            }
        }
       
        cleared.concat()
    }



    /// Returns the value(s) of the given attribute as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div id="hello">Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html);
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.get_attr_value("id"));
    /// //Output: hello
    /// ```
    /// [`Dom`]: https://docs.rs/sitescraper/0.1.1/sitescraper/parse/structs/struct.Dom.html#
    pub fn get_attr_value(&self, attrname: &str) -> String {

        let mut s: Vec<String> = vec![];

        for tag in &self.tag {
            s.push(tag.get_attr_value(attrname).clone());
        }

        let mut cleared: Vec<String> = vec![];

        for y in s {
            let mut exists = false;
            for x in &cleared {
                if &y == x {
                    exists=true;
                }
            }

            if !exists {
                cleared.push(y);
            }
        }

        cleared.concat()
    }

    #[allow(dead_code)]
    pub (in crate) fn f(&self, tag_name: &str, attr_name: &str, attr_value: &str) -> structs::Dom {

        #[allow(unused_assignments)]
        let mut new = structs::Dom::new();

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

}
