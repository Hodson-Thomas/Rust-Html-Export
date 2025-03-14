use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    tags::TagType,
};

use super::field::{AsHtmlConfig, InputFieldConfig};

/// Defines the `<select>` configs:
///
/// - `auto_complete` : The select's auto complete mode. If the value is None, this attribute is ignored.
/// - `form` : The select's associated form id. If the value is None, this attribute is ignored.
/// - `size` : The select's items displayed. If the value is None, this attribute is ignored.
/// - `multiple` : Indicates if the select can have multiple selected values. If the value is None, this attribute is ignored.
/// - `auto_focus` : Indicates if the select's auto focus is active. If the value is None, this attribute is ignored.
/// - `disabled` : Indicates if the select is disabled. If the value is None, this attribute is ignored.
/// - `required` : Indicates if the select is required. If the value is None, this attribute is ignored.
pub struct SelectInputConfigs {
    pub auto_complete: Option<String>,
    pub form: Option<String>,
    pub size: Option<usize>,
    pub multiple: Option<bool>,
    pub auto_focus: Option<bool>,
    pub disabled: Option<bool>,
    pub required: Option<bool>,
}

/// The `<select>` content.
pub enum Options {
    OptionGroup(SelectOptionGroup, HtmlElementConfig),
    Option(SelectOption, HtmlElementConfig),
}

/// Defines a `<optgroup>` html element.
///
/// - `content` : The option group's options.
/// - `label` : The option group's label. If the value is None, this attribute is ignored.
/// - `disabled` : Indicates if the option group is disabled. If the value is None, this attribute is ignored.
pub struct SelectOptionGroup {
    pub content: Vec<(SelectOption, HtmlElementConfig)>,
    pub label: Option<String>,
    pub disabled: Option<bool>,
}

/// Defines an `<option>` html element.
///
/// - `value` : The option's value.
/// - `label` : The option's displayed value.
/// - `disabled` : Indicates if the option is disabled. If the value is None, this attribute is ignored.
/// - `selected` : Indicates if the option is selected. If the value is None, this attribute is ignored.
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: Option<bool>,
    pub selected: Option<bool>,
}

/// Creates a `<label>` + `<select>` html structure.
///
/// - `html_configs` : The label and selecct html configs.
/// - `select_configs` : The select's configs.
/// - `values` : The select's content.
pub fn create_labeled_select(
    html_configs: InputFieldConfig,
    select_configs: SelectInputConfigs,
    values: Vec<Options>,
) -> Element {
    let label = Element::Element(HtmlElement::new(
        TagType::Label,
        html_configs
            .label_config
            .set_attribute("for".to_string(), Some(html_configs.id.clone())),
    )) + Element::Text(html_configs.label);

    let configs = select_configs.set_html_configs(
        html_configs
            .field_config
            .set_attribute("id".to_string(), Some(html_configs.id))
            .set_attribute("name".to_string(), Some(html_configs.name)),
    );
    let mut select = Element::Element(HtmlElement::new(TagType::Select, configs));
    for option in values {
        select += match option {
            Options::OptionGroup(select_option_group, html_element_config) => {
                create_option_group(select_option_group, html_element_config)
            }
            Options::Option(select_option, html_element_config) => {
                create_option(select_option, html_element_config)
            }
        };
    }
    Element::Element(HtmlElement::new(
        TagType::Div,
        HtmlElementConfig::new_empty(),
    )) + label
        + select
}

/// Creates a `<select>` html element.
///
/// - `html_configs` : The select's html configs.
/// - `select_configs` : The select's configs.
/// - `value` : The select's values.
pub fn create_select(
    html_configs: InputFieldConfig,
    select_configs: SelectInputConfigs,
    values: Vec<Options>,
) -> Element {
    let configs = select_configs.set_html_configs(
        html_configs
            .field_config
            .set_attribute("id".to_string(), Some(html_configs.id))
            .set_attribute("name".to_string(), Some(html_configs.name)),
    );
    let mut select = Element::Element(HtmlElement::new(TagType::Select, configs));
    for option in values {
        select += match option {
            Options::OptionGroup(select_option_group, html_element_config) => {
                create_option_group(select_option_group, html_element_config)
            }
            Options::Option(select_option, html_element_config) => {
                create_option(select_option, html_element_config)
            }
        };
    }
    Element::Element(HtmlElement::new(
        TagType::Div,
        HtmlElementConfig::new_empty(),
    )) + select
}

/// Creates an `<optgroup>` with its `<option>` content.
///
/// - `option_group` : The option group's content and configs.
/// - `configs` : The option group's html configs.
pub fn create_option_group(option_group: SelectOptionGroup, configs: HtmlElementConfig) -> Element {
    let mut configs = configs;
    if option_group.label.is_some() {
        configs = configs.set_attribute("label".to_string(), option_group.label);
    }
    if let Some(disabled) = option_group.disabled {
        if disabled {
            configs = configs.set_attribute("disabled".to_string(), None);
        }
    }
    let mut optgroup = Element::Element(HtmlElement::new(TagType::OptGroup, configs));
    for (option, option_configs) in option_group.content {
        optgroup += create_option(option, option_configs);
    }
    optgroup
}

/// Creates an `<option>` html element.
///
/// - `option` : The option's content and configs.
/// - `configs` : The option's html configs.
pub fn create_option(option: SelectOption, configs: HtmlElementConfig) -> Element {
    let mut configs = configs.set_attribute("value".to_string(), Some(option.value));
    if let Some(disabled) = option.disabled {
        if disabled {
            configs = configs.set_attribute("disabled".to_string(), None);
        }
    }
    if let Some(selected) = option.selected {
        if selected {
            configs = configs.set_attribute("selected".to_string(), None);
        }
    }
    Element::Element(HtmlElement::new(TagType::Option, configs)) + Element::Text(option.label)
}

impl SelectInputConfigs {
    /// Creates a new default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new auto complete parameter.
    ///
    /// - `auto_complete` : The new auto_complete parameter.
    pub fn with_auto_complete(mut self, auto_complete: String) -> Self {
        self.auto_complete = Some(auto_complete);
        self
    }

    /// Removes the auto complete parameter.
    pub fn without_auto_complete(mut self) -> Self {
        self.auto_complete = None;
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

    /// Sets the new multiple parameter state.
    ///
    /// - `multiple` : The new multiple parameter state.
    pub fn with_multiple(mut self, multiple: bool) -> Self {
        self.multiple = Some(multiple);
        self
    }

    /// Removes the multiple parameter.
    pub fn without_multiple(mut self) -> Self {
        self.multiple = None;
        self
    }

    /// Sets the new auto focus parameter state.
    ///
    /// - `auto_focus` : The new auto focus parameter state.
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

    /// Sets the new required parameter state.
    ///
    /// - `required` : The new required parameter state.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Removes the required parameter.
    pub fn without_required(mut self) -> Self {
        self.required = None;
        self
    }
}

impl SelectOptionGroup {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an `<option>` to the `<optgroup>`.
    ///
    /// - `option` : The option to add.
    pub fn with_option(mut self, option: (SelectOption, HtmlElementConfig)) -> Self {
        self.content.push(option);
        self
    }

    /// Adds the `<option>`s to the `<optgroup>`.
    ///
    /// - `content` : The collection of `<option>` to add.
    pub fn with_content<T>(mut self, content: T) -> Self
    where
        T: Iterator<Item = (SelectOption, HtmlElementConfig)>,
    {
        self.content.extend(content);
        self
    }

    /// Sets the new label parameter.
    ///
    /// - `label` : The new label parameter.
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Removes the label parameter.
    pub fn without_label(mut self) -> Self {
        self.label = None;
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
}

impl SelectOption {
    /// Creates a new `<option>`.
    ///
    /// - `value` : The option's value.
    /// - `label` : The option's label.
    pub fn new(value: String, label: String) -> Self {
        Self {
            value,
            label,
            ..Self::default()
        }
    }

    /// Sets the new label parameter.
    ///
    /// - `label` : The new label parameter.
    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    /// Sets the new value parameter.
    ///
    /// - `value` : The new value parameter.
    pub fn set_value(mut self, value: String) -> Self {
        self.value = value;
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

    /// Sets the new selected parameter state.
    ///
    /// - `selected` : The new selected parameter state.
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Removes the selected parameter.
    pub fn without_selected(mut self) -> Self {
        self.selected = None;
        self
    }
}

impl AsHtmlConfig for SelectInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.auto_complete.is_some() {
            configs = configs.set_attribute("autocomplete".to_string(), self.auto_complete.clone());
        }
        if self.form.is_some() {
            configs = configs.set_attribute("form".to_string(), self.form.clone());
        }
        if let Some(size) = self.size {
            configs = configs.set_attribute("size".to_string(), Some(size.to_string()));
        }
        if let Some(multiple) = self.multiple {
            if multiple {
                configs = configs.set_attribute("multiple".to_string(), None);
            }
        }
        if let Some(auto_focus) = self.auto_focus {
            if auto_focus {
                configs = configs.set_attribute("autofocus".to_string(), None);
            }
        }
        if let Some(disabled) = self.disabled {
            if disabled {
                configs = configs.set_attribute("disabled".to_string(), None);
            }
        }
        if let Some(required) = self.required {
            if required {
                configs = configs.set_attribute("required".to_string(), None);
            }
        }
        configs
    }
}

impl Default for SelectInputConfigs {
    fn default() -> Self {
        Self {
            auto_complete: None,
            form: None,
            size: None,
            multiple: None,
            auto_focus: None,
            disabled: None,
            required: None,
        }
    }
}

impl Default for SelectOptionGroup {
    fn default() -> Self {
        Self {
            content: Default::default(),
            label: Default::default(),
            disabled: Default::default(),
        }
    }
}

impl Default for SelectOption {
    fn default() -> Self {
        Self {
            value: String::new(),
            label: String::new(),
            disabled: None,
            selected: None,
        }
    }
}
