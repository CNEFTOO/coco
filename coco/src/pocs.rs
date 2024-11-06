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
    headers: HashMap<String, String>,
    body: String,
    follow_redirects: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Detail {
    author: String,
    links: Vec<String>,
}

