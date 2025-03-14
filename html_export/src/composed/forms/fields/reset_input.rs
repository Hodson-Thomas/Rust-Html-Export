use crate::element::Element;

use super::field::{create_input, create_labeled_input, InputFieldConfig, NoConfigs};

/// Creates a `<label>` + `<input type="reset">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `value` : The input's optional initial value.
pub fn create_labeled_reset_input(
    html_configs: InputFieldConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "reset".to_string(), NoConfigs::new(), value)
}

/// Creates an `<input type="reset">`.
///
/// - `html_configs` : The input's html configs.
/// - `value` : The input's optional initial value.
pub fn create_reset_input(html_configs: InputFieldConfig, value: Option<String>) -> Element {
    create_input(html_configs, "reset".to_string(), NoConfigs::new(), value)
}
