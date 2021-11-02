pub mod parse;
pub mod http;

use std::io::{Error, ErrorKind};

/// This method parses a &[`str`] to a [`Dom`].
/// It returns a [`Result`] that can be unwrapped to a [`Dom`] if the parsing-process was successful.
/// 
/// # Example
/// 
/// ```
/// use sitescraper;
/// 
/// let html = "<html><body><div>Hello World!</div></body></html>";
/// 
/// let dom = sitescraper::parse_html(html).unwrap();
/// ```
/// [`Dom`]: struct.Dom.html#
pub fn parse_html(html: &str) -> Result<Dom, Error> {

    if !html.contains("<") || !html.contains(">") {
        return Err(Error::new(ErrorKind::InvalidInput, "An error has occurred when trying to parse the html-string! (Invalid Input)"));
    }

    Ok(parse::fetch::fetch(html.to_string()))
}


#[allow(unused_macros)]
/// This macro filters a [`Dom`] by the given tag-name, attribute-name and attribute-value.
/// 
/// # Examples
/// 
/// ```
/// use sitescraper;
/// 
/// let html = "<html><body><div id="hello">Hello World!</div></body></html>";
/// 
/// let dom = sitescraper::parse_html(html);
/// 
/// let filtered_dom = sitescraper::filter!(dom, "div", "id", "hello");
/// ```
/// 
/// The first argument has to be a [`Dom`], the following arguments follow this order: tag-name (e.g. "div"), attribute-name, e.g. "id", attribute-value (e.g. "hello").
/// You can also just filter the [`Dom`] by tagname, e.g.
/// ```
/// let filtered_dom = sitescraper::filter!(dom, "div");
/// ```
/// or just filter it by tag-name and attribute-name, e.g.parse_html
/// ```
/// let filtered_dom = sitescraper::filter!(dom, "div", "id");
/// ```
/// You can also leave arguments out by typing
/// ```
/// let filtered_dom = sitescraper::filter!(dom, "", "id", "hello");
/// ```
/// or
/// ```
/// let filtered_dom = sitescraper::filter!(dom, "", "", "hello");
/// ```
/// 
/// or
/// ```
/// let filtered_dom = sitescraper::filter!(dom, "*", "*", "hello");
/// ```
/// 
/// A filtered [`Dom`] can be filtered again with this macro, e.g.
/// ```
/// let html = "<html><body><div id="hello">Hello World!</div></body></html>";
/// 
/// let dom = sitescraper::parse_html(html);
/// 
/// let filtered_dom = sitescraper::filter!(dom, "body");
/// 
/// let filtered_dom_2 = sitescraper::filter!(filtered_dom, "div");
/// ```
/// [`Dom`]: struct.Dom.html#
#[macro_export]
macro_rules! filter {
        () => {};

        ($dom: expr) => {$dom};

        ($dom: expr, $tag: expr) => {$dom.f($tag, "", "")};
        ($dom: expr, $tag: expr, $attr_name: expr) => {$dom.f($tag, $attr_name, "")};
        ($dom: expr, $tag: expr, $attr_name: expr, $attr_value: expr) => {$dom.f($tag, $attr_name, $attr_value)};
    }


/// A [`Dom`] is returned when a html-String ist parsed with [`parse_html`] that can be filtered with [`filter`]
/// [`filter`]: macro.filter.html#
/// [`parse_html`]: fn.parse_html.html#
#[derive(Clone)]
pub struct Dom {
    pub tag: Vec<Tag>,
    is_parsed: bool,
}

/// Many [`Tag`]s are part of a [`Dom`]
/// [`Tag`]: struct.Tag.html#
#[derive(Clone)]
pub struct Tag {
    tagname: String,
    tagcontent: String,
    innerhtml: String,
}


impl crate::Tag {

    /// Returns InnerHTML inside a [`Tag`] as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "body");
    /// 
    /// println!("{}", filtered_dom.tag[0].get_inner_html());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Tag`]: struct.Tag.html#
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
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.tag[0].get_tagname());
    /// //Output: div
    /// ```
    /// [`Tag`]: struct.Tag.html#
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
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.tag[0].get_text());
    /// //Output: Hello World!
    /// ```
    /// [`Tag`]: struct.Tag.html#
    pub fn get_text(&self) -> String {
        parse::text::get(&self.tagname, self.innerhtml.clone())
    }


    /// Returns the [`Tag`] and its contents as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.tag[0].to_string());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Tag`]: struct.Tag.html#
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
    /// let dom = sitescraper::parse_html(html).unwrap();
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


impl crate::Dom {
    #[allow(dead_code)]

    fn tag(&self, tagname: &str) -> crate::Dom {

        if tagname.len() < 1 {
            return self.clone();
        }

        let mut tags: Vec<crate::Tag> = vec![];

        for n in &self.tag {
            if n.tagname == tagname {
                tags.push(n.clone());
            }
        }

        crate::Dom{tag: tags, is_parsed: false}

    }
    #[allow(dead_code)]
    fn attr(&self, attr: &str) -> crate::Dom {

        if attr.len() < 1 {
            return self.clone();
        }

        let mut tags: Vec<crate::Tag> = vec![];

        for n in &self.tag {
            if n.tagcontent.contains(&format!(r#"{}=""#, attr)) || n.tagcontent.contains(&format!("{}=", attr)) {
                tags.push(n.clone());
            }
        }
        crate::Dom{tag: tags, is_parsed: false}
    }
    #[allow(dead_code)]
    fn attr_value(&self, attrvalue: &str) -> crate::Dom {

        if attrvalue.len() < 1 {
            return self.clone();
        }

        let mut tags: Vec<crate::Tag> = vec![];

        for n in &self.tag {
            if n.tagcontent.contains(&format!(r#"="{}""#, attrvalue)) || n.tagcontent.contains(&format!(r#"={} "#, attrvalue)) || n.tagcontent.contains(&format!(r#"={}>"#, attrvalue)) {
                tags.push(n.clone());
            }
        }

        crate::Dom{tag: tags, is_parsed: false}
    }


    /// Returns the [`Dom`] or a filtered [`Dom`] and its contents as a [`String`]
    /// 
    /// # Example
    /// ```
    /// use sitescraper;
    /// 
    /// let html = "<html><body><div>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.to_string());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Dom`]: struct.Dom.html#
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
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "body");
    /// 
    /// println!("{}", filtered_dom.get_inner_html());
    /// //Output: <div>Hello World!</div>
    /// ```
    /// [`Dom`]: struct.Dom.html#
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
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "body");
    /// 
    /// println!("{}", filtered_dom.get_text());
    /// //Output: Hello World!
    /// ```
    /// [`Dom`]: struct.Dom.html#
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
            s.push(parse::text::get(&self.tag[x].tagname, self.tag[x].innerhtml.clone()));
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
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = sitescraper::filter!(dom, "div");
    /// 
    /// println!("{}", filtered_dom.get_attr_value("id"));
    /// //Output: hello
    /// ```
    /// [`Dom`]: struct.Dom.html#
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

}