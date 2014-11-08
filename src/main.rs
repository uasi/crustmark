extern crate  markdown_parser;

#[cfg(not(test))]
fn main() {
    println!("Run `cargo test`");
}

#[cfg(test)]
mod tests {
    extern crate markdown_parser;

    #[test]
    fn atx_header_level3() {
        let doc = markdown_parser::doc("### Hello ###\n");
        assert!(doc.unwrap().key == markdown_parser::H3);
    }
}
