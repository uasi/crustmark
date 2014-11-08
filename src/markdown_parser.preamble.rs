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

    fn text_as_slice<'a>(&'a self) -> &'a str {
        match self.text {
            Some(ref t) => t.as_slice(),
            None    => ""
        }
    }
}

pub fn html_string_from_element(elt: &Element) -> String {
    let mut s = String::new();
    push_formatted_element(&mut s, elt);
    s
}

fn push_formatted_element(s: &mut String, elt: &Element) {
    match elt.key {
        Space => {
            s.push_str(elt.text_as_slice())
        }
        LineBreak => {
            s.push_str("<br />\n");
        }
        Text => {
            s.push_str(elt.text_as_slice()); // XXX needs escaping
        }
        List => {
            push_formatted_elements(s, &elt.children);
        }
        H1 | H2 | H3 | H4 | H5 | H6 => {
            s.push_str("<hX>"); // XXX needs level
            push_formatted_elements(s, &elt.children);
            s.push_str("</hX>");
        }
        Plain => {
            push_formatted_elements(s, &elt.children);
        }
        Para => {
            s.push_str("<p>");
            push_formatted_elements(s, &elt.children);
            s.push_str("</p>");
        }
        Terminal => {
        }
    }
}

fn push_formatted_elements(s: &mut String, elts: &Vec<Element>) {
    for elt in elts.iter() {
        push_formatted_element(s, elt);
    }
}
