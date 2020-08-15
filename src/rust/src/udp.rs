use json_flex;
use json_flex::{JFObject, Unwrap};

use std::io;
use std::net::UdpSocket;

pub fn start(socket: &UdpSocket) -> Option<std::boxed::Box<JFObject>> {
   //let socket = UdpSocket::bind("0.0.0.0:5000").expect("Could not bind socket");
   //socket.set_nonblocking(true).unwrap();
   let mut buf = [0; 1024];

   match socket.recv_from(&mut buf) {
      Ok(n) => {
         let converted: String = String::from_utf8(buf.to_vec()).unwrap();
         let array = json_flex::decode(converted.to_owned());
         return Some(array);
      }
      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
         return None;
      }
      Err(e) => panic!("encountered IO error: {}", e),
   }
   return None;
}
