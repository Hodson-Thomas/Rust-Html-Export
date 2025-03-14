use std::{
    fs::File,
    io::{self, BufWriter},
};

use crate::element::Element;

/// Defines Html export behavior.
pub trait Html {
    fn write_html(&self, writer: BufWriter<File>, tabs: usize) -> io::Result<BufWriter<File>>;
}

/// Defines Html conversion behavior.
pub trait ToHtml {
    fn to_html(&self) -> Element;
}
