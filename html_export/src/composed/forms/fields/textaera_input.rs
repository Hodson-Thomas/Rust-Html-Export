use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    tags::TagType,
};

use super::field::{AsHtmlConfig, InputFieldConfig};

/// Defines the text area input configs.
///
/// - `auto_capitalize` : The text area's capitalization mode. If the value is None this parameter is ignored.
/// - `cols` : The number of characters per ligne. If the value is None this parameter is ignored.
/// - `rows` : The number of rows. If the value is None this parameter is ignored.
/// - `form` : The text area's associated form id. If the value is None this parameter is ignored.
/// - `max_length` : The text area's content maximum length. If the value is None this parameter is ignored.
/// - `min_length` : The text area's content minimum length. If the value is None this parameter is ignored.
/// - `place_holder` : The text area's place holder text. If the value is None this parameter is ignored.
/// - `wrap` : The text area's content wrap mode. If the value is None this parameter is ignored.
/// - `auto_complete` : Indicates if the text area's auto complete mode is active. If the value is None this parameter is ignored.
/// - `auto_focus` : Indicates if the text area's auto focus is active. If the value is None this parameter is ignored.
/// - `disabled` : Indicates if the text area is disabled. If the value is None this parameter is ignored.
/// - `read_only` : Indicates if the text area is in read only mode. If the value is None this parameter is ignored.
/// - `spell_check` : Indicates if the text area spell check mode is active. If the value is None this parameter is ignored.
pub struct TextAreaInputConfigs {
    pub auto_capitalize: Option<AutoCapitalize>,
    pub cols: Option<usize>,
    pub rows: Option<usize>,
    pub form: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub place_holder: Option<String>,
    pub wrap: Option<WrapMode>,
    pub auto_complete: Option<bool>,
    pub auto_focus: Option<bool>,
    pub disabled: Option<bool>,
    pub read_only: Option<bool>,
    pub spell_check: Option<bool>,
}

/// The auto capitalize modes.
///
/// - `None` : Deactivated.
/// - `Sentences` : Automatically capitalizes the first letter of each sentences.
/// - `Words` : Automatically capitalizes each word.
/// - `Characters` : Automatically capitalizes every characters.
/// - `On` : Depricated.
/// - `Off` : Depricated.
pub enum AutoCapitalize {
    None,
    Sentences,
    Words,
    Characters,
    On,
    Off,
}

/// The different wrap modes.
///
/// - `Hard` : Automatically inserts `CR + LF` when the text length exceeds the text area width.
/// - `Soft` : Automatically replaces the line breaks with `CR + LF`.
/// - `Off` : Does nothing.
pub enum WrapMode {
    Hard,
    Soft,
    Off,
}

/// Creates a `<label>` + `<textarea>` html structure.
///
/// - `html_configs` : The label and text area html configs.
/// - `text_area_configs` : The text area's configs.
/// - `value` : The text area's optional value.
pub fn create_labeled_text_area(
    html_configs: InputFieldConfig,
    text_area_configs: TextAreaInputConfigs,
    value: Option<String>,
) -> Element {
    let label = Element::Element(HtmlElement::new(
        TagType::Label,
        html_configs
            .label_config
            .set_attribute("for".to_string(), Some(html_configs.id.clone())),
    )) + Element::Text(html_configs.label.to_string());

    let configs = text_area_configs.set_html_configs(
        html_configs
            .field_config
            .set_attribute("id".to_string(), Some(html_configs.id.clone()))
            .set_attribute("name".to_string(), Some(html_configs.name.clone())),
    );

    let mut text_area = Element::Element(HtmlElement::new(TagType::TextArea, configs));
    if let Some(value) = &value {
        text_area += Element::Text(value.clone());
    }

    Element::Element(HtmlElement::new(
        TagType::Div,
        HtmlElementConfig::new_empty(),
    )) + label
        + text_area
}

/// Creates a `<textarea>` html element.
///
/// - `html_configs` : The text area's html configs.
/// - `text_area_configs` : The text area's configs.
/// - `value` : The text area's optional value.
pub fn create_text_area(
    html_configs: InputFieldConfig,
    text_area_configs: TextAreaInputConfigs,
    value: Option<String>,
) -> Element {
    let configs = text_area_configs.set_html_configs(
        html_configs
            .field_config
            .set_attribute("id".to_string(), Some(html_configs.id.clone()))
            .set_attribute("name".to_string(), Some(html_configs.name.clone())),
    );

    let mut text_area = Element::Element(HtmlElement::new(TagType::TextArea, configs));
    if let Some(value) = &value {
        text_area += Element::Text(value.clone());
    }

    Element::Element(HtmlElement::new(
        TagType::Div,
        HtmlElementConfig::new_empty(),
    )) + text_area
}

impl TextAreaInputConfigs {
    /// Creates a new default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new auto capitalize parameter.
    ///
    /// - `auto_capitalize` : The new auto capitalize parameter.
    pub fn with_auto_capitalize(mut self, auto_capitalize: AutoCapitalize) -> Self {
        self.auto_capitalize = Some(auto_capitalize);
        self
    }

    /// Removes the auto capitalize parameter.
    pub fn without_auto_capitalize(mut self) -> Self {
        self.auto_capitalize = None;
        self
    }

    /// Sets the new cols parameter.
    ///
    /// - `cols` : The new cols parameter.
    pub fn with_cols(mut self, cols: usize) -> Self {
        self.cols = Some(cols);
        self
    }

    /// Removes the cols parameter.
    pub fn without_cols(mut self) -> Self {
        self.cols = None;
        self
    }

    /// Sets the new rows parameter.
    ///
    /// - `rows` : The new rows parameter.
    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    /// Removes the rows parameter.
    pub fn without_rows(mut self) -> Self {
        self.rows = None;
        self
    }

    /// Sets the new form parameter.
    ///
    /// - `form` : The new form parameter.
    pub fn with_form(mut self, form: String) -> Self {
        self.form = Some(form);
        self
    }

    /// Removes the form parameter.
    pub fn without_form(mut self) -> Self {
        self.form = None;
        self
    }

    /// Sets the new max length parameter.
    ///
    /// - `max_length` : The new max length parameter.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Removes the max ength parameter.
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

    /// Sets the new wrap parameter.
    ///
    /// - `wrap` : The new wrap parameter.
    pub fn with_wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = Some(wrap);
        self
    }

    /// Removes the wrap parameter.
    pub fn without_wrap(mut self) -> Self {
        self.wrap = None;
        self
    }

    /// Sets the new auto complete parameter state.
    ///
    /// - `auto_complete` : The new auto complete parameter state.
    pub fn with_auto_complete(mut self, auto_complete: bool) -> Self {
        self.auto_complete = Some(auto_complete);
        self
    }

    /// Removes the auto complete parameter.
    pub fn without_auto_complete(mut self) -> Self {
        self.auto_complete = None;
        self
    }

    /// Sets the new auto focus parameter state.
    ///
    /// - `auto_focus` : The new auto _focus parameter state.
    pub fn with_auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = Some(auto_focus);
        self
    }

    /// Removes the auto focus parameter.
    pub fn without_auto_focus(mut self) -> Self {
        self.auto_focus = None;
        self
    }

    /// Sets the new disabled parameter state.
    ///
    /// - `disabled` : The new disabled parameter state.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Removes the disabled parameter.
    pub fn without_disabled(mut self) -> Self {
        self.disabled = None;
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

    /// Sets the new spell check parameter state.
    ///
    /// - `spell_check` : The new spell check parameter state.
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

impl AsHtmlConfig for TextAreaInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if let Some(auto_capitalize) = &self.auto_capitalize {
            configs = auto_capitalize.set_html_configs(configs);
        }
        if let Some(wrap) = &self.wrap {
            configs = wrap.set_html_configs(configs);
        }
        if let Some(cols) = self.cols {
            configs = configs.set_attribute("cols".to_string(), Some(cols.to_string()));
        }
        if let Some(rows) = self.rows {
            configs = configs.set_attribute("rows".to_string(), Some(rows.to_string()));
        }
        if let Some(max_length) = self.max_length {
            configs = configs.set_attribute("maxlength".to_string(), Some(max_length.to_string()));
        }
        if let Some(min_length) = self.min_length {
            configs = configs.set_attribute("minlength".to_string(), Some(min_length.to_string()));
        }
        if self.form.is_some() {
            configs = configs.set_attribute("form".to_string(), self.form.clone());
        }
        if self.place_holder.is_some() {
            configs = configs.set_attribute("placeholder".to_string(), self.place_holder.clone());
        }
        if let Some(auto_complete) = self.auto_complete {
            configs =
                configs.set_attribute("autocomplete".to_string(), Some(auto_complete.to_string()));
        }
        if let Some(auto_focus) = self.auto_focus {
            configs = configs.set_attribute("autofocus".to_string(), Some(auto_focus.to_string()));
        }
        if let Some(disabled) = self.disabled {
            configs = configs.set_attribute("disabled".to_string(), Some(disabled.to_string()));
        }
        if let Some(read_only) = self.read_only {
            configs = configs.set_attribute("readonly".to_string(), Some(read_only.to_string()));
        }
        if let Some(spell_check) = self.spell_check {
            configs =
                configs.set_attribute("spellcheck".to_string(), Some(spell_check.to_string()));
        }
        configs
    }
}

impl AsHtmlConfig for AutoCapitalize {
    fn set_html_configs(&self, configs: HtmlElementConfig) -> HtmlElementConfig {
        configs.set_attribute(
            "autocapitalize".to_string(),
            Some(
                match self {
                    AutoCapitalize::None => "none",
                    AutoCapitalize::Sentences => "sentences",
                    AutoCapitalize::Words => "words",
                    AutoCapitalize::Characters => "characters",
                    AutoCapitalize::On => "on",
                    AutoCapitalize::Off => "off",
                }
                .to_string(),
            ),
        )
    }
}

impl AsHtmlConfig for WrapMode {
    fn set_html_configs(&self, configs: HtmlElementConfig) -> HtmlElementConfig {
        configs.set_attribute(
            "wrap".to_string(),
            Some(
                match self {
                    WrapMode::Hard => "hard",
                    WrapMode::Soft => "soft",
                    WrapMode::Off => "off",
                }
                .to_string(),
            ),
        )
    }
}

impl Default for TextAreaInputConfigs {
    fn default() -> Self {
        Self {
            auto_capitalize: None,
            auto_complete: None,
            auto_focus: None,
            cols: None,
            rows: None,
            disabled: None,
            form: None,
            max_length: None,
            min_length: None,
            place_holder: None,
            read_only: None,
            spell_check: None,
            wrap: None,
        }
    }
}
