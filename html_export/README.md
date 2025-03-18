# Export rust structures, enums and data structures to html

A simple rust crate to convert structure, enums, data structures into html files. 

This crate is designed to:

1. Be as simple as possible.
2. Simplify html generation
3. Export structure, enums and data structures to html.

This crate is not designed to:

1. Be implemented in frameworks
2. Be implemented in web applications


```rust
use html_export::{
    element::{Element, HtmlElement, HtmlElementConfig},
    html::ToHtml,
    tags::TagType,
};

pub struct Person {
    pub name: String,
    pub age: u8,
}

impl ToHtml for Person {
    fn to_html(&self) -> Element {
        let mut div = Element::Element(HtmlElement::new(
            TagType::Div,
            HtmlElementConfig::new_empty(),
        ));
        div += Element::Element(HtmlElement::new(TagType::P, HtmlElementConfig::new_empty()))
            + Element::Text(format!("{} is {} years old.", self.name.clone(), self.age));
        div
    }
}

fn main() {
    let person = Person {
        name: "Thomas Hodson".to_string(),
        age: 22,
    };
    let html = person.to_html();
}
```

Please check out the [Wiki](https://github.com/Hodson-Thomas/Rust-Html-Export/wiki).
