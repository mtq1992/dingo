use std::net::SocketAddr;

use clap::Parser;

use crate::dns_types::RecordType;


#[derive(Parser, Debug)]
#[clap(
    name = "dns-client",
    about = "A simple DNS client",
    version = "0.1.0",
    author = "FranksMa mtqmx3@gmail.com")]
pub struct Cli {
    /// 需要解析的域名
    #[clap(parse(try_from_str = parse_domain_name))]
    pub name: String,

    /// 解析记录类型
    #[clap(short, long, default_value = "A")]
    pub record_type: RecordType,
    
    /// 解析记录的服务器地址
    #[clap(short, long, parse(try_from_str = parse_socket_addr), default_value = "8.8.8.8:53")]
    pub domain_resolver: SocketAddr,
    
    /// 解析记录的协议
    #[clap(short, long, default_value = "udp", parse(try_from_str = parse_protocol))]
    pub protocol: String,

    /// doh 地址
    #[clap(short='s', long, default_value_if("protocol", Some("doh") , Some("https://cloudflare-dns.com/dns-query") ))]
    pub doh_addr: Option<String>, 

    /// verbose 模式
    #[clap(short, long)]
    pub verbose: bool,
}


fn parse_socket_addr(s: &str) -> Result<SocketAddr, std::net::AddrParseError> {
    s.parse()
}

fn parse_domain_name(s: &str) -> Result<String, String> {
    let mut s = s.to_string();
    if !s.is_ascii() {
        return Err(format!("{} is not ascii", s));
    }
    // dns协议中域名均以 . 结尾, 如果不以 . 结尾, 则补充 .
    if !s.ends_with('.') {
        s.push_str(".");
    }
    Ok(s.to_string())
}

fn parse_protocol(s: &str) -> Result<String, String> {
    let protocol = match s.to_lowercase().as_str() {
        "udp" => "udp",
        "tcp" => "tcp",
        "doh" => "doh",
        _ => return Err(format!("{} is not a valid protocol", s)),
    };  
    Ok(protocol.to_string())
}
