use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use head::Head;
use html::Html;

pub mod composed;
pub mod element;
pub mod errors;
pub mod head;
pub mod html;
pub mod miscellaneous;
pub mod prebuild;
pub mod tags;

/// Exports the elements to an html file.
///
/// - `destination` : The destination folder's path.
/// - `filename` : The html file's name.
/// - `head` : The html document's head.
/// - `elements` : The elements to export.
pub fn export_to_file<T>(
    destination: String,
    filename: String,
    head: Head,
    elements: Vec<T>,
) -> std::io::Result<()>
where
    T: html::Html,
{
    fs::create_dir_all(destination.clone())?;
    let file = File::create(format!("{}/{}", destination, filename))?;
    let mut writer = BufWriter::new(file);
    writer.write(
        r#"
<!DOCTYPE html>
<html>
    <head>
"#
        .as_bytes(),
    )?;
    for head_tag in head.content {
        writer = head_tag.write_html(writer, 2)?;
    }
    writer.write(b"\t</head>\n\t<body>\n")?;
    for element in elements {
        writer = element.write_html(writer, 2)?;
    }
    writer.write(
        r#"
    </body>
</html>
"#
        .as_bytes(),
    )?;
    Ok(())
}
