use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, AsHtmlConfig, InputFieldConfig};

/// Represents an `<input type="button">` configs.
///
/// - `value` : The input's value. If the value is None, this parameter is ignored.
pub struct ButtonInputConfig {
    pub value: Option<String>,
}

/// Creates an `<input type="button">` html element.
///
/// - `input_configs` : The field's configs.
/// - `html_configs` : The field's html configs.
pub fn create_button_input(
    input_configs: ButtonInputConfig,
    html_configs: InputFieldConfig,
) -> Element {
    create_input(html_configs, "button".to_string(), input_configs, None)
}

impl AsHtmlConfig for ButtonInputConfig {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if let Some(value) = &self.value {
            configs = configs.set_attribute("value".to_string(), Some(value.clone()));
        }
        configs
    }
}

impl Default for ButtonInputConfig {
    fn default() -> Self {
        Self { value: None }
    }
}
