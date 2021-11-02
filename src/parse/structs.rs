
impl crate::Dom {
    pub (in crate::parse) fn new() -> crate::Dom {
        let tag = crate::Tag{tagname: "".to_string(), tagcontent: "".to_string(), innerhtml: "".to_string()};
        let tags = vec![tag];
        crate::Dom{tag: tags, is_parsed: false}
    }
}