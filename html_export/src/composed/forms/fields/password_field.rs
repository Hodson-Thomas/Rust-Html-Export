use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Defines the password input configs.
///
/// - `max_length` : The input's maximum length. If the value is None, this attribute is ignored.
/// - `min_length` : The input's minimum length. If the value is None, this attribute is ignored.
/// - `pattern` : The input's regex pattern for content validation. If the value is None, this attribute is ignored.
/// - `place_holder` : The input's place holder text. If the value is None, this attribute is ignored.
/// - `read_only` : Indicates if the input is in read only mode. If the value is None, this attribute is ignored.
/// - `size` : The input's content display size. If the value is None, this attribute is ignored.
pub struct PasswordInputConfig {
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub pattern: Option<String>,
    pub place_holder: Option<String>,
    pub read_only: Option<bool>,
    pub size: Option<usize>,
}

/// Creates a `<label>` + `<input type="password">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_labeled_password_input(
    html_configs: InputFieldConfig,
    input_configs: PasswordInputConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "password".to_string(), input_configs, value)
}

/// Creates an `<input type="password">`.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_password_input(
    html_configs: InputFieldConfig,
    input_configs: PasswordInputConfig,
    value: Option<String>,
) -> Element {
    create_input(html_configs, "password".to_string(), input_configs, value)
}

impl PasswordInputConfig {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
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

    /// Sets the new pattern parameter.
    ///
    /// - `pattern` : The new pattern parameter.
    pub fn with_pattern(mut self, pattern: String) -> Self {
        self.pattern = Some(pattern);
        self
    }

    /// Removes the pattern parameter.
    pub fn without_pattern(mut self) -> Self {
        self.pattern = None;
        self
    }

    /// Sets the new place holder parameter.
    ///
    /// - `place_holder` : The new place holder parameter.
    pub fn with_place_holder(mut self, place_holder: String) -> Self {
        self.place_holder = Some(place_holder);
        self
    }

    /// Removes the place holder parameter.
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

impl AsHtmlConfig for PasswordInputConfig {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.place_holder.is_some() {
            configs = configs.set_attribute("placeholder".to_string(), self.place_holder.clone());
        }
        if self.pattern.is_some() {
            configs = configs.set_attribute("pattern".to_string(), self.pattern.clone());
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
        if let Some(read_only) = self.read_only {
            if read_only {
                configs = configs.set_attribute("readonly".to_string(), None);
            }
        }
        configs
    }
}

impl Default for PasswordInputConfig {
    fn default() -> Self {
        Self {
            max_length: None,
            min_length: None,
            pattern: None,
            place_holder: None,
            read_only: None,
            size: None,
        }
    }
}
