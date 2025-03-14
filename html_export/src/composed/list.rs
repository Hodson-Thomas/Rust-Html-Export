use std::collections::HashMap;

use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    tags::TagType,
};

/// The different list types.
pub enum ListType {
    Ordered,
    Unordered,
    TermDefinition,
}

/// Defines the behavior to convert an enity to a list item.
pub trait AsList {
    /// Converts the entity to a list item.
    ///
    /// `mode` : The list type
    /// - `ListType::Ordered` : It is recommanded to return an `<li>` html tag.
    /// - `ListType::Unordered` : It is recommanded to return an `<li>` html tag.
    /// - `ListType::TermDefinition` : It is recommanded to return `<dt>` and `<dd>` html tags.
    ///
    /// If the None variant is returned, the item is ignored.
    fn to_list_item(&self, mode: &ListType) -> Option<Element>;

    /// Converts the entity to a definition list item (`<dt>`, `<dd>`).
    /// If the None variant is returned, the item is ignored.
    fn to_definition_list_item(&self) -> Option<(Element, Element)>;
}

/// Converts a vector to a list.
///
/// `collection` : Vector of element to be converted into a list.
/// `mode` : The list type
/// `list_config` : The list tag html configs.
pub fn from_iterator<T>(
    collection: &Vec<T>,
    mode: &ListType,
    list_config: HtmlElementConfig,
) -> Option<Element>
where
    T: AsList,
{
    if collection.is_empty() {
        return None;
    }
    let mut list = match mode {
        ListType::Ordered => Element::Element(HtmlElement::new(TagType::Ol, list_config)),
        ListType::Unordered => Element::Element(HtmlElement::new(TagType::Ul, list_config)),
        ListType::TermDefinition => Element::Element(HtmlElement::new(TagType::Dl, list_config)),
    };
    for item in collection {
        match mode {
            ListType::Ordered | ListType::Unordered => {
                if let Some(list_item) = item.to_list_item(mode) {
                    list += list_item;
                }
            }
            ListType::TermDefinition => {
                if let Some((key, value)) = item.to_definition_list_item() {
                    list += Element::Element(HtmlElement::new(
                        TagType::Dt,
                        HtmlElementConfig::new_empty(),
                    )) + key;
                    list += Element::Element(HtmlElement::new(
                        TagType::Dd,
                        HtmlElementConfig::new_empty(),
                    )) + value;
                }
            }
        }
    }
    Some(list)
}

/// Converts a hashmap into a definition list.
///
/// `items` : The hashmap to be converted.
/// `list_config` : The list tag html configs.
///
/// It is recommanded to convert the hashmap keys to `<dt>` html tags and the hashmap's values to `<dd>` html tags.
pub fn term_definition_list_from_map<T, U>(
    items: &HashMap<T, U>,
    list_config: HtmlElementConfig,
) -> Option<Element>
where
    T: AsList,
    U: AsList,
{
    let mut list = Element::Element(HtmlElement::new(TagType::Dl, list_config));
    for (key, value) in items {
        match (
            key.to_list_item(&ListType::TermDefinition),
            value.to_list_item(&ListType::TermDefinition),
        ) {
            (Some(k), Some(v)) => {
                list += k;
                list += v;
            }
            _ => {}
        }
    }
    Some(list)
}
