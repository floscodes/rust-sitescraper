use crate::parse::{fetch, innerhtml};
use slicestring::Slice;

pub (super) fn get(tagname: &str, mut innerhtml: String) -> String {

    if !innerhtml.contains(">") {
        return innerhtml;
    }
    if !innerhtml.contains("<") {
        return innerhtml;
    }

    let checkbreak = innerhtml::check_break(tagname, &innerhtml);

    if checkbreak {
        innerhtml=innerhtml.replace("<br>", innerhtml::BREAK_SUBST);
    }

    let firstpart = innerhtml.slice(0, innerhtml.find("<").unwrap());
    let mut middle = innerhtml.slice(innerhtml.find("<").unwrap(), innerhtml.rfind(">").unwrap()+1);
    let lastpart = innerhtml.slice(innerhtml.rfind(">").unwrap()+1, innerhtml.len());

    let appearance = middle.matches("<").count();

    let mut parts1: Vec<String> = vec![];

    for _ in 0..appearance-1 {

        let (tagname, _ ) = fetch::get_tagname_and_content(&middle);
        if fetch::check_tagname(&tagname) {
            break
        } else {
            middle=middle.slice(1, middle.len());
            if !middle.contains("<") {
                break
            }
            parts1.push(format!("<{}", middle.slice(0, middle.find("<").unwrap())));
            middle=middle.slice(middle.find("<").unwrap(), middle.len());
        }

    }

    let mut parts2: Vec<String> = vec![];

    loop {

        if !middle.contains("<") {
            break
        }

        let part = middle.slice(middle.rfind("<").unwrap(), middle.len());

        let (mut tagname, _) = fetch::get_tagname_and_content(&part);

        if tagname.contains("/") {
            tagname=tagname.replace( "/", "");
        }
        if fetch::check_tagname(&tagname) {
            break
        }

        parts2.push(part.clone());
        middle=middle.slice(0, middle.rfind(&part).unwrap());
    }

    
    //run filter for middle
    let middle_new = get_middle(middle.to_string());

    //continue with sorting parts
    let mut parts2sorted: Vec<&str> = vec![];
    let mut x: i32 = parts2.len()as i32 - 1;

    loop {
        if x < 0 {
			break
		}

        parts2sorted.push(&parts2[x as usize]);

        x=x-1;
    }

    let mut out = format!("{} {} {} {} {}", firstpart, parts1.concat(), middle_new, parts2sorted.concat(), lastpart);

    if checkbreak {
        out=out.replace(innerhtml::BREAK_SUBST, "");
        let br: &[_] = &['<', 'b', 'r', '>'];
        out=out.trim_matches(br).to_string();
    }

    out.trim().to_string()

}

fn get_middle(middle: String) -> String {

    if !middle.contains("<") {
        return "".to_string();
    }

    let mut out = middle.clone();

    let dm = fetch::fetch(out.to_string());


    for t in dm.tag {
        let mut closing_tag = String::new();
        let mut closing_tag_in = false;

        if middle.contains(&format!("</{}", t.tagname)) {
            closing_tag=middle.slice(middle.find(&format!("</{}", t.tagname)).unwrap(), middle.len());
            closing_tag=closing_tag.to_string().slice(0, closing_tag.find(">").unwrap()+1);
            closing_tag_in=true;
        }

        out=out.replace(&t.tagcontent, "");
        if closing_tag_in {
            out=out.replace(&closing_tag, "");
        }
    }

    out.trim().to_string()
}