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

    let query_id = rand::thread_rng().gen::<u16>();
    
    let msg = Message::new_query(query_id, args.name , args.record_type).unwrap();
    
    let (resp, len) = io::send_req(msg, args.domain_resolver, args.verbose).unwrap();
    
    if let Err(e) = io::print_resp(resp, len, query_id, args.verbose){
        print!("Error sending query: {:?}", e);
    }
}
