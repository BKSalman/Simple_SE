use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn google_search(search: &str) -> String {
    let encoded_search = utf8_percent_encode(search, FRAGMENT);
    format!("https://google.com/search?q={encoded_search}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_search() {
        let actual = google_search("test");
        let expected = "https://google.com/search?q=test";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_google_search_with_encoding() {
        let actual = google_search("test lmao");
        let expected = "https://google.com/search?q=test%20lmao";
        assert_eq!(actual, expected);
    }
}
