use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, AsHtmlConfig, InputFieldConfig};

/// Defines the image input's configs.
///
/// - `alt` : The text to display if the image can not be loaded. If the value is None this attribute is ignored.
/// - `width` : The input's image width. If the value is None this attribute is ignored.
/// - `height` : The input's image height. If the value is None this attribute is ignored.
/// - `formaction` : The form url submission. If the value is None this attribute is ignored.
/// - `formenctype` : The form's encoding method. If the value is None this attribute is ignored.
/// - `formmethod` : The form's submit method. If the value is None this attribute is ignored.
/// - `formnovalidate` : Indicates if the form must not be validated before submission.
/// - `formtarget` : Indicates how to display the server's response.
pub struct ImageInputConfigs {
    pub alt: Option<String>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub formaction: Option<String>,
    pub formenctype: Option<String>,
    pub formmethod: Option<String>,
    pub formnovalidate: Option<bool>,
    pub formtarget: Option<String>,
}

/// Creates an `<input type="image">` html element.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
pub fn create_image_input(
    html_configs: InputFieldConfig,
    input_configs: ImageInputConfigs,
) -> Element {
    create_input(html_configs, "image".to_string(), input_configs, None)
}

impl ImageInputConfigs {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new alt parameter.
    ///
    /// - `alt` : The new alt parameter.
    pub fn with_alt(mut self, alt: String) -> Self {
        self.alt = Some(alt);
        self
    }

    /// Removes the alt parameter.
    pub fn without_alt(mut self) -> Self {
        self.alt = None;
        self
    }

    /// Sets the new width parameter.
    ///
    /// - `width` : The new width parameter.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Removes the width parameter.
    pub fn without_width(mut self) -> Self {
        self.width = None;
        self
    }

    /// Sets the new height parameter.
    ///
    /// - `height` : The new height parameter.
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Removes the height parameter.
    pub fn without_height(mut self) -> Self {
        self.height = None;
        self
    }

    /// Sets the new formaction parameter.
    ///
    /// - `formaction` : The new formaction parameter.
    pub fn with_formaction(mut self, formaction: String) -> Self {
        self.formaction = Some(formaction);
        self
    }

    /// Removes the formaction parameter.
    pub fn without_formaction(mut self) -> Self {
        self.formaction = None;
        self
    }

    /// Sets the new formenctype parameter.
    ///
    /// - `formenctype` : The new formenctype parameter.
    pub fn with_formenctype(mut self, formenctype: String) -> Self {
        self.formenctype = Some(formenctype);
        self
    }

    /// Removes the formenctype parameter.
    pub fn without_formenctype(mut self) -> Self {
        self.formenctype = None;
        self
    }

    /// Sets the new formmethod parameter.
    ///
    /// - `formmethod` : The new formmethod parameter.
    pub fn with_formmethod(mut self, formmethod: String) -> Self {
        self.formmethod = Some(formmethod);
        self
    }

    /// Removes the formmethod parameter.
    pub fn without_formmethod(mut self) -> Self {
        self.formmethod = None;
        self
    }

    /// Sets the new formnovalidate parameter.
    ///
    /// - `formnovalidate` : The new formnovalidate parameter.
    pub fn with_formnovalidate(mut self, formnovalidate: bool) -> Self {
        self.formnovalidate = Some(formnovalidate);
        self
    }

    /// Removes the formnovalidate parameter.
    pub fn without_formnovalidate(mut self) -> Self {
        self.formnovalidate = None;
        self
    }

    /// Sets the new formtarget parameter.
    ///
    /// - `formtarget` : The new formtarget parameter.
    pub fn with_formtarget(mut self, formtarget: String) -> Self {
        self.formtarget = Some(formtarget);
        self
    }

    /// Removes the formtarget parameter.
    pub fn without_formtarget(mut self) -> Self {
        self.formtarget = None;
        self
    }
}

impl AsHtmlConfig for ImageInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.alt.is_some() {
            configs = configs.set_attribute("alt".to_string(), self.alt.clone())
        }
        if self.formaction.is_some() {
            configs = configs.set_attribute("formaction".to_string(), self.formaction.clone());
        }
        if self.formenctype.is_some() {
            configs = configs.set_attribute("formenctype".to_string(), self.formenctype.clone());
        }
        if self.formmethod.is_some() {
            configs = configs.set_attribute("formmethod".to_string(), self.formmethod.clone());
        }
        if let Some(formnovalidate) = self.formnovalidate {
            if formnovalidate {
                configs = configs.set_attribute("formnovalidate".to_string(), None);
            }
        }
        if self.formtarget.is_some() {
            configs = configs.set_attribute("formtarget".to_string(), self.formtarget.clone());
        }
        if let Some(width) = self.width {
            configs = configs.set_attribute("width".to_string(), Some(width.to_string()));
        }
        if let Some(height) = self.height {
            configs = configs.set_attribute("height".to_string(), Some(height.to_string()));
        }

        configs
    }
}

impl Default for ImageInputConfigs {
    fn default() -> Self {
        Self {
            alt: None,
            width: None,
            height: None,
            formaction: None,
            formenctype: None,
            formmethod: None,
            formnovalidate: None,
            formtarget: None,
        }
    }
}
