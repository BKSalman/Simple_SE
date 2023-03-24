use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn twitter_url(query: &str) -> String {
    if query == "tw" {
        String::from("https://twitter.com/")
    } else if &query[..4] == "tw @" {
        twitter_profile(&query[4..])
    } else {
        twitter_search(&query[3..])
    }
}

pub fn twitter_profile(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

pub fn twitter_search(search: &str) -> String {
    let encoded_search = utf8_percent_encode(search, FRAGMENT);
    format!("https://twitter.com/search?q={}", encoded_search)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_profile() {
        let actual = twitter_url("tw @bksalman1");
        let expected = "https://twitter.com/bksalman1";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_twitter_search() {
        let actual = twitter_url("tw something");
        let expected = "https://twitter.com/search?q=something";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_twitter_search_with_encoding() {
        let actual = twitter_url("tw lmao something");
        let expected = "https://twitter.com/search?q=lmao%20something";
        assert_eq!(actual, expected);
    }
}
