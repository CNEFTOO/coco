use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Pocs {
    name: String,
    manual: bool,
    transport: String,
    set: Set,
    rules: HashMap<String, Rule>,
    expression: String,
    detail: Detail,
}

#[derive(Debug, Serialize, Deserialize)]
struct Set {
    rand: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    request: Request,
    expression: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    cache: bool,
    method: String,
    path: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
    follow_redirects: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Detail {
    author: String,
    links: Vec<String>,
}

fn load_poc_from_file(file_path: &str) -> Result<Pocs, String> {
    let content = std::fs::read_to_string(file_path).map_err(|err| err.to_string())?;
    let poc: Pocs = serde_yaml::from_str(&content).map_err(|err| err.to_string())?;
    Ok(poc)
}

// pub fn load_single_poc(path: &str) -> Pocs {
//     load_poc_from_file(path).unwrap()
// }
//
// pub fn load_all(path: &str) -> Vec<Pocs> {
//     vec![load_single_poc(path)]
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_single_poc() {
        let result = load_poc_from_file("./pocs/demo.yml");
        println!("{:#?}", result);
    }
}