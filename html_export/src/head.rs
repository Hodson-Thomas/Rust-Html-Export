use crate::{
    element::{Element, HtmlElement, HtmlElementConfig},
    errors::HeadCreationError,
    miscellaneous::{BaseUrlTarget, ScriptLoadMode},
    tags::TagType,
};

/// Defines the html document's `<head>` tag content.
pub struct Head {
    pub content: Vec<Element>,
}

impl Head {
    /// Creates a blank `<head>` tag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an Element to the head's content.
    /// If the given element is not compatible with the `<head>` html tag, the Err variant is returned.  
    ///
    /// - `element` : The element to add.
    pub fn with_element(mut self, element: Element) -> Result<Self, HeadCreationError> {
        if element.is_allowed_in_head() {
            self.content.push(element);
            Ok(self)
        } else {
            Err(HeadCreationError::InvalidContent(element))
        }
    }

    /// Adds a title to the html page
    ///
    /// - `title` : The title's content.
    pub fn with_title(mut self, title: String) -> Self {
        self.content.push(
            Element::Element(HtmlElement::new(
                TagType::Title,
                HtmlElementConfig::new_empty(),
            )) + Element::Text(title),
        );
        self
    }

    /// Adds a style tag to the html page.
    ///
    /// - `content` : The css code.
    pub fn with_style(mut self, content: String) -> Self {
        self.content.push(
            Element::Element(HtmlElement::new(
                TagType::Style,
                HtmlElementConfig::new_empty(),
            )) + Element::Text(content),
        );
        self
    }

    /// Adds a base tag to the html page.
    ///
    /// - `url` : The base url.
    /// - `target` : The optional base url's target. If the value is None, this attribute is ignored.  
    pub fn with_base_url(mut self, url: String, target: Option<BaseUrlTarget>) -> Self {
        let mut configs =
            HtmlElementConfig::new_empty().set_attribute("href".to_string(), Some(url));
        if let Some(target) = target {
            configs = configs.set_attribute("target".to_string(), Some(target.to_string()))
        }
        self.content
            .push(Element::Element(HtmlElement::new(TagType::Base, configs)));
        self
    }

    /// Adds an ico icon to the page.
    ///
    /// - `href` : The icon's path.
    pub fn with_ico_icon(mut self, href: String) -> Self {
        self.content.push(Element::Element(HtmlElement::new(
            TagType::Link,
            HtmlElementConfig::new_empty()
                .set_attribute("rel".to_string(), Some("icon".to_string()))
                .set_attribute("type".to_string(), Some("image/x-icon".to_string()))
                .set_attribute("href".to_string(), Some(href)),
        )));
        self
    }

    /// Adds a css file to the page.
    ///
    /// - `href` : The file's path.
    pub fn with_css_file(mut self, href: String) -> Self {
        self.content.push(Element::Element(HtmlElement::new(
            TagType::Link,
            HtmlElementConfig::new_empty()
                .set_attribute("rel".to_string(), Some("stylesheet".to_string()))
                .set_attribute("href".to_string(), Some(href)),
        )));
        self
    }

    /// Adds the `charset` meta tag to the html page.
    ///
    /// - `charset` : The charset.
    pub fn with_charset_meta(mut self, charset: String) -> Self {
        self.content.push(Element::Element(HtmlElement::new(
            TagType::Meta,
            HtmlElementConfig::new_empty().set_attribute("charset".to_string(), Some(charset)),
        )));
        self
    }

    /// Adds the meta tag to the html page.
    ///
    /// - `name` : The meta's name.
    /// - `content` : The meta's content.
    pub fn with_meta(mut self, name: String, content: String) -> Self {
        self.content.push(Element::Element(HtmlElement::new(
            TagType::Meta,
            HtmlElementConfig::new_empty()
                .set_attribute("name".to_string(), Some(name))
                .set_attribute("content".to_string(), Some(content)),
        )));
        self
    }

    /// Adds a script tag to the html page with raw javascript.
    ///
    /// - `content` : The script raw javascript content.
    /// - `loading_mode` : The optional script loading mode. If the value is None, this attribute is ignored.  
    pub fn with_raw_javascript(
        mut self,
        content: String,
        loading_mode: Option<ScriptLoadMode>,
    ) -> Self {
        let mut configs = HtmlElementConfig::new_empty();
        if let Some(loading_mode) = loading_mode {
            configs = configs.set_attribute(loading_mode.to_string(), None);
        }
        self.content.push(
            Element::Element(HtmlElement::new(TagType::Script, configs)) + Element::Text(content),
        );
        self
    }

    /// Adds a javascript file to the html page.
    ///
    /// - `src` : The file's path.
    /// - `loading_mode` : The optional script loading mode. If the value is None, this attribute is ignored.  
    pub fn with_javascript_file(
        mut self,
        src: String,
        loading_mode: Option<ScriptLoadMode>,
    ) -> Self {
        let mut configs =
            HtmlElementConfig::new_empty().set_attribute("src".to_string(), Some(src));
        if let Some(loading_mode) = loading_mode {
            configs = configs.set_attribute(loading_mode.to_string(), None);
        }
        self.content
            .push(Element::Element(HtmlElement::new(TagType::Script, configs)));
        self
    }

    /// Adds a no script tag to the html page.
    ///
    /// - `content` : The no script's content.
    pub fn with_no_script(mut self, content: String) -> Self {
        self.content.push(
            Element::Element(HtmlElement::new(
                TagType::NoScript,
                HtmlElementConfig::new_empty(),
            )) + Element::Text(content),
        );
        self
    }
}

impl Default for Head {
    fn default() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}
