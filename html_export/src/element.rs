use std::{
    collections::{HashMap, HashSet},
    io::Write,
    ops::{Add, AddAssign},
};

use crate::{
    errors::{CreateElementResult, ElementError, WrapElementError, WrapResult},
    html::Html,
    miscellaneous::WrapMode,
    tags::TagType,
};

/// Represents an Element.
/// The Html tree is either composed of `HtmlElement` or raw String.
#[derive(Clone, Debug)]
pub enum Element {
    Text(String),
    Element(HtmlElement),
}

/// Represents an Html Element.
///
/// `tag` : The element's Html tag.
/// `children` : The element's sub-elements.
/// `config` : The element's Html configs.
#[derive(Clone, Debug)]
pub struct HtmlElement {
    pub tag: TagType,
    pub children: Vec<Box<Element>>,
    pub config: HtmlElementConfig,
}

/// Represents an Html element config
///
/// `classes` : The element's css classes.
/// `id` : The element's id.
/// `attributes` : The element's key-value attributes.
#[derive(Clone, Debug)]
pub struct HtmlElementConfig {
    pub classes: HashSet<String>,
    pub id: Option<String>,
    pub attributes: HashMap<String, Option<String>>,
}

impl Element {
    /// Attempts to wrap the element with the given HtmlElement.
    ///  
    /// `element` : The wrapper Html element.
    /// `mode` : Defines how to wrap the element.
    pub fn wrap_with_element(self, mut element: HtmlElement, mode: WrapMode) -> WrapResult {
        if !element.tag.can_wrap() {
            return Err(WrapElementError::NonWrapperElement(element.tag));
        }
        if element.children.len() > 0 && element.tag.is_single_child() {
            return Err(WrapElementError::SingleChildExceeded(element.tag));
        }
        match mode {
            WrapMode::End => element.children.push(Box::new(self)),
            WrapMode::Start => element.children.insert(0, Box::new(self)),
            WrapMode::At(index) => {
                if index > element.children.len() {
                    return Err(WrapElementError::InvalidIndex(
                        index,
                        element.children.len(),
                    ));
                } else {
                    element.children.insert(index, Box::new(self))
                }
            }
        };
        Ok(Element::Element(element))
    }

    /// Indicates if the element can contain another child.
    pub fn can_add_child(&self) -> bool {
        match self {
            Element::Text(_) => false,
            Element::Element(html_element) => html_element.can_add_child(),
        }
    }

    /// Adds without checking the child to the element.
    ///
    /// `child` : The sub-element to add.
    pub fn add_mut_element(&mut self, child: Box<Element>) {
        match self {
            Element::Text(_) => panic!("Can not add child to {:?}", self),
            Element::Element(e) => e.add_mut_child(child),
        }
    }

    /// Indicates if the element is allowed in the html document's `<head>`.
    pub fn is_allowed_in_head(&self) -> bool {
        match self {
            Element::Text(_) => false,
            Element::Element(html_element) => html_element.tag.is_allowed_is_head(),
        }
    }
}

impl HtmlElement {
    /// Creates an empty Html element of type `TagType`.
    ///
    /// `tag` : The Html element's type.
    /// `config` : The Html element's config.
    pub fn new(tag: TagType, config: HtmlElementConfig) -> Self {
        Self {
            tag,
            config,
            children: vec![],
        }
    }

    /// Creates a new Html element of type `TagType` with content `Box<Element>`s.
    /// Note that some tags restrict their content's length:
    /// - Some elements can not have sub-elements.
    /// - Some elements can have at most 1 sub-element.
    /// If the content does not respect the restrictions, the Err variant is returned.
    ///
    /// `tag` : The Html element's type.
    /// `config` : The Html element's config.
    /// `content` : The Html element's content.
    pub fn new_with_content<T>(
        tag: TagType,
        config: HtmlElementConfig,
        content: T,
    ) -> CreateElementResult
    where
        T: Iterator<Item = Box<Element>>,
    {
        let children = content.collect::<Vec<Box<Element>>>();
        if (!tag.can_wrap() && children.len() > 0) || (tag.is_single_child() && children.len() > 1)
        {
            return Err(ElementError::InvalidContent(tag));
        }
        Ok(Self {
            tag,
            config,
            children,
        })
    }

    /// Returns the Html element's sub-elements.
    pub fn get_children(&self) -> Vec<Box<Element>> {
        self.children.clone()
    }

    /// Attempts to add the sub-element to the element.
    /// Note that some tags restrict their content's length:
    /// - Some elements can not have sub-elements.
    /// - Some elements can have at most 1 sub-element.
    /// If the restrictions are not respected when adding the child, the Err variant is returned.
    ///
    /// `child` : The sub-element to add.
    pub fn add_child(mut self, child: Element) -> Result<Self, ElementError> {
        if !self.tag.can_wrap() || (self.tag.is_single_child() && !self.children.is_empty()) {
            return Err(ElementError::InvalidContent(self.tag));
        }
        self.children.push(Box::new(child));
        Ok(self)
    }

    /// Attempts to add the sub-elements to the element.
    /// Note that some tags restrict their content's length:
    /// - Some elements can not have sub-elements.
    /// - Some elements can have at most 1 sub-element.
    /// If the restrictions are not respected when adding the children, the Err variant is returned.
    ///
    /// `children` : The sub-elements to add.
    pub fn add_children<T>(mut self, children: T) -> Result<Self, ElementError>
    where
        T: Iterator<Item = Box<Element>>,
    {
        if !self.tag.can_wrap() || (self.tag.is_single_child() && !self.children.is_empty()) {
            return Err(ElementError::InvalidContent(self.tag));
        }
        self.children.extend(children);
        Ok(self)
    }

    /// Attempts to remove the element's sub-element at the given index.
    ///
    /// `index` : The sub-element's index.
    pub fn remove_child_at(mut self, index: usize) -> Result<Self, ElementError> {
        if index >= self.children.len() {
            Err(ElementError::InvalidIndex(index, self.children.len()))
        } else {
            self.children.remove(index);
            Ok(self)
        }
    }

    /// Removes all the element's sub-element.
    pub fn clear_children(mut self) -> Self {
        self.children.clear();
        self
    }

    /// Sets the Html element's config.
    ///
    /// `config` : The new element's config.
    pub fn set_config(mut self, config: HtmlElementConfig) -> Self {
        self.config = config;
        self
    }

    /// Returns a mutable reference of the Html element's config.
    pub fn get_mut_config(&mut self) -> &mut HtmlElementConfig {
        &mut self.config
    }

    /// Returns a reference to the Html element's config.
    pub fn get_config(&self) -> &HtmlElementConfig {
        &self.config
    }

    /// Indicates if the Html element can contain another child.
    pub fn can_add_child(&self) -> bool {
        !(!self.tag.can_wrap() || (self.tag.is_single_child() && !self.children.is_empty()))
    }

    /// Adds without checking the child to the element.
    ///
    /// `child` : The sub-element to add.
    fn add_mut_child(&mut self, child: Box<Element>) {
        self.children.push(child);
    }
}

impl HtmlElementConfig {
    /// Creates an empty Html element config.
    pub fn new_empty() -> Self {
        Self::default()
    }

    /// Creates a new Html element's config.
    ///
    /// `classes` : The element's css classes.
    /// `id` : The element's id.
    /// `attributes` : The element's key-value attributes.
    /// `Properties` : The element's properties (attibutes with no values).
    pub fn new(
        classes: HashSet<String>,
        id: Option<String>,
        attributes: HashMap<String, Option<String>>,
    ) -> Self {
        Self {
            classes,
            id,
            attributes,
        }
    }

    /// Creates a new Html element's config with only classes and id.
    ///
    /// `classes` : The element's css classes.
    /// `id` : The element's id.
    pub fn new_class_id<T>(classes: T, id: Option<String>) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        Self {
            classes: HashSet::from_iter(classes),
            id,
            ..Self::default()
        }
    }

    /// Returns the element's id.
    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    /// Updates the element's id.
    ///
    /// `id` : The new element's id.
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Removes the element's id.
    pub fn remove_id(mut self) -> Self {
        self.id = None;
        self
    }

    /// Returns the element's css classes.
    pub fn get_classes(&self) -> HashSet<String> {
        self.classes.clone()
    }

    /// Adds the class to the element's class list.
    ///
    /// `class` : The class to add.
    pub fn with_class(mut self, class: String) -> Self {
        self.classes.insert(class);
        self
    }

    /// Adds the classes to element's class list.
    ///
    /// `classes` : The classes to add.
    pub fn with_classes<T>(mut self, classes: T) -> Self
    where
        T: Iterator<Item = String>,
    {
        self.classes.extend(classes);
        self
    }

    /// Replaces the element's class list with the given one.
    ///
    /// `classes` : The new element's classes.
    pub fn set_classes<T>(mut self, classes: T) -> Self
    where
        T: Iterator<Item = String>,
    {
        self.classes = classes.collect();
        self
    }

    /// Checks if the element's class list contains the given class.
    ///
    /// `class` : the targeted class.
    pub fn has_class(&self, class: String) -> bool {
        self.classes.contains(&class)
    }

    /// Removes the given class from the element's class list.
    ///
    /// `class` : The class to be removed.
    pub fn remove_class(mut self, class: String) -> Self {
        self.classes.remove(&class);
        self
    }

    /// Clears the element's class list.
    pub fn clear_classes(mut self) -> Self {
        self.classes.clear();
        self
    }

    /// Returns the element's key-value attributes.
    pub fn get_attributes(&self) -> HashMap<String, Option<String>> {
        self.attributes.clone()
    }

    /// Returns the element's targeted attribute value.  
    /// If the element does not have an attribute named `name`, the function returns None.
    ///
    /// `name` : The targeted attribute.
    pub fn get_attribute(&self, name: String) -> Option<Option<String>> {
        match self.attributes.get(&name) {
            Some(string) => Some(string.clone()),
            _ => None,
        }
    }

    /// Checks if the element has an attribute named `name`.
    ///
    /// `name` : The targeted attribute.
    pub fn has_attribute(&self, name: String) -> bool {
        self.attributes.contains_key(&name)
    }

    /// Updates the element's attribute named `name` with the value `value`.
    /// If the attribute named `name` does not exists it is declared with the value `value`
    ///
    /// `name` : The attribute name.
    /// `value` : The attribute's new value.
    pub fn set_attribute(mut self, name: String, value: Option<String>) -> Self {
        self.attributes
            .entry(name)
            .and_modify(|e| *e = value.clone())
            .or_insert(value);
        self
    }

    /// Updates the element's attributes.
    /// If an attribute does not exists it is declared.
    ///
    /// `attributes` : The attributes to update or declare.
    pub fn set_attributes(mut self, attributes: HashMap<String, Option<String>>) -> Self {
        for (key, value) in attributes {
            self.attributes
                .entry(key)
                .and_modify(|e| *e = value.clone())
                .or_insert(value);
        }
        self
    }

    /// Removes the given attribute from the element's attributes.
    ///
    /// `name` : The attribute to be removed.
    pub fn remove_attribute(mut self, name: String) -> Self {
        self.attributes.remove(&name);
        self
    }

    /// Removes all the element's attributes.
    pub fn clear_attributes(mut self) -> Self {
        self.attributes.clear();
        self
    }
}

impl Default for HtmlElementConfig {
    fn default() -> Self {
        Self {
            classes: HashSet::new(),
            id: None,
            attributes: HashMap::new(),
        }
    }
}

impl Html for Element {
    /// Writes the Element in Html format onto the given writer.
    ///
    /// `writer` : The writer
    /// `tabs` : Amount of tabulations to insert before writing the element.
    fn write_html(
        &self,
        mut writer: std::io::BufWriter<std::fs::File>,
        tabs: usize,
    ) -> std::io::Result<std::io::BufWriter<std::fs::File>> {
        match &self {
            Element::Text(text) => {
                writer.write(format!("{}\n", text).as_bytes())?;
                Ok(writer)
            }
            Element::Element(html_element) => html_element.write_html(writer, tabs),
        }
    }
}

impl Html for HtmlElement {
    /// Writes the Html element in Html format onto the given writer.
    ///
    /// `writer` : The writer
    /// `tabs` : Amount of tabulations to insert before writing the element.
    fn write_html(
        &self,
        mut writer: std::io::BufWriter<std::fs::File>,
        tabs: usize,
    ) -> std::io::Result<std::io::BufWriter<std::fs::File>> {
        let tabbing = std::iter::repeat("\t").take(tabs).collect::<String>();
        writer.write(format!("{}<{}", tabbing, self.tag.get_tag_name()).as_bytes())?;
        writer = self.config.write_html(writer, tabs)?;
        if self.children.is_empty() {
            if self.tag.is_auto_closing() {
                writer.write(" />\n".as_bytes())?;
            } else {
                writer.write(format!("></{}>\n", self.tag.get_tag_name()).as_bytes())?;
            }
        } else {
            writer.write(">\n".as_bytes())?;
            for child in self.children.iter() {
                writer = child.write_html(writer, tabs + 1)?;
            }
            writer.write(format!("{}</{}>\n", tabbing, self.tag.get_tag_name()).as_bytes())?;
        }
        Ok(writer)
    }
}

impl Html for HtmlElementConfig {
    /// Writes the Html element's config in Html format onto the given writer.
    /// The configs are written in an xml-attribute format.
    /// It is **expected** that the given writer has **already opened the Html tag**.
    /// This function **does not** close the Html tag.
    ///
    /// `writer` : The writer.
    /// `_tabs` : Unused.
    fn write_html(
        &self,
        mut writer: std::io::BufWriter<std::fs::File>,
        _tabs: usize,
    ) -> std::io::Result<std::io::BufWriter<std::fs::File>> {
        if self.classes.len() > 0 {
            writer.write(
                format!(
                    " class=\"{}\"",
                    self.classes
                        .clone()
                        .into_iter()
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_bytes(),
            )?;
        }
        if let Some(id) = &self.id {
            writer.write(format!(" id=\"{}\"", id).as_bytes())?;
        }
        for (key, value) in self.attributes.iter() {
            match value {
                Some(content) => writer.write(format!(" {}=\"{}\"", key, content).as_bytes())?,
                None => writer.write(format!(" {}", key).as_bytes())?,
            };
        }

        Ok(writer)
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(l0), Self::Text(r0)) => l0 == r0,
            (Self::Element(l0), Self::Element(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialEq for HtmlElement {
    fn eq(&self, other: &Self) -> bool {
        if self.tag != other.tag
            || self.config != other.config
            || self.children.len() != other.children.len()
        {
            return false;
        }
        for (c1, c2) in self.children.iter().zip(&other.children) {
            if c1 != c2 {
                return false;
            }
        }
        true
    }
}

impl PartialEq for HtmlElementConfig {
    fn eq(&self, other: &Self) -> bool {
        self.classes == other.classes && self.id == other.id && self.attributes == other.attributes
    }
}

impl AddAssign for Element {
    fn add_assign(&mut self, rhs: Self) {
        if !self.can_add_child() {
            panic!("Can not add child to {:?}", self)
        }
        self.add_mut_element(Box::new(rhs));
    }
}

impl AddAssign for HtmlElement {
    fn add_assign(&mut self, rhs: Self) {
        if !self.can_add_child() {
            panic!("Can not add child to {:?}", self)
        }
        self.children.push(Box::new(Element::Element(rhs)));
    }
}

impl Add for Element {
    type Output = Element;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Element::Text(_) => panic!("Can not add child to {:?}", self),
            Element::Element(html_element) => {
                Element::Element(html_element.add_child(rhs).unwrap())
            }
        }
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        Element::Text(value.to_string())
    }
}
