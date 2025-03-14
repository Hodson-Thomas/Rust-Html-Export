use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Defines the month input configs.
///
/// - `read_only` : Indicates if the input is in read only mode. If the value is None, the attribute is ignored.
/// - `list` : The `<datalist>` tag reference. If the value is None, this attribute is ignored.
/// - `step` : The input counter step value. If the value is None, this attribute is ignored.
/// - `min` : The input minimum value. If the value is None, this attribute is ignored.
/// - `max` : The input maximum value. If the value is None, this attribute is ignored.
pub struct MonthInputConfig {
    pub read_only: Option<bool>,
    pub list: Option<String>,
    pub step: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
}

/// Creates a `<label>` + `<input type="month">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_labeled_mount_input(
    html_configs: InputFieldConfig,
    input_configs: MonthInputConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "month".to_string(), input_configs, value)
}

/// Creates an `<input type="month">`
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_mount_input(
    html_configs: InputFieldConfig,
    input_configs: MonthInputConfig,
    value: Option<String>,
) -> Element {
    create_input(html_configs, "month".to_string(), input_configs, value)
}

impl MonthInputConfig {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
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

    /// Sets the new step parameter.
    ///
    /// - `step` : The new step parameter.
    pub fn with_step(mut self, step: String) -> Self {
        self.step = Some(step);
        self
    }

    /// Removes the step parameter.
    pub fn without_step(mut self) -> Self {
        self.step = None;
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
}

impl AsHtmlConfig for MonthInputConfig {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.list.is_some() {
            configs = configs.set_attribute("list".to_string(), self.list.clone());
        }
        if self.step.is_some() {
            configs = configs.set_attribute("step".to_string(), self.step.clone());
        }
        if self.min.is_some() {
            configs = configs.set_attribute("min".to_string(), self.min.clone());
        }
        if self.max.is_some() {
            configs = configs.set_attribute("max".to_string(), self.max.clone());
        }

        if let Some(read_only) = self.read_only {
            if read_only {
                configs = configs.set_attribute("readonly".to_string(), None);
            }
        }
        configs
    }
}

impl Default for MonthInputConfig {
    fn default() -> Self {
        Self {
            read_only: None,
            list: None,
            step: None,
            min: None,
            max: None,
        }
    }
}
