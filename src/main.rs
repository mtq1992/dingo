use clap::StructOpt;
use rand::Rng;
use reqwest::header;
use serde_json::Value;

use crate::{cli::Cli, message::Message, dns_types::{Class, RecordType}};

mod cli;
mod dns_types;
mod message;
mod parse;
mod io;

fn main() {
    let args = Cli::parse();
    let verbose = args.verbose;
    if verbose {
        println!("args: {:?}", args);
    }

    if args.protocol == "udp" {
        udp_client(args);
    } else if args.protocol == "doh" {
        doh_client(args);
    } else {
        panic!("unsupported protocol: {}", args.protocol);
    }
}


fn udp_client(args: Cli) {
    let query_id = rand::thread_rng().gen::<u16>();
    
    let msg = Message::new_query(query_id, args.name , args.record_type).unwrap();
    
    let (resp, len) = io::send_req(msg, args.domain_resolver, args.verbose).unwrap();
    
    if let Err(e) = io::print_resp(resp, len, query_id, args.verbose){
        print!("Error sending query: {:?}", e);
    }
}

fn doh_client(args: Cli) {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/dns-json"));

    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    let url = format!("{}?name={}&type={}", args.doh_addr.unwrap().as_str(), args.name, args.record_type.to_string());

    println!("url: {}", url);
    
    let resp = client.get(&url).send().unwrap();

    let resp_body = resp.text().unwrap();

    let test = serde_json::from_str::<Value>(&resp_body).unwrap();

    println!("{}",serde_json::to_string_pretty(&test).unwrap());
}

