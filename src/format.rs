use data::{Element};
use data::{
    List,
    Para,
    Plain,
    Text,
    BlockQuote,
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
};

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
            s.push_str((format!("<h{}>", elt.key.heading_level())).as_slice());
            push_formatted_elements(s, &elt.children);
            s.push_str((format!("</h{}>", elt.key.heading_level())).as_slice());
        }
        Plain => {
            push_formatted_elements(s, &elt.children);
        }
        Para => {
            s.push_str("<p>");
            push_formatted_elements(s, &elt.children);
            s.push_str("</p>");
        }
        BlockQuote => {
            s.push_str("<blockquote>\n");
            push_formatted_elements(s, &elt.children);
            s.push_str("</blockquote>\n");
        }
        Raw => {
            // XXX should have been processed before calling this method
            s.push_str(elt.text_as_slice());
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
