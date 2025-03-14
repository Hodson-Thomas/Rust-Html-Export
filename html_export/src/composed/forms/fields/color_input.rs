use crate::element::Element;

use super::field::{create_input, create_labeled_input, InputFieldConfig, NoConfigs};

/// Creates a `<label>` + `<input type="color">` html structure.
///
/// - `html_configs` : The label + input html configs.
/// - `value` : The input's optional value.
pub fn create_labeled_color_input(
    html_configs: InputFieldConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "color".to_string(), NoConfigs::new(), value)
}

/// Creates a `<input type="color">`.
///
/// - `html_configs` : The input's html config.
/// - `value` : The input's optional value.
pub fn create_color_input(html_configs: InputFieldConfig, value: Option<String>) -> Element {
    create_input(html_configs, "color".to_string(), NoConfigs::new(), value)
}
