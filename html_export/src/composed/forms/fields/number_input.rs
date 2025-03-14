use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Defines the number input's configs.
///
/// - `list` : The `<datalist>` tag reference. If the value is None, this attribute is ignored.
/// - `min` : The input's minimum value. If the value is None, this attribute is ignored.
/// - `max` : The input's maximum value. If the value is None, this attribute is ignored.
/// - `place_holder` : The input's place holder text. If the value is None, this attribute is ignored.
/// - `read_only` : Indicates if the input is in read only mode. If the value is None, this attribute is ignored.
/// - `step` : The step value for the input's counter. If the value is None, this attribute is ignored.
pub struct NumberInputConfigs {
    pub list: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub place_holder: Option<String>,
    pub read_only: Option<bool>,
    pub step: Option<f64>,
}

/// Creates a `<label>` + `<input type="number">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `input_configs` : The input's configs.
/// - `value`  : The input's optional initial value.
pub fn create_labeled_number_input(
    html_configs: InputFieldConfig,
    input_configs: NumberInputConfigs,
    value: Option<f64>,
) -> Element {
    let value = match value {
        Some(val) => Some(val.to_string()),
        None => None,
    };
    create_labeled_input(html_configs, "number".to_string(), input_configs, value)
}

/// Creates an `<input type="number">`.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value`  : The input's optional initial value.
pub fn create_number_input(
    html_configs: InputFieldConfig,
    input_configs: NumberInputConfigs,
    value: Option<f64>,
) -> Element {
    let value = match value {
        Some(val) => Some(val.to_string()),
        None => None,
    };
    create_input(html_configs, "number".to_string(), input_configs, value)
}

impl NumberInputConfigs {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new list parameter.
    ///
    /// - `list` : The new list parameter.
    pub fn with_list(mut self, list: String) -> Self {
        self.list = Some(list);
        self
    }

    /// Removes the list parameter.
    pub fn without_list(mut self) -> Self {
        self.list = None;
        self
    }

    /// Sets the new min parameter.
    ///
    /// - `min` : The new min parameter.
    pub fn with_min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Removes the min parameter.
    pub fn without_min(mut self) -> Self {
        self.min = None;
        self
    }

    /// Sets the new max parameter.
    ///
    /// - `max` : The new max parameter.
    pub fn with_max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Removes the max parameter.
    pub fn without_max(mut self) -> Self {
        self.max = None;
        self
    }

    /// Sets the new place holder parameter.
    ///
    /// - `place_holder` : The new place holder parameter.
    pub fn with_place_holder(mut self, place_holder: String) -> Self {
        self.place_holder = Some(place_holder);
        self
    }

    /// Removes the place_holder parameter.
    pub fn without_place_holder(mut self) -> Self {
        self.place_holder = None;
        self
    }

    /// Sets the new read only parameter state.
    ///
    /// - `read_only` : The new read only parameter state.
    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }

    /// Removes the read only parameter.
    pub fn without_read_only(mut self) -> Self {
        self.read_only = None;
        self
    }

    /// Sets the new step parameter.
    ///
    /// - `step` : The new step parameter.
    pub fn with_step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    /// Removes the step parameter.
    pub fn without_step(mut self) -> Self {
        self.step = None;
        self
    }
}

impl AsHtmlConfig for NumberInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.list.is_some() {
            configs = configs.set_attribute("list".to_string(), self.list.clone());
        }
        if self.place_holder.is_some() {
            configs = configs.set_attribute("placeholder".to_string(), self.place_holder.clone());
        }
        if let Some(max) = self.max {
            configs = configs.set_attribute("max".to_string(), Some(max.to_string()));
        }
        if let Some(min) = self.min {
            configs = configs.set_attribute("min".to_string(), Some(min.to_string()));
        }
        if let Some(step) = self.step {
            configs = configs.set_attribute("step".to_string(), Some(step.to_string()));
        }
        if let Some(read_only) = self.read_only {
            if read_only {
                configs =
                    configs.set_attribute("readonly".to_string(), Some(read_only.to_string()));
            }
        }
        configs
    }
}

impl Default for NumberInputConfigs {
    fn default() -> Self {
        Self {
            list: None,
            min: None,
            max: None,
            place_holder: None,
            read_only: None,
            step: None,
        }
    }
}
