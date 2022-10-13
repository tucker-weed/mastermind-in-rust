mod server;
mod player;

use std::{
  process,
  env
};
use crate::server::MastermindServer;


fn main() {
  let mut args: Vec<_> = env::args().collect();
  if args.len() < 3 {
    eprintln!("Please enter [listen address] and [port] as arguments");
    process::exit(1);
  }
  let tcp_server = MastermindServer{address: args.remove(1), port: args.remove(1)};
  tcp_server.listen();
}
