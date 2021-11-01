# Scraping Websites in Rust!


## Examples:

### Get InnerHTML:

```
let html = "<html><body><div>Hello World!</div></body></html>";
     
let dom = sitescraper::parse_html(html);
     
let filtered_dom = sitescraper::filter!(dom, "body");
     
println!("{}", filtered_dom.get_inner_html());
//Output: <div>Hello World!</div>
```

### Get Text:
```
let html = "<html><body><div>Hello World!</div></body></html>";

let dom = sitescraper::parse_html(html);

let filtered_dom = sitescraper::filter!(dom, "body");

println!("{}", filtered_dom.get_text());
//Output: Hello World!
```

### Get Text from single Tags:

```
use sitescraper;

let html = "<html><body><div>Hello World!</div></body></html>";

let dom = sitescraper::parse_html(html);

let filtered_dom = sitescraper::filter!(dom, "div");

println!("{}", filtered_dom.tag[0].get_text());
//Output: Hello World!
```

**Works also with GetInnerHTML()**



### Get Website-Content:

```
use sitescraper;

let html = sitescraper::http::get("http://example.com/).unwrap();

let dom = sitescraper::parse_html(html);

let filtered_dom = sitescraper::filter!(dom, "div");

println!("{}", filtered_dom.get_inner_html());

```