use std::fmt::Display;

use crate::{
    element::{Element, HtmlElement},
    tags::TagType,
};

pub type WrapResult = Result<Element, WrapElementError>;
pub type CreateElementResult = Result<HtmlElement, ElementError>;

#[derive(Debug)]
/// Defines Html element's related errors.
pub enum ElementError {
    /// Error when adding a sub-element to Html elements.
    InvalidContent(TagType),
    /// Error when indexing sub-element to Html elements.
    InvalidIndex(usize, usize),
}

#[derive(Debug)]
/// Defines Html wrapping error.
pub enum WrapElementError {
    /// Error when wrapping with a no-child Html element.
    NonWrapperElement(TagType),
    /// Error when wrapping multiple Html elements wwith a single-child Html element.
    SingleChildExceeded(TagType),
    /// Error when wrapping a sub-element at an invalid index.
    InvalidIndex(usize, usize),
}

#[derive(Debug)]
/// Defines the radio creation errors
pub enum FieldCreationError {
    /// Error when defining which element is checked. The given index is invalid.
    SetCheckedInvalidIndex(usize, usize),
}

#[derive(Debug)]
/// The `<head>` tag errors.
pub enum HeadCreationError {
    /// Error when adding an html incompatible hmlt tag.
    InvalidContent(Element),
}

impl Display for WrapElementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrapElementError::NonWrapperElement(tag_type) => {
                writeln!(
                    f,
                    "Element Tag=<{}> can not wrap another element.",
                    tag_type.get_tag_name()
                )
            }
            WrapElementError::SingleChildExceeded(tag_type) => {
                writeln!(
                    f,
                    "Element Tag=<{}> can have at most 1 child element.",
                    tag_type.get_tag_name()
                )
            }
            WrapElementError::InvalidIndex(index, size) => {
                writeln!(
                    f,
                    "Attempted to insert a child element at position {} but the container had {} child-elements.",
                    index,
                    size
                )
            }
        }
    }
}

impl Display for ElementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementError::InvalidContent(tag_type) => {
                if !tag_type.can_wrap() {
                    write!(
                        f,
                        "The element <{}> can not have sub-elements.",
                        tag_type.get_tag_name()
                    )
                } else if tag_type.is_single_child() {
                    write!(
                        f,
                        "The element <{}> can have at most 1 sub-elements.",
                        tag_type.get_tag_name()
                    )
                } else {
                    write!(
                        f,
                        "Unknown behavior occured while adding a sub-element to <{}> element.",
                        tag_type.get_tag_name()
                    )
                }
            }
            ElementError::InvalidIndex(index, size) => write!(
                f,
                "Could not remove element at index {} from the collection of size {}.",
                index, size
            ),
        }
    }
}

impl Display for FieldCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldCreationError::SetCheckedInvalidIndex(index, size) => write!(
                f,
                "Can not set element checked at index {} for a list of size {}",
                index, size
            ),
        }
    }
}

impl Display for HeadCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadCreationError::InvalidContent(element) => {
                write!(
                    f,
                    "Can not add element {:?} in an <head> html tag.",
                    element
                )
            }
        }
    }
}
