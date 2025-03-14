use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// The range input configs.
///
/// - `max` : The range max value.
/// - `min` : The range min value.
/// - `step` : The range step value.
pub struct RangeInputConfigs {
    pub max: f64,
    pub min: f64,
    pub step: Option<f64>,
}

/// Creates a `<label>` + `<input type="range">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_labeled_range_input(
    html_configs: InputFieldConfig,
    input_configs: RangeInputConfigs,
    value: Option<f64>,
) -> Element {
    let value = match value {
        Some(val) => Some(val.to_string()),
        None => None,
    };
    create_labeled_input(html_configs, "range".to_string(), input_configs, value)
}

/// Creates an `<input type="range">`.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_range_input(
    html_configs: InputFieldConfig,
    input_configs: RangeInputConfigs,
    value: Option<f64>,
) -> Element {
    let value = match value {
        Some(val) => Some(val.to_string()),
        None => None,
    };
    create_input(html_configs, "range".to_string(), input_configs, value)
}

impl RangeInputConfigs {
    /// Creates a new object.
    ///
    /// - `min` : The range input's minimum value.
    /// - `max` : The range input's maximum value.
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            max,
            min,
            ..Self::default()
        }
    }

    /// Sets the new max parameter.
    ///
    /// - `max` : The new max parameter.
    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Sets the new min parameter.
    ///
    /// - `min` : The new min parameter.
    pub fn with_min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    /// Sets the step parameter.
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

impl AsHtmlConfig for RangeInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        configs = configs.set_attribute("min".to_string(), Some(self.min.to_string()));
        configs = configs.set_attribute("max".to_string(), Some(self.max.to_string()));
        if let Some(step) = self.step {
            configs = configs.set_attribute("max".to_string(), Some(step.to_string()));
        }
        configs
    }
}

impl Default for RangeInputConfigs {
    fn default() -> Self {
        Self {
            max: 1.0,
            min: 0.0,
            step: None,
        }
    }
}
