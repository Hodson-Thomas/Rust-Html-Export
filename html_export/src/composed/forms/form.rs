use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    tags::TagType,
};

/// Defines the form's configs.
///
/// - `method` : The form's optional method. If the value is None, this attribute is ignored.
/// - `action` : The form's action url. If the value is None, this attribute is ignored.
pub struct FormConfig {
    pub method: Option<String>,
    pub action: Option<String>,
}

pub trait AsForm {
    fn as_form_field(&self) -> Option<Element>;
}

/// Wraps the given fields in a `<fieldset>`.
///
/// - `fields` : The fields to wrap.
/// - `legend` : The fieldset's legend content.
/// - `fieldset_config` : The fieldset's html configs.
/// - `legend_config` : The fieldset's legend html configs.
pub fn wrap_fields_in_fieldset(
    fields: Vec<Element>,
    legend: String,
    fieldset_config: HtmlElementConfig,
    legend_config: HtmlElementConfig,
) -> Element {
    let mut fieldset = Element::Element(HtmlElement::new(TagType::FieldSet, fieldset_config));
    let legend =
        Element::Element(HtmlElement::new(TagType::Legend, legend_config)) + Element::Text(legend);
    fieldset += legend;
    for field in fields {
        fieldset += field;
    }
    fieldset
}

/// Wraps an entity in a `<fieldset>`.
///
/// - `entity` : The entity to wrap. The entity must implement the `AsForm` trait.
/// - `legend` : The fieldset's legend content.
/// - `fieldset_config` : The fieldset's html configs.
/// - `legend_config` : The fieldset's legend html configs.
pub fn wrap_entity_in_fieldsets<T>(
    entity: &T,
    legend: String,
    fieldset_config: HtmlElementConfig,
    legend_config: HtmlElementConfig,
) -> Option<Element>
where
    T: AsForm,
{
    if let Some(field) = entity.as_form_field() {
        let mut fieldset = Element::Element(HtmlElement::new(TagType::FieldSet, fieldset_config));
        let legend = Element::Element(HtmlElement::new(TagType::Legend, legend_config))
            + Element::Text(legend);
        fieldset += legend;
        fieldset += field;
        Some(fieldset)
    } else {
        None
    }
}

/// Wraps entities in a `<fieldset>`.
///
/// - `entities` : The entities to wrap. The entities must implement the `AsForm` trait.
/// - `legend` : The fieldset's legend content.
/// - `fieldset_config` : The fieldset's html configs.
/// - `legend_config` : The fieldset's legend html configs.
pub fn wrap_entities_in_fieldset<T>(
    entities: &Vec<T>,
    legend: String,
    fieldset_config: HtmlElementConfig,
    legend_config: HtmlElementConfig,
) -> Option<Element>
where
    T: AsForm,
{
    let mut has_element = false;
    let mut fieldset = Element::Element(HtmlElement::new(TagType::FieldSet, fieldset_config));
    let legend =
        Element::Element(HtmlElement::new(TagType::Legend, legend_config)) + Element::Text(legend);
    fieldset += legend;
    for entity in entities {
        if let Some(field) = entity.as_form_field() {
            has_element = true;
            fieldset += field;
        }
    }
    if has_element {
        Some(fieldset)
    } else {
        None
    }
}

/// Creates a `<form>` with the given elements.
///
/// - `fields` : The form's fields.
/// - `form_config` : The form's configs.
/// - `html_config` : The form's html configs.
pub fn create_form(
    fields: Vec<Element>,
    form_config: FormConfig,
    html_config: HtmlElementConfig,
) -> Option<Element> {
    if fields.is_empty() {
        return None;
    }
    let mut cfg = html_config;
    if let Some(method) = form_config.method {
        cfg = cfg.set_attribute("method".to_string(), Some(method));
    }
    if let Some(action) = form_config.action {
        cfg = cfg.set_attribute("action".to_string(), Some(action));
    }
    let mut form = Element::Element(HtmlElement::new(TagType::Form, cfg));
    for field in fields {
        form += field;
    }
    Some(form)
}

impl FormConfig {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the form's method.
    ///
    /// - `method` : The form's method.
    pub fn with_method(mut self, method: String) -> Self {
        self.method = Some(method);
        self
    }

    /// Sets the form's action.
    ///
    /// - `action` : The form's action.
    pub fn with_action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            method: None,
            action: None,
        }
    }
}
