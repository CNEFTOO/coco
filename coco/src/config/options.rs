use clap::{ Command, Arg, ArgAction };

#[derive(Debug)]
pub struct Options {
    pub version: bool,
    pub file: String,
    pub url: String,
    pub threads: u32,
    pub timeout: u32,
    pub debug: bool
}

pub fn parse_args() -> Options {
    let matches = Command::new("coco")
        .version("0.1.0")
        .author("seaung")
        .about("一个基于CEL的漏洞检测工具")
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .action(ArgAction::SetTrue)
            .help("显示版本信息"))
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("指定POC文件路径")
            .required(false))
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .help("指定目标URL")
            .required(false))
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .help("并发线程数")
            .default_value("10"))
        .arg(Arg::new("timeout")
            .long("timeout")
            .help("请求超时时间(秒)")
            .default_value("30"))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .action(ArgAction::SetTrue)
            .help("启用调试模式"))
        .get_matches();

    Options {
        version: matches.get_flag("version"),
        file: matches.get_one::<String>("file").unwrap_or(&String::from("")).to_string(),
        url: matches.get_one::<String>("url").unwrap_or(&String::from("")).to_string(),
        threads: matches.get_one::<String>("threads").unwrap().parse().unwrap_or(10),
        timeout: matches.get_one::<String>("timeout").unwrap().parse().unwrap_or(30),
        debug: matches.get_flag("debug")
    }
}