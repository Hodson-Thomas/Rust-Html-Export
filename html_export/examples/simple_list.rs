use html_export::{
    composed::list::{from_iterator, AsList, ListType},
    export_to_file,
    head::Head,
    li,
};

use html_export::elem;
use html_export::element::*;
use html_export::tags::*;

pub struct Person {
    name: String,
    age: u8,
    country: String,
}

impl Person {
    pub fn new(name: String, age: u8, country: String) -> Self {
        Self { name, age, country }
    }
}

impl AsList for Person {
    fn to_list_item(
        &self,
        mode: &html_export::composed::list::ListType,
    ) -> Option<html_export::element::Element> {
        match mode {
            ListType::Ordered | ListType::Unordered => Some(
                li!()
                    + Element::Text(format!(
                        "Name: {}, Age: {}, Country: {}",
                        self.name, self.age, self.country
                    )),
            ),
            ListType::TermDefinition => None,
        }
    }

    fn to_definition_list_item(&self) -> Option<(Element, Element)> {
        None
    }
}

fn main() {
    let persons = vec![
        Person::new("Johns James".to_string(), 20, "France".to_string()),
        Person::new("Smith Jennifer".to_string(), 30, "England".to_string()),
        Person::new("Brown Robert".to_string(), 40, "Australia".to_string()),
        Person::new("Taylor Mary".to_string(), 50, "South Africa".to_string()),
    ];
    let ul_list = from_iterator(
        &persons,
        &ListType::Unordered,
        HtmlElementConfig::new_empty(),
    )
    .unwrap();

    let ol_list =
        from_iterator(&persons, &ListType::Ordered, HtmlElementConfig::new_empty()).unwrap();
    let head = Head::new().with_title("Simple list".to_string());
    export_to_file(
        "examples_output".to_string(),
        "simple_list.html".to_string(),
        head,
        vec![ul_list, ol_list],
    )
    .unwrap();
}
