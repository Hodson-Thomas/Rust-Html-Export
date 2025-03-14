use crate::element::{Element, HtmlElementConfig};

use super::field::{create_input, create_labeled_input, AsHtmlConfig, InputFieldConfig};

/// Defines the file input configs.
///
/// - `accept` : Lists the authorized file extensions. If the value is None, this attribute is ignored.
/// - `capture` : Defines the capture tool. If the value is None, this attribute is ignored.
/// - `mutltiple` : Indicates if the input can accept multiple files. If the value is None, this attribute is ignored.
pub struct FileInputConfig {
    pub accept: Option<String>,
    pub capture: Option<String>,
    pub multiple: Option<bool>,
}

/// Creates a `<label>` and `<input type="file">` html structure.
///
/// - `html_configs` : The label + input html configs.
/// - `input_configs` : The email input's configs.
pub fn create_labeled_file_input(
    html_configs: InputFieldConfig,
    input_configs: FileInputConfig,
) -> Element {
    create_labeled_input(html_configs, "file".to_string(), input_configs, None)
}

/// Creates an `<input type="file">` html element.
///
/// - `html_configs` : The label + input html configs.
/// - `input_configs` : The email input's configs.
pub fn create_file_input(
    html_configs: InputFieldConfig,
    input_configs: FileInputConfig,
) -> Element {
    create_input(html_configs, "file".to_string(), input_configs, None)
}

impl FileInputConfig {
    /// Creates a default object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a new accept parameter.
    ///
    /// - `accept` : The new accept parameter.
    pub fn with_accept(mut self, accept: String) -> Self {
        self.accept = Some(accept);
        self
    }

    /// Removes the accept parameter.
    pub fn without_accept(mut self) -> Self {
        self.accept = None;
        self
    }

    /// Sets a new capture parameter.
    ///
    /// - `capture` : The new capture parameter.
    pub fn with_capture(mut self, capture: String) -> Self {
        self.capture = Some(capture);
        self
    }

    /// Removes the capture parameter.
    pub fn without_capture(mut self) -> Self {
        self.capture = None;
        self
    }

    /// Sets a new multiple parameter.
    ///
    /// - `state` : The new multiple state.
    pub fn with_multiple(mut self, state: bool) -> Self {
        self.multiple = Some(state);
        self
    }

    /// Removes the multiple parameter.
    pub fn without_multiple(mut self) -> Self {
        self.multiple = None;
        self
    }
}

impl AsHtmlConfig for FileInputConfig {
    fn set_html_configs(&self, mut configs: HtmlElementConfig) -> HtmlElementConfig {
        if self.accept.is_some() {
            configs = configs.set_attribute("accept".to_string(), self.accept.clone());
        }
        if self.capture.is_some() {
            configs = configs.set_attribute("capture".to_string(), self.capture.clone());
        }
        if let Some(multiple) = self.multiple {
            if multiple {
                configs = configs.set_attribute("multiple".to_string(), None);
            }
        }
        configs
    }
}

impl Default for FileInputConfig {
    fn default() -> Self {
        Self {
            accept: None,
            capture: None,
            multiple: None,
        }
    }
}
