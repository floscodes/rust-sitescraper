use slicestring::Slice;

pub (in crate::parse) fn get(tagname: &str, mut html: String) -> String {

    if !html.contains(&format!("</{}", tagname)) {
        return html;
    }

    let mut closing_tag = html.slice(html.find(&format!("</{}", tagname)).unwrap(), html.len()).to_string();
    closing_tag = closing_tag.slice(0, closing_tag.find(">").unwrap()+1);
    

    let mut firstpart = html.slice(0, html.find(&closing_tag).unwrap()+closing_tag.len()).to_string().clone();

    if check_break(tagname, &firstpart) {
        firstpart = firstpart.replace("<br>", BREAK_SUBST);
    }

    let appearance = firstpart.matches(&format!("<{}", tagname)).count();

    if appearance < 1 {
        match firstpart.rfind(&closing_tag) {
            Some(_) => firstpart=firstpart.slice(0, firstpart.rfind(&closing_tag).unwrap()).to_string(),
            None => ()
        }
        return firstpart;
    }

    //Cut away firstpart

    html=html.slice(html.find(&closing_tag).unwrap()+closing_tag.len(), html.len()).to_string();

    let mut secondparts: Vec<String> = vec![];
    let mut x = 0;

    loop {
        if x > appearance {
            break
        }
        if html.len() < 1 {
            break
        }
        if !html.contains(&format!("</{}", tagname)) {
            break
        }

        closing_tag=html.slice(html.find(&format!("</{}", tagname)).unwrap(), html.len()).to_string();
        closing_tag=closing_tag.slice(0, closing_tag.find(">").unwrap()+1).to_string();
        secondparts.push(html.slice(0, html.find(&closing_tag).unwrap()+closing_tag.len()).to_string());
        html=html.slice(html.find(&closing_tag).unwrap()+closing_tag.len(), html.len()).to_string();

        x=x+1;
    }

    //firstpart added
    let mut out = format!("{}{}", firstpart, secondparts.concat());

    out=out.replace(BREAK_SUBST, "<br>");

    match out.rfind(&closing_tag) {
        Some(_) => {
            closing_tag=out.slice(out.find(&format!("</{}", tagname)).unwrap(), out.len()).to_string();
            closing_tag=closing_tag.slice(0, closing_tag.find(">").unwrap()+1).to_string();
            out=out.slice(0, out.rfind(&closing_tag).unwrap()).to_string();
        },

        None => ()
    }
    
    out
}


pub (in crate::parse) fn check_break(tagname: &str, firstpart: &str) -> bool {
    
    if tagname == "b" {
        if firstpart.contains("<br") {
            return true;
        }
    }
    false
}

pub (in crate::parse) const BREAK_SUBST: &str = "{|231!sdhjsd73487sd[|}";