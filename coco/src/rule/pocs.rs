use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::Poc;

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

pub fn load_pocs(path: &str) -> Vec<super::Poc> {
    let mut pocs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "yml" || ext == "yaml" {
                        match load_poc_from_file(entry.path().to_str().unwrap()) {
                            Ok(poc) => pocs.push(super::Poc {
                                name: poc.name,
                                description: poc.detail.author,
                                expression: poc.expression,
                                method: poc.rules.values().next().map(|r| r.request.method.clone()).unwrap_or_default(),
                                headers: poc.rules.values().next().and_then(|r| r.request.headers.clone()).unwrap_or_default(),
                                body: poc.rules.values().next().and_then(|r| r.request.body.clone()),
                            }),
                            Err(_) => ()
                        }
                    }
                }
            }
        }
    }
    pocs
}

pub fn load_single_poc(path: &str) -> Result<Pocs, String> {
    load_poc_from_file(path)
}

pub fn load_all(dir_path: &str) -> Result<Vec<Pocs>, String> {
    let mut pocs = Vec::new();
    let paths = std::fs::read_dir(dir_path).map_err(|e| e.to_string())?;

    for path in paths {
        let path = path.map_err(|e| e.to_string())?;
        let file_path = path.path();
        
        if let Some(extension) = file_path.extension() {
            if extension == "yml" || extension == "yaml" {
                match load_single_poc(file_path.to_str().unwrap()) {
                    Ok(poc) => pocs.push(poc),
                    Err(e) => eprintln!("Error loading POC file {}: {}", file_path.display(), e)
                }
            }
        }
    }

    Ok(pocs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_single_poc() {
        let result = load_poc_from_file("./pocs/demo.yml");
        println!("{:#?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_single_poc() {
        let result = load_poc_from_file("./pocs/demo.yml");
        println!("{:#?}", result);
    }
}