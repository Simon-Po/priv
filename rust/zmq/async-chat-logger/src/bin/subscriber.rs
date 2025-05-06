use zmq;
fn main() {
    let ctx = zmq::Context::new();
    let sub_soc = ctx.socket(zmq::SocketType::SUB).unwrap();
    sub_soc.connect("tcp://127.0.0.1:3000").unwrap();
    sub_soc.set_subscribe(b"").unwrap();

    loop {
        let msg = sub_soc.recv_string(0).unwrap().unwrap();
        println!("{}", msg);
    }
}
