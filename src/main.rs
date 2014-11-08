extern crate  markdown_parser;

#[cfg(not(test))]
fn main() {
    println!("Run `cargo test`");
}

#[cfg(test)]
mod tests {
    extern crate markdown_parser;
    use markdown_parser::html_string_from_element;

    #[test]
    fn para() {
        let doc = markdown_parser::doc("Hello\nWorld\n\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, markdown_parser::Para);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<p>Hello\nWorld</p>");
    }

    #[test]
    fn atx_header_level3() {
        let doc = markdown_parser::doc("### Hello ###\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, markdown_parser::H3);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<h3>Hello</h3>");
    }
}
