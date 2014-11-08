extern crate  markdown_parser;

#[cfg(not(test))]
fn main() {
    println!("Run `cargo test`");
}

#[cfg(test)]
mod tests {
    extern crate markdown_parser;

    #[test]
    fn para() {
        let doc = markdown_parser::doc("Hello\nWorld\n\n");
        assert_eq!(doc.unwrap().key, markdown_parser::Para);
    }

    #[test]
    fn atx_header_level3() {
        let doc = markdown_parser::doc("### Hello ###\n");
        assert_eq!(doc.unwrap().key, markdown_parser::H3);
    }
}
