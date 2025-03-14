use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Represents the date input's configs.
///
/// - `max` : The optional maximum date.
/// - `min` : The optional minimum date.
/// - `step` : The optional counter step.
pub struct DateInputConfig {
    pub is_local_datetime: bool,
    pub max: Option<String>,
    pub min: Option<String>,
    pub step: Option<DateInputStep>,
}

/// The counter's step modes.
pub enum DateInputStep {
    /// No restrictions.
    Any,
    /// The step value.
    Value(String),
}

/// Creates a `<label>` + `<input type="date">` html structure.
///
/// - `html_configs` : The label + input html configs
/// - `input_configs` : The date input's config.
/// - `value` : The date input's value.
pub fn create_date_input_with_label(
    html_configs: InputFieldConfig,
    input_configs: DateInputConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(
        html_configs,
        if input_configs.is_local_datetime {
            "datetime-local".to_string()
        } else {
            "date".to_string()
        },
        input_configs,
        value,
    )
}

/// Creates an `<input type="date">`.
///
/// - `html_configs` : The label + input html configs
/// - `input_configs` : The date input's config.
/// - `value` : The date input's value.
pub fn create_date_input(
    html_configs: InputFieldConfig,
    input_configs: DateInputConfig,
    value: Option<String>,
) -> Element {
    create_input(
        html_configs,
        if input_configs.is_local_datetime {
            "datetime-local".to_string()
        } else {
            "date".to_string()
        },
        input_configs,
        value,
    )
}

impl DateInputConfig {
    /// creates a new
    pub fn new(is_local_datetime: bool) -> Self {
        Self {
            is_local_datetime,
            max: None,
            min: None,
            step: None,
        }
    }

    /// Sets the maximum value.
    ///
    /// - `max` : The maximum value.
    pub fn with_max(mut self, max: String) -> Self {
        self.max = Some(max);
        self
    }

    /// Removes the maximum value.
    pub fn without_max(mut self) -> Self {
        self.max = None;
        self
    }

    /// Sets the minimum value.
    ///
    /// - `min` : The minimum value.
    pub fn with_min(mut self, min: String) -> Self {
        self.min = Some(min);
        self
    }

    /// Removes the minimum value.
    pub fn without_min(mut self) -> Self {
        self.min = None;
        self
    }

    /// Sets the step value.
    ///
    /// - `step` : The step value.
    pub fn with_step(mut self, step: DateInputStep) -> Self {
        self.step = Some(step);
        self
    }

    /// Removes the step value.
    pub fn without_step(mut self) -> Self {
        self.step = None;
        self
    }
}

impl AsHtmlConfig for DateInputConfig {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.max.is_some() {
            configs = configs.set_attribute("max".to_string(), self.max.clone());
        }
        if self.min.is_some() {
            configs = configs.set_attribute("min".to_string(), self.min.clone());
        }
        match &self.step {
            Some(DateInputStep::Any) => {
                configs = configs.set_attribute("step".to_string(), Some("any".to_string()))
            }
            Some(DateInputStep::Value(value)) => {
                configs = configs.set_attribute("step".to_string(), Some(value.to_string()))
            }
            None => {}
        }
        configs
    }
}

impl Default for DateInputConfig {
    fn default() -> Self {
        Self {
            is_local_datetime: false,
            max: None,
            min: None,
            step: None,
        }
    }
}
