mod crustmark;

#[cfg(not(test))]
fn main() {
    println!("Run `cargo test`");
}

#[cfg(test)]
mod tests {
    use crustmark;
    use crustmark::parse::html_string_from_element;

    #[test]
    fn para() {
        let doc = crustmark::parse::doc("Hello\nWorld\n\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, crustmark::parse::Para);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<p>Hello\nWorld</p>");
    }

    #[test]
    fn atx_header_level3() {
        let doc = crustmark::parse::doc("### Hello ###\n");
        let elt = doc.unwrap();
        assert_eq!(elt.key, crustmark::parse::H3);
        assert_eq!(html_string_from_element(&elt).as_slice(),
                   "<h3>Hello</h3>");
    }
}
