use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    tags::TagType,
};

use super::field::InputFieldConfig;

/// Internal function. Creates a `<label>` + `<input type="checkbox">`
///
/// - `id` : The checkbox's id. Used for linking the label and the input.
/// - `value` : The checkbox's value.
/// - `name` : The checkbox's name.
/// - `label` : The label's content.
/// - `label_config` : The label's html config.
/// - `field_config` : The input's html config.
/// - `checked` : Indicates if the checkbox is checked.
fn _create_labeled_checkbox(
    id: String,
    value: String,
    name: String,
    label: String,
    label_config: HtmlElementConfig,
    field_config: HtmlElementConfig,
    checked: bool,
) -> Element {
    let label = Element::Element(HtmlElement::new(
        TagType::Label,
        label_config.set_attribute("for".to_string(), Some(id.clone())),
    )) + Element::Text(label);

    let mut cfg = field_config
        .clone()
        .set_attribute("type".to_string(), Some("checkbox".to_string()))
        .set_attribute("value".to_string(), Some(value))
        .set_attribute("name".to_string(), Some(name.clone()))
        .set_attribute("id".to_string(), Some(id.clone()));

    if checked {
        cfg = cfg.set_attribute("checked".to_string(), None);
    }
    Element::Element(HtmlElement::new(
        TagType::Div,
        HtmlElementConfig::new_empty(),
    )) + label
        + Element::Element(HtmlElement::new(TagType::Input, cfg))
}

/// Creates a `<label>` + `<input type="checkbox">`
///
/// - `field_config` : The label + input html configs.
/// - `value` : The checkbox's value.
/// - `checked` : Indicates if the checkbox is checked.
pub fn create_labeled_checkbox(
    field_config: &InputFieldConfig,
    value: String,
    checked: bool,
) -> Element {
    _create_labeled_checkbox(
        field_config.id.clone(),
        value,
        field_config.name.clone(),
        field_config.label.clone(),
        field_config.label_config.clone(),
        field_config.field_config.clone(),
        checked,
    )
}

/// Creates `<input type="checkbox">`s with associated `<label>`s.
///
/// - `field_config` : The label + input html configs.
/// - `checkboxes` : A vector of tuples containing all label + checkbox details:
/// - `String` : The checkbox's id. Used for linking the label and the input.
/// - `String` : The label's content.
/// - `String` : The checkbox's value.
/// - `bool` : Indicates if the checkbox is checked.
pub fn create_labeled_checkboxes(
    field_config: InputFieldConfig,
    checkboxes: Vec<(String, String, String, bool)>,
) -> Element {
    let mut div = Element::Element(HtmlElement::new(
        TagType::Div,
        HtmlElementConfig::new_empty(),
    ));

    for (id, label, value, checked) in checkboxes.into_iter() {
        div += _create_labeled_checkbox(
            id,
            value,
            field_config.name.clone(),
            label,
            field_config.label_config.clone(),
            field_config.field_config.clone(),
            checked,
        )
    }

    div
}
