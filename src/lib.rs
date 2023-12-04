//! # Scraping Websites in Rust!
//!
//! Sitescraper is a libary for the scraping and extraction of website content.
//! You can easily parse html doms and extract data.
//! 
//! See examples below:
//!
//! ### Get InnerHTML:
//!
//! ```
//! let html = "<html><body><div>Hello World!</div></body></html>";
//!    
//! let dom = sitescraper::parse_html(html).unwrap();
//!     
//! let filtered_dom = dom.filter("body");
//!      
//! println!("{}", filtered_dom.get_inner_html());
//! //Output: <div>Hello World!</div>
//! ```
//! 
//! ### Get Text:
//! ```
//! let html = "<html><body><div>Hello World!</div></body></html>";
//!
//! let dom = sitescraper::parse_html(html).unwrap();
//! 
//! let filtered_dom = dom.filter("body");
//! 
//! println!("{}", filtered_dom.get_text());
//! //Output: Hello World!
//! ```
//! ** Make sure to enable loop unrolling to avoid possible slow code execution! **
//!
//! ### Get Text from single Tags:
//!
//! ```
//! use sitescraper;
//!
//! let html = "<html><body><div>Hello World!</div></body></html>";
//! 
//! let dom = sitescraper::parse_html(html).unwrap();
//! 
//! let filtered_dom = dom.filter("div");
//! 
//! println!("{}", filtered_dom.tag[0].get_text());
//! //Output: Hello World!
//! ```
//! 
//! **Works also with**
//! ```
//! get_inner_html()
//! ```
//! 
//! ### Filter by tag-name, attribute-name and attribute-value using a tuple:
//!
//! ```
//! use sitescraper;
//! 
//! let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
//! 
//! let dom = sitescraper::parse_html(html).unwrap();
//! 
//! let filtered_dom = dom.filter(("div", "id", "hello"));
//! 
//! println!("{}", filtered_dom.tag[0].get_text());
//! //Output: Hello World!
//! ```
//! 
//! You can also filter only by attribute value by writing the following:
//! 
//! ```
//! use sitescraper;
//! 
//! let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
//! 
//! let dom = sitescraper::parse_html(html).unwrap();
//! 
//! let filtered_dom = dom.filter(("", "", "hello"));
//! 
//! println!("{}", filtered_dom.tag[0].get_text());
//! //Output: Hello World!
//! ```
//! ** Check out more examples how to use the [`filter`] method **
//! 
//! ### Get Website-Content:
//! 
//! ```
//! use sitescraper;
//! 
//! let html = sitescraper::http::get("http://example.com/).await.unwrap();
//! 
//! let dom = sitescraper::parse_html(html).unwrap();
//! 
//! let filtered_dom = dom.filter("div");
//! 
//! println!("{}", filtered_dom.get_inner_html());
//! 
//! ```
//! 
//! [`filter`]: struct.Dom.html#method.filter

pub (in crate) mod parse;
pub mod http;

use std::io::{Error, ErrorKind};
use parse::Args;

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


/// A [`Dom`] is returned when a html-String ist parsed with [`parse_html`] that can be filtered with [`filter`]
#[derive(Clone)]
pub struct Dom {
    pub tag: Vec<Tag>,
    is_parsed: bool,
}

impl crate::Dom {
#[allow(dead_code)]
/// This method can filter a [`Dom`] by the given tag-name, attribute-name and attribute-value.
/// 
/// # Examples
/// 
/// To filter the dom just by a tag-name, pass the tag-name as an argument.
/// 
/// ```
/// use sitescraper;
/// 
/// let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
/// 
/// let dom = sitescraper::parse_html(html).unwrap();
/// 
/// let filtered_dom = dom.filter("div");
/// ```
/// if you want to filter it by tag-name and attribute-name, you can pass a tuple:
/// ```
/// let filtered_dom = dom.filter(("div", "id"));
/// ```
/// If you want to filter a dom by an attribute-value as well, you can do the following:
/// 
/// ```
/// let filtered_dom = dom.filter(("div, "id, "hello"));
/// ````
/// 
/// If you only want to filter by attribute-name and attribute-value, you can just write:
/// ```
/// let filtered_dom = dom.filter(("", "id", "hello"));
/// ```
/// or
/// ```
/// let filtered_dom = dom.filter(("", "", "hello"));
/// ```
/// 
/// or
/// ```
/// let filtered_dom = dom.filter(("*", "*", "hello"));
/// ```
/// 
/// A filtered [`Dom`] can be filtered again with this method, e.g.
/// ```
/// let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
/// 
/// let dom = sitescraper::parse_html(html).unwrap();
/// 
/// let filtered_dom = dom.filter("body");
/// 
/// let filtered_dom_2 = dom.filter("div");
/// ```
/// [`Dom`]: struct.Dom.html#
    pub fn filter(&self, args: impl Args) -> crate::Dom {

        let (tag_name, attr_name, attr_value) = args.extract();

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

    fn new() -> Dom {
        let tag = crate::Tag{tagname: "".to_string(), tagcontent: "".to_string(), innerhtml: "".to_string()};
        let tags = vec![tag];
        crate::Dom{tag: tags, is_parsed: false}
    }

}

/// Many [`Tag`]s are part of a [`Dom`]
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
    /// let filtered_dom = dom.filter("body");
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
    /// let filtered_dom = dom.filter("div");
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
    /// let filtered_dom = dom.filter("div");
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
    /// let filtered_dom = dom.filter("div");
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
    /// let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
    /// 
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = dom.filter("div");
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
            } else if out.chars().nth(0).unwrap() == '\''{
                out=out[1..].to_string();
                out=out[..out.find('\'').unwrap()].to_string();
            } else {
                match out.find(" ") {
                    Some(v) => out=out[..v].to_string(),
                    None => out=out[..out.len()-1].to_string()
                }
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
    /// let filtered_dom = dom.filter("div");
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
    /// let filtered_dom = dom.filter("body");
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
    /// let filtered_dom = dom.filter("body");
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
    /// let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
    ///
    /// let dom = sitescraper::parse_html(html).unwrap();
    /// 
    /// let filtered_dom = dom.filter("div");
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

// Test filter method
#[test]
fn test_filter_method() {
    let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
    
    let dom = parse_html(html).unwrap();
    
    let filtered_dom = dom.filter("div");
    let filtered_dom2 = dom.filter("body");
    let filtered_dom3 = dom.filter(("div", "id"));
    let filtered_dom4 = dom.filter(("div", "id"));

    assert_eq!(filtered_dom.tag[0].get_attr_value("id"), "hello");
    assert_eq!(filtered_dom.tag[0].get_inner_html(), "Hello World!");
    assert_eq!(filtered_dom2.tag[0].get_text(), "Hello World!");
    assert_eq!(filtered_dom3.tag[0].get_text(), "Hello World!");
    assert_eq!(filtered_dom4.get_inner_html(), "Hello World!");

    let html = r#"
    <!doctype html>
    <html>
    <head>
        <title>Example Domain</title>
    
        <meta charset="utf-8" />
        <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <style type="text/css">
        body {
            background-color: #f0f0f2;
            margin: 0;
            padding: 0;
            font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", "Open Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
            
        }
        div {
            width: 600px;
            margin: 5em auto;
            padding: 2em;
            background-color: #fdfdff;
            border-radius: 0.5em;
            box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
        }
        a:link, a:visited {
            color: #38488f;
            text-decoration: none;
        }
        @media (max-width: 700px) {
            div {
                margin: 0 auto;
                width: auto;
            }
        }
        </style>    
    </head>
    
    <body>
    <div>
        <h1>Example Domain</h1>
        <p>This domain is for use in illustrative examples in documents. You may use this
        domain in literature without prior coordination or asking for permission.</p>
        <p><a href="https://www.iana.org/domains/example">More information...</a></p>
    </div>
    </body>
    </html>
    "#;

    let dom = parse_html(html).unwrap();
    let filtered_dom = dom.filter("h1");
        assert_eq!(filtered_dom.tag[0].get_text(), "Example Domain");

}