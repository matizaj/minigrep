pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result= Vec::new();
    for line in content.lines() {
        if line.contains(query){
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
}