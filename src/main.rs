extern crate  markdown_parser;

#[cfg(not(test))]
fn main() {
    println!("Run `cargo test`");
}

#[cfg(test)]
mod tests {
    extern crate markdown_parser;

    #[test]
    fn atx_head() {
        let doc = markdown_parser::doc("# Hello #\n");
        assert!(doc.is_ok());
    }
}
