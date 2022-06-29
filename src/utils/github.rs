use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ')
.add(b'"')
.add(b'<')
.add(b'>')
.add(b'`');

pub fn github_url(query: &str) -> String {
    if query == "gh" {
        String::from("https://github.com/")
    } else if &query[..4] == "gh @" {
        github_page(&query[4..])
    } else {
        github_search(&query[3..])
    }
}

pub fn github_page(page:&str) -> String {
    format!("https://github.com/{}", page)
}

pub fn github_search(search:&str) -> String {
    let encoded_search = utf8_percent_encode(search, FRAGMENT);
    format!("https://github.com/search?q={}", encoded_search)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_github_page() {
        let actual = github_url("gh @bksalman");
        let expected = "https://github.com/bksalman";
    assert_eq!(actual, expected);
    }

    #[test]
    fn test_github_repo() {
        let actual = github_url("gh @bksalman/letters_game");
        let expected = "https://github.com/bksalman/letters_game";
    assert_eq!(actual, expected);
    }

    #[test]
    fn test_github_search() {
        let actual = github_url("gh something");
        let expected = "https://github.com/search?q=something";
    assert_eq!(actual, expected);
    }

    #[test]
    fn test_github_search_with_encoding() {
        let actual = github_url("gh lmao something");
        let expected = "https://github.com/search?q=lmao%20something";
    assert_eq!(actual, expected);
    }
}