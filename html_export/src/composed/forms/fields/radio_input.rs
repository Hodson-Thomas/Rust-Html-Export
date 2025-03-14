use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    errors::FieldCreationError,
    tags::TagType,
};

use super::field::InputFieldConfig;

/// Creates a `<label>` and `<input type="radio">`.
///
/// - `field_config` : The label and radio input html configs.
///     - sub filed `field_config.id` is ignored .
///     - sub filed `field_config.label` is ignored
/// - `id` : The input's id (used for linking the label and the input).
/// - `label` : The label's content.
/// - `value` : The radio input's value.
/// - `checked` : Indicates if the radio is checked.
pub fn create_labeled_radio(
    field_config: &InputFieldConfig,
    id: String,
    label: String,
    value: String,
    checked: bool,
) -> Element {
    let label = Element::Element(HtmlElement::new(
        TagType::Label,
        field_config
            .label_config
            .clone()
            .set_attribute("for".to_string(), Some(id.clone())),
    )) + Element::Text(label);

    let mut cfg = field_config
        .field_config
        .clone()
        .set_attribute("type".to_string(), Some("radio".to_string()))
        .set_attribute("value".to_string(), Some(value))
        .set_attribute("name".to_string(), Some(field_config.name.clone()))
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

/// Creates `<label>`s and `<input type="radio">`s.
///
/// - `field_config` : The labels and radios html configs.
/// - `checked` : The index of the checked radio.If the index is None, it is ignored. If the index is invalid, the function returns an Error.
/// - `radios` : Vector of triplets of all the radios to be created containing
///     - the radio id (used for linking the radio input and the label).
///     - The label's content.
///     - The radio input's value.
pub fn create_labeled_radios(
    field_config: InputFieldConfig,
    checked: Option<usize>,
    radios: Vec<(String, String, String)>,
) -> Result<Element, FieldCreationError> {
    if let Some(check) = checked {
        if check >= radios.len() {
            return Err(FieldCreationError::SetCheckedInvalidIndex(
                check,
                radios.len(),
            ));
        }
    }
    let mut div = Element::Element(HtmlElement::new(
        TagType::Div,
        HtmlElementConfig::new_empty(),
    ));
    for (index, (id, label, value)) in radios.into_iter().enumerate() {
        if let Some(check) = checked {
            div += create_labeled_radio(&field_config, id, label, value, check == index);
        } else {
            div += create_labeled_radio(&field_config, id, label, value, false);
        }
    }
    Ok(div)
}
