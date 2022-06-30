use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ')
.add(b'"')
.add(b'<')
.add(b'>')
.add(b'`');

pub fn twitch_url(query: &str) -> String {
    if query == "ttv" {
        String::from("https://twitch.tv/")
    } else if &query[..5] == "ttv @" {
        twitch_page(&query[5..])
    } else if query == "ttv ppt" {
        twitch_popout()
    } else {
        twitch_search(&query[4..])
    }
}

pub fn twitch_page(page:&str) -> String {
    format!("https://twitch.tv/{}", page)
}

pub fn twitch_search(search:&str) -> String {
    let encoded_search = utf8_percent_encode(search, FRAGMENT);
    format!("https://twitch.tv/search?term={}", encoded_search)
}

pub fn twitch_popout() -> String {
    String::from("https://www.twitch.tv/popout/bksalman/chat?popout=")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_twitch_page() {
        let actual = twitch_url("ttv @bksalman");
        let expected = "https://twitch.tv/bksalman";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_twitch_search() {
        let actual = twitch_url("ttv something");
        let expected = "https://twitch.tv/search?term=something";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_twitch_search_with_encoding() {
        let actual = twitch_url("ttv lmao something");
        let expected = "https://twitch.tv/search?term=lmao%20something";
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_twitch_popout() {
        let actual = twitch_url("ttv ppt");
        let expected = "https://www.twitch.tv/popout/bksalman/chat?popout=";
        assert_eq!(actual, expected);
    }
}