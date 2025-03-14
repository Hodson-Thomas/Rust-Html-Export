use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, AsHtmlConfig, InputFieldConfig};

/// Defines the submit input's configs.
///
/// - `formaction` : The form url submission. If the value is None this attribute is ignored.
/// - `formenctype` : The form's encoding method. If the value is None this attribute is ignored.
/// - `formmethod` : The form's submit method. If the value is None this attribute is ignored.
/// - `formnovalidate` : Indicates if the form must not be validated before submission.
/// - `formtarget` : Indicates how to display the server's response.
pub struct SubmitInputConfigs {
    pub formaction: Option<String>,
    pub formenctype: Option<String>,
    pub formmethod: Option<String>,
    pub formnovalidate: Option<bool>,
    pub formtarget: Option<String>,
}

/// Creates an `<input type="submit">` html element.
///
/// - `html_configs` : The input's html configs.
/// - `input_configs` : The input's configs.
pub fn create_submit_input(
    html_configs: InputFieldConfig,
    input_configs: SubmitInputConfigs,
) -> Element {
    create_input(html_configs, "submit".to_string(), input_configs, None)
}

impl SubmitInputConfigs {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
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

impl AsHtmlConfig for SubmitInputConfigs {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
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
        configs
    }
}

impl Default for SubmitInputConfigs {
    fn default() -> Self {
        Self {
            formaction: None,
            formenctype: None,
            formmethod: None,
            formnovalidate: None,
            formtarget: None,
        }
    }
}
