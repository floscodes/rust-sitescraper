use crate::parse::{tagnames, innerhtml};
use slicestring::Slice;

pub fn fetch(html: String) -> crate::Dom  {

    let mut dm = crate::Dom::new();
    let mut html_string = html.clone();
    
    loop {

        if !html_string.contains("<") {
            break
        }

        let (tagname, tagcontent) = get_tagname_and_content(&html_string);

        html_string=html_string[html_string.find(&tagcontent).unwrap()+tagcontent.len()..].to_string();

        if check_tagname(&tagname) {
            dm.tag.push(crate::Tag{tagname: tagname.clone(), tagcontent: tagcontent, innerhtml: innerhtml::get(&tagname, html_string.to_string()).replace(innerhtml::BREAK_SUBST, "<br>")});
        }
    }
    
    dm=clear_closing_tags(dm);

    dm.is_parsed=true;

    dm

}

fn clear_closing_tags(d: crate::Dom) -> crate::Dom {
    let mut new = crate::Dom::new();
    for i in 0..d.tag.len() {
        if !d.tag[i].tagcontent.contains("</") {
            new.tag.push(d.tag[i].clone());
        }
     }
     new
}

pub (super) fn get_tagname_and_content(html: &str) -> (String, String)  {

    let mut tagcontent = html.to_string().slice(html.find("<").unwrap(), html.len());

    tagcontent = tagcontent.slice(0, tagcontent.find(">").unwrap()+1);

    let mut tagname = String::new();

    if tagcontent.contains(" ") {
        tagname = tagcontent.slice(1, tagcontent.find(" ").unwrap()).to_string();
    } else {
        tagname = match tagcontent.find(">") {
            Some(v) => tagcontent.slice(1, v).to_string(),
            None => tagname
        };
    }

    (tagname, tagcontent)

}

pub (super) fn check_tagname(tagname: &str) -> bool {

    for n in tagnames::TAGNAMES {

        if tagname == n {
            return true;
        }

    }

    false
}

