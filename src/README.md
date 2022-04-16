# rust 实现的 dig client

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
    -h, --doh-addr <DOH_ADDR>                  doh 地址
        --help                                 Print help information
    -p, --protocol <PROTOCOL>                  解析记录的协议 [default: udp]
    -r, --record-type <RECORD_TYPE>            解析记录类型 [default: A]
    -v, --verbose                              verbose 模式
    -V, --version                              Print version information

```

## 参考文档
[rust clap](https://docs.rs/clap/latest/clap/index.html)
[making-a-dns-client](https://blog.adamchalmers.com/making-a-dns-client/)
