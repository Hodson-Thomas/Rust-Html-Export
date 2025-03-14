/// Defines the wrap mode behavior.
#[derive(Clone, Debug)]
pub enum WrapMode {
    /// Places the wrapped element at the end of the wrapper's sub-element list.
    End,
    /// Places the wrapped element at the start of the wrapper's sub-element list.
    Start,
    /// Places the wrapped element at a specific index of the wrapper's sub-element list.
    At(usize),
}

/// Specifies the default target for all hyperlinks and forms in the page.
#[derive(Clone, Debug)]
pub enum BaseUrlTarget {
    _Blank,
    _Parent,
    _Self,
    _Top,
}

/// Specifies the loading mode of the `<script>` tag.
pub enum ScriptLoadMode {
    Defer,
    Async,
}

impl ToString for BaseUrlTarget {
    fn to_string(&self) -> String {
        match self {
            BaseUrlTarget::_Blank => "_blank",
            BaseUrlTarget::_Parent => "_parent",
            BaseUrlTarget::_Self => "_self",
            BaseUrlTarget::_Top => "_top",
        }
        .to_string()
    }
}

impl ToString for ScriptLoadMode {
    fn to_string(&self) -> String {
        match self {
            ScriptLoadMode::Defer => "defer",
            ScriptLoadMode::Async => "async",
        }
        .to_string()
    }
}
