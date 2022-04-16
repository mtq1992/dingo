use clap::StructOpt;
use rand::Rng;

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

    if verbose {
        println!("done");
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
    panic!("not implemented");
}