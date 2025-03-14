use html_export::composed::table::from_iterator;
use html_export::composed::table::AsTable;
use html_export::elem;
use html_export::element::*;
use html_export::head::Head;
use html_export::tags::*;
use html_export::thead;
use html_export::*;
extern crate html_export;

pub enum Item {
    BasicItem(BasicItem),
    DiscountedItem(DiscountedItem),
}

pub struct BasicItem {
    name: String,
    unit_price: f64,
    quantity: u64,
}

pub struct DiscountedItem {
    name: String,
    unit_price: f64,
    quantity: u64,
    unit_discount: f64,
}

impl BasicItem {
    pub fn new(name: String, unit_price: f64, quantity: u64) -> Self {
        Self {
            name,
            unit_price,
            quantity,
        }
    }

    pub fn as_row(&self) -> Option<Element> {
        let mut row = tr!();
        row += td!() + Element::Text(self.name.clone());
        row +=
            td!(attributes = {"colspan" => Some("2")}) + Element::Text(self.unit_price.to_string());
        row += td!() + Element::Text(self.quantity.to_string());
        row += td!(attributes = {"colspan" => Some("3")})
            + Element::Text((self.unit_price * (self.quantity as f64)).to_string());
        Some(row)
    }
}

impl DiscountedItem {
    pub fn new(name: String, unit_price: f64, quantity: u64, unit_discount: f64) -> Self {
        Self {
            name,
            unit_price,
            quantity,
            unit_discount,
        }
    }
    pub fn as_row(&self) -> Option<Element> {
        let mut row = tr!();
        row += td!() + Element::Text(self.name.clone());
        row += td!() + Element::Text(self.unit_price.to_string());
        row += td!() + Element::Text(self.unit_discount.to_string());
        row += td!() + Element::Text(self.quantity.to_string());
        row += td!() + Element::Text((self.unit_price * (self.quantity as f64)).to_string());
        row += td!() + Element::Text((self.unit_discount * (self.quantity as f64)).to_string());
        row += td!()
            + Element::Text(
                (self.unit_price * (self.quantity as f64)
                    - self.unit_discount * (self.quantity as f64))
                    .to_string(),
            );
        Some(row)
    }
}

impl AsTable for Item {
    fn as_table_head(&self) -> Option<html_export::element::Element> {
        let heading =
            tr!() + (th!(attributes = {"colspan" => Some("7")}) + (h2!() + text!("Items")));
        let mut sub_heading = tr!();
        let mut sub_sub_heading = tr!();
        sub_heading += th!(attributes = {"rowspan" => Some("2")}) + text!("Name");
        sub_heading += th!(attributes = {"colspan" => Some("2")}) + text!("Unitary");
        sub_heading += th!(attributes = {"rowspan" => Some("2")}) + text!("Quantity");
        sub_heading += th!(attributes = {"colspan" => Some("3")}) + text!("Totals");
        sub_sub_heading += th!() + text!("Unit price");
        sub_sub_heading += th!() + text!("Unit discount");
        sub_sub_heading += th!() + text!("Price");
        sub_sub_heading += th!() + text!("Discount");
        sub_sub_heading += th!() + text!("Result");

        Some(thead!() + heading + sub_heading + sub_sub_heading)
    }

    fn as_table_row(&self) -> Option<html_export::element::Element> {
        match self {
            Item::BasicItem(basic_item) => basic_item.as_row(),
            Item::DiscountedItem(discounted_item) => discounted_item.as_row(),
        }
    }

    fn as_table_foot(&self) -> Option<html_export::element::Element> {
        None
    }
}

fn main() {
    let items = vec![
        Item::BasicItem(BasicItem::new("Item A".to_string(), 10.0, 3)),
        Item::BasicItem(BasicItem::new("Item B".to_string(), 11.0, 2)),
        Item::DiscountedItem(DiscountedItem::new("Item C".to_string(), 20.0, 5, 5.0)),
        Item::DiscountedItem(DiscountedItem::new("Item D".to_string(), 5.0, 1, 10.0)),
        Item::BasicItem(BasicItem::new("Item E".to_string(), 100.0, 1)),
    ];
    let table = from_iterator(
        &items,
        HtmlElementConfig::new_empty(),
        HtmlElementConfig::new_empty(),
    )
    .unwrap();
    let head = Head::new().with_title("Union table".to_string());
    export_to_file(
        "examples_output".to_string(),
        "union_tables.html".to_string(),
        head,
        vec![table],
    )
    .unwrap();
}
