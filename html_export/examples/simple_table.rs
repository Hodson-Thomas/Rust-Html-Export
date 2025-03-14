use html_export::composed::table::from_iterator;
use html_export::composed::table::AsTable;
use html_export::elem;
use html_export::element::*;
use html_export::head::Head;
use html_export::tags::*;
use html_export::thead;
use html_export::*;
extern crate html_export;

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

impl AsTable for Person {
    fn as_table_head(&self) -> Option<html_export::element::Element> {
        let mut head = thead!();
        let mut row = tr!();
        row += th!() + text!("Name");
        row += th!() + text!("Age");
        row += th!() + text!("Country");
        head += row;
        Some(head)
    }

    fn as_table_row(&self) -> Option<html_export::element::Element> {
        let mut row = tr!();
        row += td!() + text!(self.name.clone());
        row += td!() + text!(self.age.to_string());
        row += td!() + text!(self.country.clone());
        Some(row)
    }

    fn as_table_foot(&self) -> Option<html_export::element::Element> {
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
    let table = from_iterator(
        &persons,
        HtmlElementConfig::new_empty(),
        HtmlElementConfig::new_empty(),
    )
    .unwrap();
    let head = Head::new().with_title("Simple table".to_string());
    export_to_file(
        "examples_output".to_string(),
        "simple_table.html".to_string(),
        head,
        vec![table],
    )
    .unwrap();
}
