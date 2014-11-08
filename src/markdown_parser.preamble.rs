#![allow(non_snake_case, unused)]

#[deriving(PartialEq, Show)]
pub enum Key {
    List,
    Para,
    Plain,
    Text,
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
    fn from_header_level(u: uint) -> Key {
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
}

pub struct Element {
    pub key: Key,
    pub children: Vec<Element>,
    pub text: Option<String>
}

impl Element {
    fn new(key: Key) -> Element {
        Element {
            key: key,
            children: vec![],
            text: None
        }
    }

    fn new_list(children: Vec<Element>) -> Element {
        Element::new_with_children(List, children)
    }

    fn new_with_children(key: Key, children: Vec<Element>) -> Element {
        Element {
            key: key,
            children: children,
            text: None
        }
    }

    fn new_text(s: &str) -> Element {
        Element {
            key: Text,
            children: vec![],
            text: Some(s.to_string())
        }
    }

    fn put_key(self, key: Key) -> Element {
        Element {
            key: key,
            children: self.children,
            text: self.text
        }
    }
}
