mod data;
mod format;
mod parse;

#[cfg(not(test))]
fn main() {
    println!("Run `cargo test`");
}

#[cfg(test)]
mod tests {
    use data;
    use parse;
    use format::html_string_from_element;

    #[test]
    fn para() {
        let doc = parse::doc("Hello\nWorld\n\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, data::Para);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<p>Hello\nWorld</p>");
    }

    #[test]
    fn atx_heading3() {
        let doc = parse::doc("### Hello ###\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, data::H3);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<h3>Hello</h3>");
    }

    #[test]
    fn setext_heading1() {
        let doc = parse::doc("Hello\n===\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, data::H1);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<h1>Hello</h1>");
    }

    #[test]
    fn verbatim() {
        let doc = parse::doc("    Hello\n    World\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, data::Verbatim);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<pre><code>Hello\nWorld\n</code></pre>");
    }

    #[test]
    fn horizontal_rule() {
        let doc = parse::doc("---\n\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, data::HorizontalRule);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<hr />");
    }
}
