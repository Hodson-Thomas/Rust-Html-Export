#[macro_export]
/// Creates an element with any HtmlElementConfig parameters combination :
/// - id
/// - classes
/// - attributes
/// - id, classes
/// - classes, id
/// - id, attributes
/// - attributes, id,
/// - classes, attributes
/// - attributes, classes
/// - id, classes, attributes
/// - id, attributes, classes
/// - classes, id, attributes
/// - classes, attributes, id
/// - attributes, id, classes
/// - attributes, classes, id
macro_rules! elem {
    (tag = $tag:expr) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new($tag, html_export::element::HtmlElementConfig::new_empty()))
    };

    (tag = $tag:expr, id = $id:expr) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new($tag, html_export::element::HtmlElementConfig::new_empty().with_id($id.to_string())))
    };

    (tag = $tag:expr, classes = [$( $class:expr ),*]) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new($tag, html_export::element::HtmlElementConfig::new_empty().set_classes({
            let mut set = std::collections::HashSet::new();
            $(set.insert($class.to_string());)*
            set.into_iter()
        })))
    };

    (tag = $tag:expr, attributes = {$( $key:expr => $value:expr ),*}) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new($tag, html_export::element::HtmlElementConfig::new_empty().set_attributes({
            let mut map = std::collections::HashMap::new();
            $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
            map
        })))
    };

    (tag = $tag:expr, id = $id:expr, classes = [$( $class:expr ),*]) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
        ))
    };

    (tag = $tag:expr, classes = [$( $class:expr ),*], id = $id:expr) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
        ))
    };

    (tag = $tag:expr, id = $id:expr, attributes = {$( $key:expr => $value:expr ),*}) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, attributes = {$( $key:expr => $value:expr ),*}, id = $id:expr) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, classes = [$( $class:expr ),*], attributes = {$( $key:expr => $value:expr ),*}) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, attributes = {$( $key:expr => $value:expr ),*}, classes = [$( $class:expr ),*]) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, id = $id:expr, classes = [$( $class:expr ),*], attributes = {$( $key:expr => $value:expr ),*}) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, id = $id:expr, attributes = {$( $key:expr => $value:expr ),*}, classes = [$( $class:expr ),*]) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, classes = [$( $class:expr ),*], id = $id:expr, attributes = {$( $key:expr => $value:expr ),*}) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, classes = [$( $class:expr ),*], attributes = {$( $key:expr => $value:expr ),*}, id = $id:expr) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, attributes = {$( $key:expr => $value:expr ),*}, id = $id:expr, classes = [$( $class:expr ),*]) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };

    (tag = $tag:expr, attributes = {$( $key:expr => $value:expr ),*}, classes = [$( $class:expr ),*], id = $id:expr) => {
        html_export::element::Element::Element(html_export::element::HtmlElement::new(
            $tag,
            html_export::element::HtmlElementConfig::new_empty()
                .with_id($id.to_string())
                .set_classes({
                    let mut set = std::collections::HashSet::new();
                    $(set.insert($class.to_string());)*
                    set.into_iter()
                })
                .set_attributes({
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($key.to_string(), $value.map(|v| v.to_string()));)*
                    map
                })
        ))
    };
}

#[macro_export]
/// Creates a raw text element.
macro_rules! text {
    ($content:expr) => {
        html_export::element::Element::Text($content.to_string())
    };
}

#[macro_export]
/// Creates a new `P` element. Same behavior as the `elem!` macro.
macro_rules! p {
    () => { elem!(tag = TagType::P) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::P, $( $param )* ) };
}

#[macro_export]
/// Creates a new `H1` element. Same behavior as the `elem!` macro.
macro_rules! h1 {
    () => { elem!(tag = TagType::H1) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::H1, $( $param )* ) };
}

#[macro_export]
/// Creates a new `H2` element. Same behavior as the `elem!` macro.
macro_rules! h2 {
    () => { elem!(tag = TagType::H2) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::H2, $( $param )* ) };
}

#[macro_export]
/// Creates a new `H3` element. Same behavior as the `elem!` macro.
macro_rules! h3 {
    () => { elem!(tag = TagType::H3) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::H3, $( $param )* ) };
}

#[macro_export]
/// Creates a new `H4` element. Same behavior as the `elem!` macro.
macro_rules! h4 {
    () => { elem!(tag = TagType::H4) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::H4, $( $param )* ) };
}

#[macro_export]
/// Creates a new `H5` element. Same behavior as the `elem!` macro.
macro_rules! h5 {
    () => { elem!(tag = TagType::H5) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::H5, $( $param )* ) };
}

#[macro_export]
/// Creates a new `H6` element. Same behavior as the `elem!` macro.
macro_rules! h6 {
    () => { elem!(tag = TagType::H6) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::H6, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Abbr` element. Same behavior as the `elem!` macro.
macro_rules! abbr {
    () => { elem!(tag = TagType::Abbr) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Abbr, $( $param )* ) };
}

#[macro_export]
/// Creates a new `B` element. Same behavior as the `elem!` macro.
macro_rules! b {
    () => { elem!(tag = TagType::B) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::B, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Bdi` element. Same behavior as the `elem!` macro.
macro_rules! bdi {
    () => { elem!(tag = TagType::Bdi) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Bdi, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Bdo` element. Same behavior as the `elem!` macro.
macro_rules! bdo {
    () => { elem!(tag = TagType::Bdo) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Bdo, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Blockquote` element. Same behavior as the `elem!` macro.
macro_rules! blockquote {
    () => { elem!(tag = TagType::Blockquote) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Blockquote, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Cite` element. Same behavior as the `elem!` macro.
macro_rules! cite {
    () => { elem!(tag = TagType::Cite) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Cite, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Del` element. Same behavior as the `elem!` macro.
macro_rules! del {
    () => { elem!(tag = TagType::Del) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Del, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Dfn` element. Same behavior as the `elem!` macro.
macro_rules! dfn {
    () => { elem!(tag = TagType::Dfn) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Dfn, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Em` element. Same behavior as the `elem!` macro.
macro_rules! em {
    () => { elem!(tag = TagType::Em) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Em, $( $param )* ) };
}

#[macro_export]
/// Creates a new `I` element. Same behavior as the `elem!` macro.
macro_rules! i {
    () => { elem!(tag = TagType::I) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::I, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Ins` element. Same behavior as the `elem!` macro.
macro_rules! ins {
    () => { elem!(tag = TagType::Ins) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Ins, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Mark` element. Same behavior as the `elem!` macro.
macro_rules! mark {
    () => { elem!(tag = TagType::Mark) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Mark, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Q` element. Same behavior as the `elem!` macro.
macro_rules! q {
    () => { elem!(tag = TagType::Q) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Q, $( $param )* ) };
}

#[macro_export]
/// Creates a new `S` element. Same behavior as the `elem!` macro.
macro_rules! s {
    () => { elem!(tag = TagType::S) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::S, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Small` element. Same behavior as the `elem!` macro.
macro_rules! small {
    () => { elem!(tag = TagType::Small) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Small, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Strong` element. Same behavior as the `elem!` macro.
macro_rules! strong {
    () => { elem!(tag = TagType::Strong) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Strong, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Sub` element. Same behavior as the `elem!` macro.
macro_rules! sub {
    () => { elem!(tag = TagType::Sub) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Sub, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Sup` element. Same behavior as the `elem!` macro.
macro_rules! sup {
    () => { elem!(tag = TagType::Sup) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Sup, $( $param )* ) };
}

#[macro_export]
/// Creates a new `U` element. Same behavior as the `elem!` macro.
macro_rules! u {
    () => { elem!(tag = TagType::U) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::U, $( $param )* ) };
}

#[macro_export]
/// Creates a new `A` element. Same behavior as the `elem!` macro.
macro_rules! a {
    () => { elem!(tag = TagType::A) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::A, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Link` element. Same behavior as the `elem!` macro.
macro_rules! link {
    () => { elem!(tag = TagType::Link) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Link, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Menu` element. Same behavior as the `elem!` macro.
macro_rules! menu {
    () => { elem!(tag = TagType::Menu) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Menu, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Ul` element. Same behavior as the `elem!` macro.
macro_rules! ul {
    () => { elem!(tag = TagType::Ul) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Ul, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Ol` element. Same behavior as the `elem!` macro.
macro_rules! ol {
    () => { elem!(tag = TagType::Ol) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Ol, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Li` element. Same behavior as the `elem!` macro.
macro_rules! li {
    () => { elem!(tag = TagType::Li) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Li, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Dl` element. Same behavior as the `elem!` macro.
macro_rules! dl {
    () => { elem!(tag = TagType::Dl) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Dl, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Dt` element. Same behavior as the `elem!` macro.
macro_rules! dt {
    () => { elem!(tag = TagType::Dt) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Dt, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Dd` element. Same behavior as the `elem!` macro.
macro_rules! dd {
    () => { elem!(tag = TagType::Dd) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Dd, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Table` element. Same behavior as the `elem!` macro.
macro_rules! table {
    () => { elem!(tag = TagType::Table) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Table, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Caption` element. Same behavior as the `elem!` macro.
macro_rules! caption {
    () => { elem!(tag = TagType::Caption) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Caption, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Th` element. Same behavior as the `elem!` macro.
macro_rules! th {
    () => { elem!(tag = TagType::Th) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Th, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Tr` element. Same behavior as the `elem!` macro.
macro_rules! tr {
    () => { elem!(tag = TagType::Tr) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Tr, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Td` element. Same behavior as the `elem!` macro.
macro_rules! td {
    () => { elem!(tag = TagType::Td) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Td, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Thead` element. Same behavior as the `elem!` macro.
macro_rules! thead {
    () => { elem!(tag = TagType::Thead) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Thead, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Tbody` element. Same behavior as the `elem!` macro.
macro_rules! tbody {
    () => { elem!(tag = TagType::Tbody) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Tbody, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Tfoot` element. Same behavior as the `elem!` macro.
macro_rules! tfoot {
    () => { elem!(tag = TagType::Tfoot) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Tfoot, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Col` element. Same behavior as the `elem!` macro.
macro_rules! col {
    () => { elem!(tag = TagType::Col) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Col, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Colgroup` element. Same behavior as the `elem!` macro.
macro_rules! colgroup {
    () => { elem!(tag = TagType::Colgroup) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Colgroup, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Template` element. Same behavior as the `elem!` macro.
macro_rules! template {
    () => { elem!(tag = TagType::Template) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Template, $( $param )* ) };
}

#[macro_export]
/// Creates a new `IFrame` element. Same behavior as the `elem!` macro.
macro_rules! iframe {
    () => { elem!(tag = TagType::IFrame) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::IFrame, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Nav` element. Same behavior as the `elem!` macro.
macro_rules! nav {
    () => { elem!(tag = TagType::Nav) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Nav, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Div` element. Same behavior as the `elem!` macro.
macro_rules! div {
    () => { elem!(tag = TagType::Div) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Div, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Span` element. Same behavior as the `elem!` macro.
macro_rules! span {
    () => { elem!(tag = TagType::Span) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Span, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Header` element. Same behavior as the `elem!` macro.
macro_rules! header {
    () => { elem!(tag = TagType::Header) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Header, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Hgroup` element. Same behavior as the `elem!` macro.
macro_rules! hgroup {
    () => { elem!(tag = TagType::Hgroup) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Hgroup, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Footer` element. Same behavior as the `elem!` macro.
macro_rules! footer {
    () => { elem!(tag = TagType::Footer) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Footer, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Main` element. Same behavior as the `elem!` macro.
macro_rules! main {
    () => { elem!(tag = TagType::Main) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Main, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Section` element. Same behavior as the `elem!` macro.
macro_rules! section {
    () => { elem!(tag = TagType::Section) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Section, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Search` element. Same behavior as the `elem!` macro.
macro_rules! search {
    () => { elem!(tag = TagType::Search) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Search, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Article` element. Same behavior as the `elem!` macro.
macro_rules! article {
    () => { elem!(tag = TagType::Article) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Article, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Aside` element. Same behavior as the `elem!` macro.
macro_rules! aside {
    () => { elem!(tag = TagType::Aside) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Aside, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Details` element. Same behavior as the `elem!` macro.
macro_rules! details {
    () => { elem!(tag = TagType::Details) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Details, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Dialog` element. Same behavior as the `elem!` macro.
macro_rules! dialog {
    () => { elem!(tag = TagType::Dialog) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Dialog, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Summary` element. Same behavior as the `elem!` macro.
macro_rules! summary {
    () => { elem!(tag = TagType::Summary) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Summary, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Data` element. Same behavior as the `elem!` macro.
macro_rules! data {
    () => { elem!(tag = TagType::Data) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Data, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Code` element. Same behavior as the `elem!` macro.
macro_rules! code {
    () => { elem!(tag = TagType::Code) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Code, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Pre` element. Same behavior as the `elem!` macro.
macro_rules! pre {
    () => { elem!(tag = TagType::Pre) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Pre, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Samp` element. Same behavior as the `elem!` macro.
macro_rules! samp {
    () => { elem!(tag = TagType::Samp) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Samp, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Form` element. Same behavior as the `elem!` macro.
macro_rules! form {
    () => { elem!(tag = TagType::Form) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Form, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Input` element. Same behavior as the `elem!` macro.
macro_rules! input {
    () => { elem!(tag = TagType::Input) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Input, $( $param )* ) };
}

#[macro_export]
/// Creates a new `TextArea` element. Same behavior as the `elem!` macro.
macro_rules! text_area {
    () => { elem!(tag = TagType::TextArea) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::TextArea, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Button` element. Same behavior as the `elem!` macro.
macro_rules! button {
    () => { elem!(tag = TagType::Button) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Button, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Select` element. Same behavior as the `elem!` macro.
macro_rules! select {
    () => { elem!(tag = TagType::Select) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Select, $( $param )* ) };
}

#[macro_export]
/// Creates a new `OptGroup` element. Same behavior as the `elem!` macro.
macro_rules! opt_group {
    () => { elem!(tag = TagType::OptGroup) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::OptGroup, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Option` element. Same behavior as the `elem!` macro.
macro_rules! option {
    () => { elem!(tag = TagType::Option) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Option, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Label` element. Same behavior as the `elem!` macro.
macro_rules! label {
    () => { elem!(tag = TagType::Label) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Label, $( $param )* ) };
}

#[macro_export]
/// Creates a new `FieldSet` element. Same behavior as the `elem!` macro.
macro_rules! field_set {
    () => { elem!(tag = TagType::FieldSet) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::FieldSet, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Legend` element. Same behavior as the `elem!` macro.
macro_rules! legend {
    () => { elem!(tag = TagType::Legend) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Legend, $( $param )* ) };
}

#[macro_export]
/// Creates a new `DataList` element. Same behavior as the `elem!` macro.
macro_rules! data_list {
    () => { elem!(tag = TagType::DataList) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::DataList, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Output` element. Same behavior as the `elem!` macro.
macro_rules! output {
    () => { elem!(tag = TagType::Output) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Output, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Img` element. Same behavior as the `elem!` macro.
macro_rules! img {
    () => { elem!(tag = TagType::Img) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Img, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Map` element. Same behavior as the `elem!` macro.
macro_rules! map {
    () => { elem!(tag = TagType::Map) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Map, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Area` element. Same behavior as the `elem!` macro.
macro_rules! area {
    () => { elem!(tag = TagType::Area) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Area, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Canvas` element. Same behavior as the `elem!` macro.
macro_rules! canvas {
    () => { elem!(tag = TagType::Canvas) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Canvas, $( $param )* ) };
}

#[macro_export]
/// Creates a new `FigCaption` element. Same behavior as the `elem!` macro.
macro_rules! fig_caption {
    () => { elem!(tag = TagType::FigCaption) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::FigCaption, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Figure` element. Same behavior as the `elem!` macro.
macro_rules! figure {
    () => { elem!(tag = TagType::Figure) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Figure, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Picture` element. Same behavior as the `elem!` macro.
macro_rules! picture {
    () => { elem!(tag = TagType::Picture) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Picture, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Svg` element. Same behavior as the `elem!` macro.
macro_rules! svg {
    () => { elem!(tag = TagType::Svg) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Svg, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Audio` element. Same behavior as the `elem!` macro.
macro_rules! audio {
    () => { elem!(tag = TagType::Audio) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Audio, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Source` element. Same behavior as the `elem!` macro.
macro_rules! source {
    () => { elem!(tag = TagType::Source) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Source, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Track` element. Same behavior as the `elem!` macro.
macro_rules! track {
    () => { elem!(tag = TagType::Track) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Track, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Video` element. Same behavior as the `elem!` macro.
macro_rules! video {
    () => { elem!(tag = TagType::Video) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Video, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Wbr` element. Same behavior as the `elem!` macro.
macro_rules! wbr {
    () => { elem!(tag = TagType::Wbr) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Wbr, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Br` element. Same behavior as the `elem!` macro.
macro_rules! br {
    () => { elem!(tag = TagType::Br) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Br, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Hr` element. Same behavior as the `elem!` macro.
macro_rules! hr {
    () => { elem!(tag = TagType::Hr) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Hr, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Comment` element. Same behavior as the `elem!` macro.
macro_rules! comment {
    () => { elem!(tag = TagType::Comment) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Comment, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Address` element. Same behavior as the `elem!` macro.
macro_rules! address {
    () => { elem!(tag = TagType::Address) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Address, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Kdb` element. Same behavior as the `elem!` macro.
macro_rules! kdb {
    () => { elem!(tag = TagType::Kdb) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Kdb, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Meter` element. Same behavior as the `elem!` macro.
macro_rules! meter {
    () => { elem!(tag = TagType::Meter) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Meter, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Progress` element. Same behavior as the `elem!` macro.
macro_rules! progress {
    () => { elem!(tag = TagType::Progress) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Progress, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Time` element. Same behavior as the `elem!` macro.
macro_rules! time {
    () => { elem!(tag = TagType::Time) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Time, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Var` element. Same behavior as the `elem!` macro.
macro_rules! var {
    () => { elem!(tag = TagType::Var) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Var, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Script` element. Same behavior as the `elem!` macro.
macro_rules! script {
    () => { elem!(tag = TagType::Script) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Script, $( $param )* ) };
}

#[macro_export]
/// Creates a new `NoScript` element. Same behavior as the `elem!` macro.
macro_rules! no_script {
    () => { elem!(tag = TagType::NoScript) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::NoScript, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Embed` element. Same behavior as the `elem!` macro.
macro_rules! embed {
    () => { elem!(tag = TagType::Embed) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Embed, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Object` element. Same behavior as the `elem!` macro.
macro_rules! object {
    () => { elem!(tag = TagType::Object) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Object, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Param` element. Same behavior as the `elem!` macro.
macro_rules! param {
    () => { elem!(tag = TagType::Param) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Param, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Meta` element. Same behavior as the `elem!` macro.
macro_rules! meta {
    () => { elem!(tag = TagType::Meta) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Meta, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Base` element. Same behavior as the `elem!` macro.
macro_rules! base {
    () => { elem!(tag = TagType::Base) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Base, $( $param )* ) };
}

#[macro_export]
/// Creates a new `Head` element. Same behavior as the `elem!` macro.
macro_rules! head {
    () => { elem!(tag = TagType::Head) };
    ( $( $param:tt )* ) => { elem!(tag = TagType::Head, $( $param )* ) };
}
