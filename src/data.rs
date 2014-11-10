#[deriving(PartialEq, Show)]
pub enum Key {
    List,
    Para,
    Plain,
    Text,
    BlockQuote,
    Verbatim,
    HorizontalRule,
    Raw,
    LineBreak,
    Space,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Terminal
}

impl Key {
    pub fn from_heading_level(u: uint) -> Key {
        match u {
            1 => H1,
            2 => H2,
            3 => H3,
            4 => H4,
            5 => H5,
            6 => H6,
            _ => unreachable!()
        }
    }

    pub fn heading_level(&self) -> uint {
        match self {
            &H1 => 1,
            &H2 => 2,
            &H3 => 3,
            &H4 => 4,
            &H5 => 5,
            &H6 => 6,
            _   => unreachable!()
        }
    }
}

pub struct Element {
    pub key: Key,
    pub children: Vec<Element>,
    pub text: Option<String>
}

impl Element {
    pub fn new(key: Key) -> Element {
        Element {
            key: key,
            children: vec![],
            text: None
        }
    }

    pub fn with_children(key: Key, children: Vec<Element>) -> Element {
        Element {
            key: key,
            children: children,
            text: None
        }
    }

    pub fn new_list(children: Vec<Element>) -> Element {
        Element::with_children(List, children)
    }

    pub fn new_text(s: &str) -> Element {
        Element {
            key: Text,
            children: vec![],
            text: Some(s.to_string())
        }
    }

    pub fn put_key(self, key: Key) -> Element {
        Element {
            key: key,
            children: self.children,
            text: self.text
        }
    }

    pub fn text_as_slice<'a>(&'a self) -> &'a str {
        match self.text {
            Some(ref t) => t.as_slice(),
            None => ""
        }
    }
}
