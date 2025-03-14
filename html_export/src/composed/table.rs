use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    tags::TagType,
};

/// Defines the behavior to convert the item to a table.
pub trait AsTable {
    /// Defines how to convert the item to the table header.
    /// If the function returns None, the generated table has no header.
    /// It is recommanded to return an `<thead>` element.
    fn as_table_head(&self) -> Option<Element>;

    /// Defines how to convert the item as a table row.
    /// If the function returns None, the generated table will ignore this item.
    fn as_table_row(&self) -> Option<Element>;

    /// Defines how to convert the item to the table footer.
    /// If the function returns None, the generated table has no footer.
    /// It is recommanded to return an `<tfoot>` element.
    fn as_table_foot(&self) -> Option<Element>;
}

/// Converts the given vector to a html table.
/// If the vector is empty, the function returns None.
/// The table's headear and footer are generated using the vector's first element.
/// The table's body is wrapped in a `<tbody>` html element.
pub fn from_iterator<T>(
    collection: &Vec<T>,
    table_config: HtmlElementConfig,
    table_body_config: HtmlElementConfig,
) -> Option<Element>
where
    T: AsTable,
{
    if collection.is_empty() {
        return None;
    }
    let mut table = Element::Element(HtmlElement::new(TagType::Table, table_config));
    let mut body = Element::Element(HtmlElement::new(TagType::Tbody, table_body_config));
    let head = collection.get(0).unwrap().as_table_head();
    let foot = collection.get(0).unwrap().as_table_foot();
    if let Some(head) = head {
        table += head;
    };
    for item in collection {
        if let Some(row) = item.as_table_row() {
            body += row;
        }
    }
    table += body;
    if let Some(foot) = foot {
        table += foot;
    };
    Some(table)
}
