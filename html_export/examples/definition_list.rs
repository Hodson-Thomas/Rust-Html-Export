use std::collections::HashMap;

use html_export::composed::list::from_iterator;
use html_export::composed::list::term_definition_list_from_map;
use html_export::composed::list::AsList;
use html_export::composed::list::ListType;
use html_export::dd;
use html_export::dt;
use html_export::elem;
use html_export::element::*;
use html_export::export_to_file;
use html_export::head::Head;
use html_export::tags::*;

pub struct ProgrammingLanguage {
    name: String,
    published_year: u32,
}

impl ProgrammingLanguage {
    pub fn new(name: String, published_year: u32) -> Self {
        Self {
            name,
            published_year,
        }
    }
}

impl AsList for ProgrammingLanguage {
    fn to_list_item(&self, _mode: &ListType) -> Option<Element> {
        None
    }

    fn to_definition_list_item(&self) -> Option<(Element, Element)> {
        Some((
            Element::Text(self.name.clone()),
            Element::Text(self.published_year.to_string()),
        ))
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Country(String);
pub struct Population(u64);

impl AsList for Country {
    fn to_list_item(&self, mode: &ListType) -> Option<Element> {
        match mode {
            ListType::Ordered | ListType::Unordered => None,
            ListType::TermDefinition => Some(dt!() + Element::Text(self.0.clone())),
        }
    }

    fn to_definition_list_item(&self) -> Option<(Element, Element)> {
        None
    }
}

impl AsList for Population {
    fn to_list_item(&self, mode: &ListType) -> Option<Element> {
        match mode {
            ListType::Ordered | ListType::Unordered => None,
            ListType::TermDefinition => Some(dd!() + Element::Text(self.0.to_string())),
        }
    }

    fn to_definition_list_item(&self) -> Option<(Element, Element)> {
        None
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert(Country("India".to_string()), Population(1_450_935_791));
    map.insert(Country("China".to_string()), Population(1_419_321_278));
    map.insert(Country("Usa".to_string()), Population(345_426_571));
    map.insert(Country("Indonesia".to_string()), Population(283_487_931));

    let languages = vec![
        ProgrammingLanguage::new("Rust".to_string(), 2006),
        ProgrammingLanguage::new("C".to_string(), 1972),
        ProgrammingLanguage::new("Python".to_string(), 1991),
        ProgrammingLanguage::new("Haskell".to_string(), 1990),
    ];

    let head = Head::new().with_title("Definition list".to_string());

    export_to_file(
        "examples_output".to_string(),
        "definition_list.html".to_string(),
        head,
        vec![
            term_definition_list_from_map(&map, HtmlElementConfig::new_empty()).unwrap(),
            from_iterator(
                &languages,
                &ListType::TermDefinition,
                HtmlElementConfig::new_empty(),
            )
            .unwrap(),
        ],
    )
    .unwrap();
}
