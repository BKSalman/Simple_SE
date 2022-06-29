pub mod google;
pub mod twitter;
pub mod github;

pub fn get_command(query: &str) -> &str {
    if query.contains(' ') {
        let space_index = query.find(' ').unwrap_or(0);
        return &query[..space_index];
    }
    query
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_command() {
        let actual = get_command("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_get_command_with_whitespace() {
        let actual = get_command("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

}