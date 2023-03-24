use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn nix_url(query: &str) -> String {
    if query == "n" {
        String::from("https://nixos.wiki/")
    } else if &query[..=2] == "n p" {
        unstable_package_search(&query[3..].trim())
    } else if &query[..=2] == "n o" {
        unstable_option_search(&query[3..].trim())
    } else {
        query.to_string()
    }
}

pub fn unstable_package_search(search: &str) -> String {
    let encoded_search = utf8_percent_encode(search, FRAGMENT);
    format!(
        "https://search.nixos.org/packages?channel=unstable&query={}",
        encoded_search
    )
}

pub fn unstable_option_search(search: &str) -> String {
    let encoded_search = utf8_percent_encode(search, FRAGMENT);
    format!(
        "https://search.nixos.org/options?channel=unstable&query={}",
        encoded_search
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_url() {
        let query = "n";

        assert_eq!(nix_url(query), String::from("https://nixos.wiki/"))
    }

    #[test]
    fn test_unstable_package_search() {
        let query = "n p freecad";

        let result = nix_url(query);

        assert_eq!(
            result,
            String::from("https://search.nixos.org/packages?channel=unstable&query=freecad")
        )
    }

    #[test]
    fn test_unstable_option_search() {
        let query = "n o virtualisation";

        assert_eq!(
            nix_url(query),
            String::from("https://search.nixos.org/options?channel=unstable&query=virtualisation")
        )
    }
}
