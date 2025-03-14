use crate::element::{Element, HtmlElement, HtmlElementConfig};

/// Defines the form input configs.
///
/// - `name` : The input's name.
/// - `id` : The input's id used for linking the input and the label.
/// - `label` : The label's content.
/// - `label_config` : The label's html configs.
/// - `field_config` : The input's html configs.
pub struct InputFieldConfig {
    pub name: String,
    pub id: String,
    pub label: String,
    pub label_config: HtmlElementConfig,
    pub field_config: HtmlElementConfig,
}

/// Represents an empty input configs.
pub struct NoConfigs;

impl InputFieldConfig {
    /// Creates a new object with a name, an id and a label.
    ///
    /// - `name` : The input config's name.
    /// - `id` : The input config's id.
    /// - `label` : The input config's label.
    pub fn new(name: String, id: String, label: String) -> Self {
        Self {
            name,
            label,
            label_config: HtmlElementConfig::new_empty(),
            field_config: HtmlElementConfig::new_empty(),
            id,
        }
    }

    /// Sets the name and the label attributes to the given html configs.
    pub fn set_html_configs(&self, configs: HtmlElementConfig) -> HtmlElementConfig {
        configs
            .set_attribute("name".to_string(), Some(self.name.clone()))
            .set_attribute("id".to_string(), Some(self.id.clone()))
    }
}

impl NoConfigs {
    /// Creates a new object.
    pub fn new() -> Self {
        Self
    }
}

impl AsHtmlConfig for NoConfigs {
    fn set_html_configs(&self, configs: HtmlElementConfig) -> HtmlElementConfig {
        configs
    }
}

pub trait AsHtmlConfig {
    /// Sets the entitie's properties as html attributes in the givenhtml configs object.
    fn set_html_configs(&self, configs: HtmlElementConfig) -> HtmlElementConfig;
}

/// Creates an `<input type="?">` and a `label` html structure.
///
/// - `html_configs` : The label and input html configs.
/// - `input_type` : The input's type.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value. If the valeu is None, this attribute is ignored.
pub fn create_labeled_input<T>(
    html_configs: InputFieldConfig,
    input_type: String,
    input_configs: T,
    value: Option<String>,
) -> Element
where
    T: AsHtmlConfig,
{
    let label = Element::Element(HtmlElement::new(
        crate::tags::TagType::Label,
        html_configs
            .label_config
            .clone()
            .set_attribute("for".to_string(), Some(html_configs.id.clone())),
    )) + Element::Text(html_configs.label.clone());

    let mut cfg = input_configs.set_html_configs(
        html_configs
            .set_html_configs(html_configs.field_config.clone())
            .set_attribute("type".to_string(), Some(input_type)),
    );
    if value.is_some() {
        cfg = cfg.set_attribute("value".to_string(), value);
    }
    Element::Element(HtmlElement::new(
        crate::tags::TagType::Div,
        HtmlElementConfig::new_empty(),
    )) + label
        + Element::Element(HtmlElement::new(crate::tags::TagType::Input, cfg))
}

/// Creates an `<input type="?">`.
///
/// - `html_configs` : The input's html configs.
/// - `input_type` : The input's type.
/// - `input_configs` : The input's configs.
/// - `value` : The input's optional initial value. If the valeu is None, this attribute is ignored.
pub fn create_input<T>(
    html_configs: InputFieldConfig,
    input_type: String,
    input_configs: T,
    value: Option<String>,
) -> Element
where
    T: AsHtmlConfig,
{
    let mut cfg = input_configs.set_html_configs(
        html_configs
            .set_html_configs(html_configs.field_config.clone())
            .set_attribute("type".to_string(), Some(input_type)),
    );
    if value.is_some() {
        cfg = cfg.set_attribute("value".to_string(), value);
    }
    Element::Element(HtmlElement::new(crate::tags::TagType::Input, cfg))
}
