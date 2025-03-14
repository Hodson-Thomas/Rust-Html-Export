#[derive(Clone, Debug)]
/// Defines all Html tags.
pub enum TagType {
    /* Text */
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Title,
    Abbr,
    B,
    Bdi,
    Bdo,
    Blockquote,
    Cite,
    Del,
    Dfn,
    Em,
    I,
    Ins,
    Mark,
    Q,
    Rp,
    Ruby,
    Rt,
    S,
    Small,
    Strong,
    Sub,
    Sup,
    U,

    /* Links */
    A,
    Link,

    /* Lists */
    Menu,
    Ul,
    Ol,
    Li,
    Dl,
    Dt,
    Dd,

    /* Tables */
    Table,
    Caption,
    Th,
    Tr,
    Td,
    Thead,
    Tbody,
    Tfoot,
    Col,
    Colgroup,

    /* Containers */
    Template,
    IFrame,
    Nav,
    Div,
    Span,
    Header,
    Hgroup,
    Footer,
    Main,
    Section,
    Search,
    Article,
    Aside,
    Details,
    Dialog,
    Summary,
    Data,

    /* Raw */
    Code,
    Pre,
    Samp,

    /* Forms */
    Form,
    Input,
    TextArea,
    Button,
    Select,
    OptGroup,
    Option,
    Label,
    FieldSet,
    Legend,
    DataList,
    Output,

    /* Display */
    Img,
    Map,
    Area,
    Canvas,
    FigCaption,
    Figure,
    Picture,
    Svg,
    Audio,
    Source,
    Track,
    Video,

    /* Miscellaneous */
    Wbr,
    Br,
    Hr,
    Comment,
    Address,
    Kdb,
    Meter,
    Progress,
    Time,
    Var,
    Script,
    NoScript,
    Embed,
    Object,
    Param,
    Meta,
    Base,
    Head,
    Style,
}

impl TagType {
    /// Returns the Html tag name.
    pub fn get_tag_name(&self) -> String {
        match self {
            TagType::P => "p",
            TagType::H1 => "h1",
            TagType::H2 => "h2",
            TagType::H3 => "h3",
            TagType::H4 => "h4",
            TagType::H5 => "h5",
            TagType::H6 => "h6",
            TagType::Title => "title",
            TagType::Abbr => "abbr",
            TagType::B => "b",
            TagType::Bdi => "bdi",
            TagType::Bdo => "bdo",
            TagType::Blockquote => "blockquote",
            TagType::Cite => "cite",
            TagType::Del => "del",
            TagType::Dfn => "dfn",
            TagType::Em => "em",
            TagType::I => "i",
            TagType::Ins => "ins",
            TagType::Mark => "mark",
            TagType::Q => "q",
            TagType::Rp => "rp",
            TagType::Ruby => "ruby",
            TagType::Rt => "rt",
            TagType::S => "s",
            TagType::Small => "small",
            TagType::Strong => "strong",
            TagType::Sub => "sub",
            TagType::Sup => "sup",
            TagType::U => "u",
            TagType::A => "a",
            TagType::Link => "link",
            TagType::Menu => "menu",
            TagType::Ul => "ul",
            TagType::Ol => "ol",
            TagType::Li => "li",
            TagType::Dl => "dl",
            TagType::Dt => "dt",
            TagType::Dd => "dd",
            TagType::Table => "table",
            TagType::Caption => "caption",
            TagType::Th => "th",
            TagType::Tr => "tr",
            TagType::Td => "td",
            TagType::Thead => "thead",
            TagType::Tbody => "tbody",
            TagType::Tfoot => "tfoot",
            TagType::Col => "col",
            TagType::Colgroup => "colgroup",
            TagType::Template => "template",
            TagType::IFrame => "iframe",
            TagType::Nav => "nav",
            TagType::Div => "div",
            TagType::Span => "span",
            TagType::Header => "header",
            TagType::Hgroup => "hgroup",
            TagType::Footer => "footer",
            TagType::Main => "main",
            TagType::Section => "section",
            TagType::Search => "search",
            TagType::Article => "article",
            TagType::Aside => "aside",
            TagType::Details => "details",
            TagType::Dialog => "dialog",
            TagType::Summary => "summary",
            TagType::Data => "data",
            TagType::Code => "code",
            TagType::Pre => "pre",
            TagType::Samp => "samp",
            TagType::Form => "form",
            TagType::Input => "input",
            TagType::TextArea => "textarea",
            TagType::Button => "button",
            TagType::Select => "select",
            TagType::OptGroup => "optgroup",
            TagType::Option => "option",
            TagType::Label => "label",
            TagType::FieldSet => "fieldset",
            TagType::Legend => "legend",
            TagType::DataList => "datalist",
            TagType::Output => "output",
            TagType::Img => "img",
            TagType::Map => "map",
            TagType::Area => "area",
            TagType::Canvas => "canvas",
            TagType::FigCaption => "figcaption",
            TagType::Figure => "figure",
            TagType::Picture => "picture",
            TagType::Svg => "svg",
            TagType::Audio => "audio",
            TagType::Source => "source",
            TagType::Track => "track",
            TagType::Video => "video",
            TagType::Wbr => "wbr",
            TagType::Br => "br",
            TagType::Hr => "hr",
            TagType::Comment => "comment",
            TagType::Address => "address",
            TagType::Kdb => "kdb",
            TagType::Meter => "meter",
            TagType::Progress => "progress",
            TagType::Time => "time",
            TagType::Var => "var",
            TagType::Script => "script",
            TagType::NoScript => "noscript",
            TagType::Embed => "embed",
            TagType::Object => "object",
            TagType::Param => "param",
            TagType::Meta => "meta",
            TagType::Base => "base",
            TagType::Head => "head",
            TagType::Style => "style",
        }
        .to_string()
    }

    /// Indicates if the Html tag can wrap another element.
    /// Therfore, indicates if the Html tag is a no-child Html element.
    pub fn can_wrap(&self) -> bool {
        match self {
            Self::Area
            | Self::Base
            | Self::Br
            | Self::Col
            | Self::Embed
            | Self::Hr
            | Self::Img
            | Self::Input
            | Self::Link
            | Self::Meta
            | Self::Param
            | Self::Source
            | Self::Track
            | Self::Wbr => false,
            _ => true,
        }
    }

    /// Indicates if the tag is a single-child element.
    pub fn is_single_child(&self) -> bool {
        match self {
            Self::Title | Self::Legend | Self::Label | Self::Caption => true,
            _ => false,
        }
    }

    /// Indicates if the tag can be auto-closed.
    pub fn is_auto_closing(&self) -> bool {
        match self {
            Self::Map
            | Self::Base
            | Self::Br
            | Self::Col
            | Self::Embed
            | Self::Hr
            | Self::Img
            | Self::Input
            | Self::Link
            | Self::Meta
            | Self::Param
            | Self::Source
            | Self::Track
            | Self::Wbr => true,
            _ => false,
        }
    }

    /// Indicates if the tag is allowed in the html document's \<head>.
    pub fn is_allowed_is_head(&self) -> bool {
        match self {
            Self::Title
            | Self::Style
            | Self::Base
            | Self::Link
            | Self::Meta
            | Self::Script
            | Self::NoScript
            | Self::Comment => true,
            _ => false,
        }
    }
}

impl PartialEq for TagType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::P, Self::P)
            | (Self::H1, Self::H1)
            | (Self::H2, Self::H2)
            | (Self::H3, Self::H3)
            | (Self::H4, Self::H4)
            | (Self::H5, Self::H5)
            | (Self::H6, Self::H6)
            | (Self::Title, Self::Title)
            | (Self::Abbr, Self::Abbr)
            | (Self::B, Self::B)
            | (Self::Bdi, Self::Bdi)
            | (Self::Bdo, Self::Bdo)
            | (Self::Blockquote, Self::Blockquote)
            | (Self::Cite, Self::Cite)
            | (Self::Del, Self::Del)
            | (Self::Dfn, Self::Dfn)
            | (Self::Em, Self::Em)
            | (Self::I, Self::I)
            | (Self::Ins, Self::Ins)
            | (Self::Mark, Self::Mark)
            | (Self::Q, Self::Q)
            | (Self::Rp, Self::Rp)
            | (Self::Ruby, Self::Ruby)
            | (Self::Rt, Self::Rt)
            | (Self::S, Self::S)
            | (Self::Small, Self::Small)
            | (Self::Strong, Self::Strong)
            | (Self::Sub, Self::Sub)
            | (Self::Sup, Self::Sup)
            | (Self::U, Self::U)
            | (Self::A, Self::A)
            | (Self::Link, Self::Link)
            | (Self::Menu, Self::Menu)
            | (Self::Ul, Self::Ul)
            | (Self::Ol, Self::Ol)
            | (Self::Li, Self::Li)
            | (Self::Dl, Self::Dl)
            | (Self::Dt, Self::Dt)
            | (Self::Dd, Self::Dd)
            | (Self::Table, Self::Table)
            | (Self::Caption, Self::Caption)
            | (Self::Th, Self::Th)
            | (Self::Tr, Self::Tr)
            | (Self::Td, Self::Td)
            | (Self::Thead, Self::Thead)
            | (Self::Tbody, Self::Tbody)
            | (Self::Tfoot, Self::Tfoot)
            | (Self::Col, Self::Col)
            | (Self::Colgroup, Self::Colgroup)
            | (Self::Template, Self::Template)
            | (Self::IFrame, Self::IFrame)
            | (Self::Nav, Self::Nav)
            | (Self::Div, Self::Div)
            | (Self::Span, Self::Span)
            | (Self::Header, Self::Header)
            | (Self::Hgroup, Self::Hgroup)
            | (Self::Footer, Self::Footer)
            | (Self::Main, Self::Main)
            | (Self::Section, Self::Section)
            | (Self::Search, Self::Search)
            | (Self::Article, Self::Article)
            | (Self::Aside, Self::Aside)
            | (Self::Details, Self::Details)
            | (Self::Dialog, Self::Dialog)
            | (Self::Summary, Self::Summary)
            | (Self::Data, Self::Data)
            | (Self::Code, Self::Code)
            | (Self::Pre, Self::Pre)
            | (Self::Samp, Self::Samp)
            | (Self::Form, Self::Form)
            | (Self::Input, Self::Input)
            | (Self::TextArea, Self::TextArea)
            | (Self::Button, Self::Button)
            | (Self::Select, Self::Select)
            | (Self::OptGroup, Self::OptGroup)
            | (Self::Option, Self::Option)
            | (Self::Label, Self::Label)
            | (Self::FieldSet, Self::FieldSet)
            | (Self::Legend, Self::Legend)
            | (Self::DataList, Self::DataList)
            | (Self::Output, Self::Output)
            | (Self::Img, Self::Img)
            | (Self::Map, Self::Map)
            | (Self::Area, Self::Area)
            | (Self::Canvas, Self::Canvas)
            | (Self::FigCaption, Self::FigCaption)
            | (Self::Figure, Self::Figure)
            | (Self::Picture, Self::Picture)
            | (Self::Svg, Self::Svg)
            | (Self::Audio, Self::Audio)
            | (Self::Source, Self::Source)
            | (Self::Track, Self::Track)
            | (Self::Video, Self::Video)
            | (Self::Wbr, Self::Wbr)
            | (Self::Br, Self::Br)
            | (Self::Hr, Self::Hr)
            | (Self::Comment, Self::Comment)
            | (Self::Address, Self::Address)
            | (Self::Kdb, Self::Kdb)
            | (Self::Meter, Self::Meter)
            | (Self::Progress, Self::Progress)
            | (Self::Time, Self::Time)
            | (Self::Var, Self::Var)
            | (Self::Script, Self::Script)
            | (Self::NoScript, Self::NoScript)
            | (Self::Embed, Self::Embed)
            | (Self::Object, Self::Object)
            | (Self::Param, Self::Param)
            | (Self::Meta, Self::Meta)
            | (Self::Base, Self::Base)
            | (Self::Head, Self::Head)
            | (Self::Style, Self::Style) => true,
            _ => false,
        }
    }
}
