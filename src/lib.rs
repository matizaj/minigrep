pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result= Vec::new();
    for line in content.lines() {
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
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