pub mod parse;
pub mod http;

use std::io::{Error, ErrorKind};

/// This method parses a &str to a [`Dom`].
/// It returns a [`Result`] that can be unwrapped to a [`Dom`] if the parsing-process was successful.
/// 
/// # Example
/// 
/// ```
/// use sitescraper;
/// 
/// let html = "<html><body><div>Hello World!</div></body></html>";
/// 
/// let dom = sitescraper::parse_html(html);
/// ```
/// [`Dom`]: https://docs.rs/sitescraper/0.1.5/sitescraper/parse/structs/struct.Dom.html#
pub fn parse_html(html: &str) -> Result<parse::structs::Dom, Error> {

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
/// [`Dom`]: https://docs.rs/sitescraper/0.1.5/sitescraper/parse/structs/struct.Dom.html#
#[macro_export]
macro_rules! filter {
        () => {};

        ($dom: expr) => {$dom};

        ($dom: expr, $tag: expr) => {$dom.f($tag, "", "")};
        ($dom: expr, $tag: expr, $attr_name: expr) => {$dom.f($tag, $attr_name, "")};
        ($dom: expr, $tag: expr, $attr_name: expr, $attr_value: expr) => {$dom.f($tag, $attr_name, $attr_value)};
    }





