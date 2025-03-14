use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Defines all the email input configs.
///
/// - `list` : The `<datalist>` tag reference. If the value is None, this attribute is ignored.
/// - `max_length` : The input's content max length. If the value is None, this attribute is ignored.
/// - `min_length` : The input's content MIN length. If the value is None, this attribute is ignored.
/// - `multiple` : Indicates if the input can contains multiple emails. If the value is None, this attribute is ignored.
/// - `pattern` : The input's regex validation pattern. If the value is None, this attribute is ignored.
/// - `placeholder` : The input's placeholder text. If the value is None, this attribute is ignored.
/// - `readonly` : Indicates if the input is in readonly mode. If the value is None, this attribute is ignored.
/// - `size` : The input's content display size. If the value is None, this attribute is ignored.
pub struct EmailInputConfig {
    pub list: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub multiple: Option<bool>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: Option<bool>,
    pub size: Option<usize>,
}

/// Creates a `<label>` + `<input type="email">` html structure.
///
/// - `html_configs` : The label + input html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_labeled_email_input(
    html_configs: InputFieldConfig,
    input_configs: EmailInputConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "email".to_string(), input_configs, value)
}

/// Creates a `<input type="email">` html structure.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_email_input(
    html_configs: InputFieldConfig,
    input_configs: EmailInputConfig,
    value: Option<String>,
) -> Element {
    create_input(html_configs, "email".to_string(), input_configs, value)
}

impl EmailInputConfig {
    /// Creates a default email input configs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new list parameter.
    ///
    /// - `list` : The `<datalist>` tag reference.
    pub fn with_list(mut self, list: String) -> Self {
        self.list = Some(list);
        self
    }

    /// Removes the list parameter.
    pub fn without_list(mut self) -> Self {
        self.list = None;
        self
    }

    /// Sets the new max length parameter.
    ///
    /// - `max_length` : The new max length parameter.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Removes the max length parameter.
    pub fn without_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }

    /// Sets the new min length parameter.
    ///
    /// - `min_length` : The new min length parameter.
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Removes the min length parameter.
    pub fn without_min_length(mut self) -> Self {
        self.min_length = None;
        self
    }

    /// Sets the new multiple parameter state.
    ///
    /// - `state` : The new multiple parameter state.
    pub fn with_multiple(mut self, state: bool) -> Self {
        self.multiple = Some(state);
        self
    }

    /// Removes the multiple parameter.
    pub fn without_multiple(mut self) -> Self {
        self.multiple = None;
        self
    }

    /// Sets the new regex pattern for field validation.
    ///
    /// - `pattern` : The new regex pattern. Note that this pattern must follow the regex html standarts.
    pub fn with_pattern(mut self, pattern: String) -> Self {
        self.pattern = Some(pattern);
        self
    }

    /// Removes the regex pattern parameter.
    pub fn without_pattern(mut self) -> Self {
        self.pattern = None;
        self
    }

    /// Sets the new placeholder text.
    ///
    /// - `placeholder` : The new placeholder text.
    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Removes the regex pattern parameter.
    pub fn without_placeholder(mut self) -> Self {
        self.placeholder = None;
        self
    }

    /// Sets the new readonly state.
    ///
    /// - `state` : The new readonly state.
    pub fn with_readonly(mut self, state: bool) -> Self {
        self.readonly = Some(state);
        self
    }

    /// Removes the readonly parameter.
    pub fn without_readonly(mut self) -> Self {
        self.readonly = None;
        self
    }

    /// Sets the new size parameter.
    ///
    /// - `size` : The new size parameter.
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Removes the size parameter.
    pub fn without_size(mut self) -> Self {
        self.size = None;
        self
    }
}

impl AsHtmlConfig for EmailInputConfig {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.list.is_some() {
            configs = configs.set_attribute("list".to_string(), self.list.clone());
        }
        if let Some(max_length) = self.max_length {
            configs = configs.set_attribute("maxlength".to_string(), Some(max_length.to_string()));
        }
        if let Some(min_length) = self.min_length {
            configs = configs.set_attribute("minlength".to_string(), Some(min_length.to_string()));
        }
        if let Some(size) = self.size {
            configs = configs.set_attribute("size".to_string(), Some(size.to_string()));
        }
        if let Some(pattern) = &self.pattern {
            configs = configs.set_attribute("pattern".to_string(), Some(pattern.clone()));
        }
        if let Some(placeholder) = &self.placeholder {
            configs = configs.set_attribute("placeholder".to_string(), Some(placeholder.clone()));
        }
        if let Some(multiple) = self.multiple {
            if multiple {
                configs = configs.set_attribute("multiple".to_string(), None);
            }
        }
        if let Some(readonly) = self.readonly {
            if readonly {
                configs = configs.set_attribute("readonly".to_string(), None);
            }
        }

        configs
    }
}

impl Default for EmailInputConfig {
    fn default() -> Self {
        Self {
            list: None,
            max_length: None,
            min_length: None,
            multiple: None,
            pattern: None,
            placeholder: None,
            readonly: None,
            size: None,
        }
    }
}
