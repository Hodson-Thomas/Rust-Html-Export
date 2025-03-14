use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Defines the time input's configs.
///
/// - `list` : The `<datalist>` tag reference. If the value is None, this attribute is ignored.
/// - `max` : The input's maximum value. If the value is None, this attribute is ignored.
/// - `min` : The input's minimum value. If the value is None, this attribute is ignored.
/// - `read_only` : Indicates if the input is in read only mode. If the value is None, this attribute is ignored.
pub struct TimeInputConfigs {
    pub list: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub read_only: Option<bool>,
}

/// Creates a `<label>` + `<input type="time">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_labeled_time_input(
    html_configs: InputFieldConfig,
    input_configs: TimeInputConfigs,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "time".to_string(), input_configs, value)
}

/// Creates an `<input type="time">`.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_time_input(
    html_configs: InputFieldConfig,
    input_configs: TimeInputConfigs,
    value: Option<String>,
) -> Element {
    create_input(html_configs, "time".to_string(), input_configs, value)
}

impl TimeInputConfigs {
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

    /// Sets the new max parameter.
    ///
    /// - `max` : The new max parameter.
    pub fn with_max(mut self, max: String) -> Self {
        self.max = Some(max);
        self
    }

    /// Removes the max parameter.
    pub fn without_max(mut self) -> Self {
        self.max = None;
        self
    }

    /// Sets the new min parameter.
    ///
    /// - `min` : The new min parameter.
    pub fn with_min(mut self, min: String) -> Self {
        self.min = Some(min);
        self
    }

    /// Removes the min parameter.
    pub fn without_min(mut self) -> Self {
        self.min = None;
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
}

impl AsHtmlConfig for TimeInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.list.is_some() {
            configs = configs.set_attribute("list".to_string(), self.list.clone());
        }
        if let Some(min) = &self.min {
            configs = configs.set_attribute("min".to_string(), Some(min.to_string()));
        }
        if let Some(max) = &self.max {
            configs = configs.set_attribute("max".to_string(), Some(max.to_string()));
        }
        if let Some(read_only) = self.read_only {
            if read_only {
                configs = configs.set_attribute("readonly".to_string(), None);
            }
        }

        configs
    }
}

impl Default for TimeInputConfigs {
    fn default() -> Self {
        Self {
            list: None,
            max: None,
            min: None,
            read_only: None,
        }
    }
}
