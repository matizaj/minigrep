/// Search function for a given query and path to file
/// 
/// # Example
/// 
///  ```
///  query = rust contenct= 'Trust me rust is interesting'
/// assert_eq!(['Trust me rust is interesting'], search(query,content))
/// ```

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}


/// Search function for a given query and path file, set IGNORE_CASE to true in env var to use this option
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_one() {
        let query = "duct";
        let content = "\
Rust:
productive, fast, safe, not easy.
Pick three. ";
        let result = search(query, content);
        assert_eq!(result, vec!["productive, fast, safe, not easy."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
productive, fast, safe, not easy.
Trust me. ";

    assert_eq!(vec!["Rust:", "Trust me. "], search_case_insensitive(query, content));
    }
}