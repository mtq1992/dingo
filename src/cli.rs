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
    pub name: String,

    /// 解析记录类型
    #[clap(short, long, default_value = "A")]
    pub record_type: RecordType,
    
    /// 解析记录的服务器地址
    #[clap(short, long, default_value = "8.8.8.8:53")]
    pub domain_resolver: SocketAddr,
    
    /// 解析记录的协议
    #[clap(short, long, default_value = "udp")]
    pub protocol: String,

    /// doh 地址
    #[clap(short='h', long, default_value_if("protocol", Some("doh") , Some("https://dns.google.com/resolve") ))]
    pub doh_addr: Option<String>, 

    /// verbose 模式
    #[clap(short, long)]
    pub verbose: bool,
}



