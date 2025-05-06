use core::time;
use std::thread::sleep;

use rand;
use zmq;

const FAKE_TASKS: &[&str] = &[
    "process image_001.png",
    "resize image_002.png",
    "convert video_001.mp4",
    "extract text from document_1.pdf",
    "generate thumbnail for image_003.png",
    "compress archive backup_2023.zip",
    "apply watermark to image_004.jpg",
    "analyze sentiment in review_123.txt",
    "upload report to cloud",
    "clean temp folder",
    "index document_45.docx",
    "scan email logs for spam",
    "train model batch_03",
    "update configuration file",
    "rotate log files",
    "sync database replica",
    "fetch remote CSV",
    "generate PDF invoice_076",
    "merge user data from backups",
    "check system health metrics",
];

fn main() {
    let ctx = zmq::Context::new();
    let s = ctx.socket(zmq::SocketType::PUSH).unwrap();
    let addr = "tcp://127.0.0.1:4000";
    match s.bind(addr) {
        Ok(_) => println!("bound to {}", addr),
        Err(_) => panic!("could not bind socket"),
    };
    let mut tasks: Vec<(String, &str)> = FAKE_TASKS
        .iter()
        .enumerate()
        .map(|(i, x)| (format!("Task: {:03}", i), *x))
        .collect();

    while let Some((id, task)) = tasks.pop() {
        let message = format!("{}: {}", id, task);
        s.send(&message, 0).unwrap();
        println!("Sent: {}", message);
        sleep(time::Duration::from_secs(1));
    }
}
