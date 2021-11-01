#[derive(Clone)]
pub struct Dom {
    pub tag: Vec<Tag>,
    pub (in crate::parse) is_parsed: bool,
}

#[derive(Clone)]
pub struct Tag {
    pub (in crate::parse) tagname: String,
    pub (in crate::parse) tagcontent: String,
    pub (in crate::parse) innerhtml: String,
}

impl Dom {
    pub (in crate::parse) fn new() -> Dom {
        let tag = Tag{tagname: "".to_string(), tagcontent: "".to_string(), innerhtml: "".to_string()};
        let tags = vec![tag];
        Dom{tag: tags, is_parsed: false}
    }
}