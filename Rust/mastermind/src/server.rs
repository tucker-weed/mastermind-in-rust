use std::{
  io::{BufReader, BufRead, Write},
  net::{TcpListener, TcpStream},
  thread
};
use crate::player::MastermindPlayer;

pub struct MastermindServer {
  pub address: String,
  pub port: String
}

impl MastermindServer {
  pub fn listen(&self) {
    println!("Listening on {0} port {1}", self.address, self.port);
    let listener = TcpListener::bind(format!("{0}:{1}", self.address, self.port)).expect("Failed to bind TCP listener");
    for stream in listener.incoming() {
        let stream = stream.expect("Error processing TCP stream");
        self.accept_client(stream);
    }
  }

  fn accept_client(&self, mut stream: TcpStream) {
    let remote_addr = stream.peer_addr().expect("Error reading peer address for TCP stream");
    println!("Accepted connection from: ({0}, {1})", remote_addr.ip(), remote_addr.port());
    thread::spawn(move || {
      let mut player = MastermindPlayer{colors: vec!()};
      player.pick_colors();
      loop {
        let mut line = String::new();
        let mut buf_reader = BufReader::new(&mut stream);
        buf_reader.read_line(&mut line).expect("IO error encountered when reading TCP stream");
        let line = line.trim();
        if line.chars().count() == 0 {
          break;
        }
        println!("Received: {}", line);
        let (done, grade) = player.grade(&line);
        stream.write_all(grade.as_bytes()).expect("IO error encountered when writing TCP stream");
        if done {
          break;
        }
      }
      println!("Closed connection from: ({0}, {1})", remote_addr.ip(), remote_addr.port());
    });
  }
}
