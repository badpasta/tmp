/* 
  This is a rust program...

  Author: badpasta <beforget@hotmail.com> 

  Environment:
  None.

  Create at: 2017-11-07 22:16:36
*/

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use std::net::SocketAddr;
use std::thread;
//use futures::sync::mpsc::{self, Sender, Receiver};
use futures::sync::mpsc;
use futures::{Sink, Future, Stream};
//use futures::prelude::*;

use tokio_core::reactor::Core;
use tokio_io::io as tokio;
use tokio_core::net::TcpStream;

struct Point {
    name: String,
    value: f64,
    timestamp: i64,
}

fn main() {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let (mut tx, rx) = mpsc::channel(0);
    thread::spawn(|| {
        let point: Point = Point {name: "test".to_owned(), value: 1.0, timestamp: 123456};
        tx = match tx.send(point).wait() {
            Ok(tx) => tx,
            Err(e) => panic!(e),
        };
    });

    let rx = rx.map_err(|_| panic!());

    let socket = TcpStream::connect(&addr, &core.handle());

    let request = socket.and_then(|socket| {
        let result = rx.collect();
        let r = result.wait();
        let mut s = String::new();
        for re in &r.unwrap() {
            let print = format!("{} {:?} {:?}\n", re.name, re.value, re.timestamp);
            println!("{}", &print);
            s += &print;
        }
        tokio::write_all(socket, s.as_bytes().clone())
    });
    let response = request.and_then(|(socket, addr)| {
        //println!("send to {:?}", String::from_utf8_lossy(&addr));
        tokio::read_to_end(socket, Vec::new())
    });

    let (_, data) = core.run(response).unwrap();
    println!("response: {}", String::from_utf8_lossy(&data));
}
