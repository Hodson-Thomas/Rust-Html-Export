use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Defines the url input's configs.
///
/// - `list` : The `<datalist>` tag reference. If the value is None, this attribute is ignored.
/// - `pattern` : The input's regex validation pattern. If the value is None, this attribute is ignored.
/// - `max_length` : The input's content max length. If the value is None, this attribute is ignored.
/// - `min_length` : The input's content MIN length. If the value is None, this attribute is ignored.
/// - `size` : The input's content display size. If the value is None, this attribute is ignored.
/// - `read_only` : Indicates if the input is in read only mode. If the value is None, this attribute is ignored.
/// - `spell_check` : Indicates if the input spell check is active. If the value is None, this attribute is ignored.
pub struct UrlInputConfigs {
    pub list: Option<String>,
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub size: Option<usize>,
    pub read_only: Option<bool>,
    pub spell_check: Option<bool>,
}

/// Creates a `<label>` + `<input type="url">` html structure.
///
/// - `html_configs` : The label + input html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_labeled_url_input(
    html_configs: InputFieldConfig,
    input_configs: UrlInputConfigs,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "url".to_string(), input_configs, value)
}

/// Creates a `<input type="url">` html structure.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_url_input(
    html_configs: InputFieldConfig,
    input_configs: UrlInputConfigs,
    value: Option<String>,
) -> Element {
    create_input(html_configs, "url".to_string(), input_configs, value)
}

impl UrlInputConfigs {
    /// Creates a new default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a new list parameter.
    ///
    /// - `list` The new list parameter.
    pub fn with_list(mut self, list: String) -> Self {
        self.list = Some(list);
        self
    }

    /// Removes the list parameter.
    pub fn without_list(mut self) -> Self {
        self.list = None;
        self
    }

    /// Sets a new pattern parameter.
    ///
    /// - `pattern` The new pattern parameter.
    pub fn with_pattern(mut self, pattern: String) -> Self {
        self.pattern = Some(pattern);
        self
    }

    /// Removes the pattern parameter.
    pub fn without_pattern(mut self) -> Self {
        self.pattern = None;
        self
    }

    /// Sets a new min length parameter.
    ///
    /// - `min_length` The new min length parameter.
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Removes the min length parameter.
    pub fn without_min_length(mut self) -> Self {
        self.min_length = None;
        self
    }

    /// Sets a new max length parameter.
    ///
    /// - `max_length` The new max length parameter.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Removes the max length parameter.
    pub fn without_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }

    /// Sets a new size parameter.
    ///
    /// - `size` The new size parameter.
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Removes the size parameter.
    pub fn without_size(mut self) -> Self {
        self.size = None;
        self
    }

    /// Sets a new read only parameter state.
    ///
    /// - `read_only` The new read only parameter state.
    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }

    /// Removes the read only parameter.
    pub fn without_read_only(mut self) -> Self {
        self.read_only = None;
        self
    }

    /// Sets a new spell check parameter state.
    ///
    /// - `spell_check` The new spell check parameter state.
    pub fn with_spell_check(mut self, spell_check: bool) -> Self {
        self.spell_check = Some(spell_check);
        self
    }

    /// Removes the spell check parameter.
    pub fn without_spell_check(mut self) -> Self {
        self.spell_check = None;
        self
    }
}

impl AsHtmlConfig for UrlInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.list.is_some() {
            configs = configs.set_attribute("list".to_string(), self.list.clone());
        }
        if self.pattern.is_some() {
            configs = configs.set_attribute("pattern".to_string(), self.pattern.clone());
        }
        if let Some(min_length) = &self.min_length {
            configs = configs.set_attribute("minlength".to_string(), Some(min_length.to_string()));
        }
        if let Some(max_length) = &self.max_length {
            configs = configs.set_attribute("maxlength".to_string(), Some(max_length.to_string()));
        }
        if let Some(size) = &self.size {
            configs = configs.set_attribute("size".to_string(), Some(size.to_string()));
        }
        if let Some(read_only) = self.read_only {
            if read_only {
                configs = configs.set_attribute("readonly".to_string(), None);
            }
        }
        if let Some(spell_check) = self.spell_check {
            if spell_check {
                configs = configs.set_attribute("spellcheck".to_string(), None);
            }
        }
        configs
    }
}

impl Default for UrlInputConfigs {
    fn default() -> Self {
        Self {
            list: None,
            pattern: None,
            min_length: None,
            max_length: None,
            size: None,
            read_only: None,
            spell_check: None,
        }
    }
}
