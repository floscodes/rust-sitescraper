[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)
# Scraping Websites! [![crates.io](https://img.shields.io/crates/v/sitescraper.svg)](https://crates.io/crates/sitescraper)


## Examples:

### Get InnerHTML:

```
let html = "<html><body><div>Hello World!</div></body></html>";
     
let dom = sitescraper::parse_html(html).unwrap();
     
let filtered_dom = dom.filter("body");
     
println!("{}", filtered_dom.get_inner_html());
//Output: <div>Hello World!</div>
```

### Get Text:
```
let html = "<html><body><div>Hello World!</div></body></html>";

let dom = sitescraper::parse_html(html).unwrap();

let filtered_dom = dom.filter("body");

println!("{}", filtered_dom.get_text());
//Output: Hello World!
```

### Get Text from single Tags:

```
use sitescraper;

let html = "<html><body><div>Hello World!</div></body></html>";

let dom = sitescraper::parse_html(html).unwrap();

let filtered_dom = dom.filter("div");

println!("{}", filtered_dom.tag[0].get_text());
//Output: Hello World!
```

**Works also with**
```
get_inner_html()
```

### Filter by tag-name, attribute-name and attribute-value using a tuple:

```
use sitescraper;
 
let html = "<html><body><div id='hello'>Hello World!</div></body></html>";
 
let dom = sitescraper::parse_html(html).unwrap();
 
let filtered_dom = dom.filter(("div", "id", "hello"));
 
println!("{}", filtered_dom.tag[0].get_text());
//Output: Hello World!
```

**Works also with a tuple consisting of two string literals**
```
let filtered_dom = dom.filter(("div", "id"));
```

### You can also leave arguments out by passing "*" or "":

```
use sitescraper;

let html = "<html><body><div id="hello">Hello World!</div></body></html>";

let dom = sitescraper::parse_html(html).unwrap();

let filtered_dom = dom.filter(("*", "id", "hello"));

println!("{}", filtered_dom.tag[0].get_text());
//Output: Hello World!
```

or

```
use sitescraper;

let html = "<html><body><div id="hello">Hello World!</div></body></html>";

let dom = sitescraper::parse_html(html).unwrap();

let filtered_dom = dom.filter(("", "", "hello"));

println!("{}", filtered_dom.tag[0].get_text());
//Output: Hello World!
```


### Get Website-Content:

```
use sitescraper;

let html = sitescraper::http::get("http://example.com/).await.unwrap();

let dom = sitescraper::parse_html(html).unwrap();

let filtered_dom = sitescraper::filter!(dom, "div");

println!("{}", filtered_dom.get_inner_html());

```
