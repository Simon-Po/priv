use core::time;
use std::thread::sleep;

use zmq;

fn main() {
    let ctx = zmq::Context::new();
    let s = ctx.socket(zmq::SocketType::PULL).unwrap();
    let mut args = std::env::args();
    while let Some(arg) = args.next() {
        if arg == "id" || arg == "-i" || arg == "--id" {
            break;
        }
    }
    let name = format!("Worker {}", args.next().unwrap());
    s.connect("tcp://127.0.0.1:4000").unwrap();
    loop {
        if let Some(msg) = match s.recv_string(0) {
            Ok(msg) => Some(msg.unwrap()),
            Err(_) => None,
        } {

         println!("{}: {}", name, msg);
        } else {
            println!("{}: No work found yet",name);
        }


        sleep(time::Duration::from_secs(rand::random_range(1..5)));
    }
}
