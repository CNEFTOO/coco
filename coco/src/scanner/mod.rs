use crate::http::HttpClient;
use crate::rule::Poc;
use crate::utils::{self, debug, error};
use cel_interpreter::{Context, Value};
use cel_interpreter::objects::Map;

mod coco;
pub fn start() {
    use crate::config::parse_args;
    
    utils::show_banner();
    
    let options = parse_args();
    
    if options.version {
        println!("coco 0.1.0");
        return;
    }
    
    if options.file.is_empty() && options.url.is_empty() {
        error("请指定目标URL或URL列表文件");
        return;
    }
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let scanner = Scanner::new(options.threads as usize, options.timeout);
        
        let pocs = if !options.file.is_empty() {
            crate::rule::pocs::load_pocs(&options.file)
        } else {
            error("请指定POC文件或目录");
            return;
        };

        if pocs.is_empty() {
            error("未找到有效的POC");
            return;
        }

        let pocs: Vec<Arc<Poc>> = pocs.into_iter().map(|p| Arc::new(p)).collect();
        
        let mut urls = Vec::new();
        if !options.url.is_empty() {
            urls.push(options.url);
        }
        if !options.file.is_empty() {
            if let Ok(content) = std::fs::read_to_string(&options.file) {
                urls.extend(content.lines().map(String::from));
            } else {
                error("无法读取URL列表文件");
                return;
            }
        }
        
        if options.debug {
            debug("调试模式已启用");
        }
        
        scanner.scan(pocs, urls).await;
    });
}
use std::sync::Arc;
use tokio::sync::Semaphore;
use reqwest::Client;
use std::time::Duration;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{Table, row};

pub struct Scanner {
    client: Arc<Client>,
    threads: usize,
    timeout: u32,
}

impl Scanner {
    pub fn new(threads: usize, timeout: u32) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout as u64))
            .build()
            .unwrap();
        
        Scanner {
            client: Arc::new(client),
            threads,
            timeout,
        }
    }

    pub async fn scan(&self, pocs: Vec<Arc<Poc>>, urls: Vec<String>) {
        let semaphore = Arc::new(Semaphore::new(self.threads));
        
        let total_tasks = urls.len() * pocs.len();
        let pb = ProgressBar::new(total_tasks as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap());

        let mut results = Table::new();
        results.add_row(row!["URL", "漏洞名称", "状态"]);

        let mut handles = vec![];
        for url in urls {
            for poc in pocs.iter() {
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                let client = self.client.clone();
                let poc = poc.clone();
                let url = url.clone();
                let pb = pb.clone();
                
                let handle = tokio::spawn(async move {
                    let _permit = permit;
                    let result = Self::check_vulnerability(&client, &url, &poc).await;
                    pb.inc(1);
                    (url, poc.name.clone(), result)
                });
                handles.push(handle);
            }
        }

        for handle in handles {
            let (url, name, is_vulnerable) = handle.await.unwrap();
            let status = if is_vulnerable {
                "存在漏洞".red().to_string()
            } else {
                "未发现漏洞".green().to_string()
            };
            results.add_row(row![&url, &name, &status]);
        }

        pb.finish_with_message("检测完成");
        println!();
        println!("{}", results);
    }

    async fn check_vulnerability(client: &Client, url: &str, poc: &Poc) -> bool {
        let http_client = HttpClient::new(client);
        match http_client.send_request(url, poc).await {
            Ok((status, headers, body)) => {
                let mut values = std::collections::HashMap::new();
                values.insert("status".to_string(), Value::Int(status as i64));
                values.insert("body".to_string(), Value::String(body.into()));
                
                let mut header_map = std::collections::HashMap::new();
                for (name, value) in headers.iter() {
                    if let Ok(v) = value.to_str() {
                        header_map.insert(name.as_str().to_string(), Value::String(v.to_string().into()));
                    }
                }
                values.insert("headers".to_string(), Value::Map(Map::from(header_map)));
                let mut context = Context::default();
                for (k, v) in values {
                    context.add_variable(&k, v);
                }
                
                debug("正在编译CEL表达式...");
                match poc.compile_and_execute(&mut context) {
                    Ok(is_vulnerable) => is_vulnerable,
                    Err(e) => {
                        error(&format!("执行POC失败: {}", e));
                        false
                    }
                }
            }
            Err(e) => {
                error(&format!("请求失败: {}", e));
                false
            }
        }
    }
}