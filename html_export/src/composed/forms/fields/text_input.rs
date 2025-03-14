use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Represents an `<input type="text">` configs.
///
/// - `max_length` : The input's max length. If the value is None, this parameter is ignored.
/// - `min_length` : The input's min length. If the value is None, this parameter is ignored.
/// - `list` : The input's associated `<datalist>` id. If the value is None, this parameter is ignored.
/// - `place_holder` : The helper text displayed in the input. If the value is None, this parameter is ignored.
/// - `pattern` : The regex pattern for the input validation. If the value is None, this parameter is ignored.
/// - `size` : The number of characters displayed in the input. If the value is None, this parameter is ignored.
/// - `required` : Indicates if the input is required.
/// - `read_only` : Indicates if the input is read only.
/// - `spell_check` : Indicates if spell checking is activated.
pub struct TextInputConfig {
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub list: Option<String>,
    pub place_holder: Option<String>,
    pub pattern: Option<String>,
    pub size: Option<u64>,
    pub required: bool,
    pub read_only: bool,
    pub spell_check: bool,
}

/// Creates a `<label>` + `<input type="text">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_label_text_field(
    html_configs: InputFieldConfig,
    input_configs: TextInputConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "text".to_string(), input_configs, value)
}

/// Creates an `<input type="text">`.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_text_field(
    html_configs: InputFieldConfig,
    input_configs: TextInputConfig,
    value: Option<String>,
) -> Element {
    create_input(html_configs, "text".to_string(), input_configs, value)
}

/// Creates a `<label>` + `<input type="search">` html structure.
///
/// - `html_configs` : The input and label html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_label_search_field(
    html_configs: InputFieldConfig,
    input_configs: TextInputConfig,
    value: Option<String>,
) -> Element {
    create_labeled_input(html_configs, "search".to_string(), input_configs, value)
}

/// Creates an `<input type="search">`.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value.
pub fn create_search_field(
    html_configs: InputFieldConfig,
    input_configs: TextInputConfig,
    value: Option<String>,
) -> Element {
    create_input(html_configs, "search".to_string(), input_configs, value)
}

impl TextInputConfig {
    /// Creates a new text input configs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the field's max length attribute.
    ///
    /// - `max_length` : The new field's max length.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Clears the field's max length attribute.
    pub fn without_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }

    /// Sets the field's min length attribute.
    ///
    /// - `min_length` : The new field's min length.
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Clears the field's min length attribute.
    pub fn without_min_length(mut self) -> Self {
        self.min_length = None;
        self
    }

    /// Sets the field's list attribute.
    ///
    /// - `list` : The new field's list.
    pub fn with_list(mut self, list: String) -> Self {
        self.list = Some(list);
        self
    }

    /// Clears the field's list attribute.
    pub fn without_list(mut self) -> Self {
        self.list = None;
        self
    }

    /// Sets the field's place holder attribute.
    ///
    /// - `place_holder` : The new field's place holder.
    pub fn with_place_holder(mut self, place_holder: String) -> Self {
        self.place_holder = Some(place_holder);
        self
    }

    /// Clears the field's place holder attribute.
    pub fn without_place_holder(mut self) -> Self {
        self.place_holder = None;
        self
    }

    /// Sets the field's pattern attribute.
    ///
    /// - `pattern` : The new field's pattern.
    pub fn with_pattern(mut self, pattern: String) -> Self {
        self.pattern = Some(pattern);
        self
    }

    /// Clears the field's pattern attribute.
    pub fn without_pattern(mut self) -> Self {
        self.pattern = None;
        self
    }

    /// Sets the field's size attribute.
    ///
    /// - `size` : The new field's size.
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Clears the field's size attribute.
    pub fn without_size(mut self) -> Self {
        self.size = None;
        self
    }

    /// Sets the field's required attribute at true.
    pub fn set_required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the field's required attribute at false.
    pub fn set_non_required(mut self) -> Self {
        self.required = false;
        self
    }

    /// Sets the field's read only attribute at true.
    pub fn set_read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Sets the field's read only attribute at false.
    pub fn set_non_read_only(mut self) -> Self {
        self.read_only = false;
        self
    }

    /// Sets the field's spell check attribute at true.
    pub fn set_spell_check(mut self) -> Self {
        self.spell_check = true;
        self
    }

    /// Sets the field's spell check attribute at false.
    pub fn set_non_spell_check(mut self) -> Self {
        self.spell_check = false;
        self
    }
}

impl AsHtmlConfig for TextInputConfig {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if let Some(max_length) = &self.max_length {
            configs = configs.set_attribute("maxlength".to_string(), Some(max_length.to_string()));
        }
        if let Some(min_length) = &self.min_length {
            configs = configs.set_attribute("minlength".to_string(), Some(min_length.to_string()));
        }
        if let Some(list) = &self.list {
            configs = configs.set_attribute("list".to_string(), Some(list.clone()));
        }
        if let Some(place_holder) = &self.place_holder {
            configs = configs.set_attribute("placeholder".to_string(), Some(place_holder.clone()));
        }
        if let Some(pattern) = &self.pattern {
            configs = configs.set_attribute("pattern".to_string(), Some(pattern.clone()));
        }
        if let Some(size) = self.size {
            configs = configs.set_attribute("size".to_string(), Some(size.to_string()));
        }
        if self.required {
            configs = configs.set_attribute("required".to_string(), None);
        }
        if self.read_only {
            configs = configs.set_attribute("readonly".to_string(), None);
        }
        if self.spell_check {
            configs = configs.set_attribute("spellcheck".to_string(), None);
        }

        configs
    }
}

impl Default for TextInputConfig {
    fn default() -> Self {
        Self {
            max_length: None,
            min_length: None,
            list: None,
            place_holder: None,
            pattern: None,
            size: None,
            required: false,
            read_only: false,
            spell_check: false,
        }
    }
}
