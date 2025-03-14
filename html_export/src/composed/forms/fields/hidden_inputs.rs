use crate::element::Element;

use super::field::{create_input, InputFieldConfig, NoConfigs};

/// Creates a `<input type="hidden">`.
///
/// - `html_configs` : The input html configs.
/// - `value` : The input's optional value.
pub fn create_hidden_input(html_configs: InputFieldConfig, value: Option<String>) -> Element {
    create_input(html_configs, "hidden".to_string(), NoConfigs::new(), value)
}
