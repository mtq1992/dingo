# rust 实现的 dns client  

## 使用帮助  
```
dns-client 0.1.0
FranksMa mtqmx3@gmail.com
A simple DNS client

USAGE:
    dingo [OPTIONS] <NAME>

ARGS:
    <NAME>    需要解析的域名

OPTIONS:
    -d, --domain-resolver <DOMAIN_RESOLVER>    解析记录的服务器地址 [default: 8.8.8.8:53]
    -h, --help                                 Print help information
    -p, --protocol <PROTOCOL>                  解析记录的协议 [default: udp]
    -r, --record-type <RECORD_TYPE>            解析记录类型 [default: A]
    -s, --doh-addr <DOH_ADDR>                  doh 地址
    -v, --verbose                              verbose 模式
    -V, --version                              Print version information
```

## TODO 
- doh查询

## 参考文档
[rust clap](https://docs.rs/clap/latest/clap/index.html)  
[making-a-dns-client](https://blog.adamchalmers.com/making-a-dns-client/)  
[cloudflare doh use json](https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-json/)