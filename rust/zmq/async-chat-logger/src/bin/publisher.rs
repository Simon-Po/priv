use core::time;
use std::thread::sleep;

use rand;
use zmq::{self};

const FAKE_USERS: &[&str] = &[
    "alice", "bob", "charlie", "dave", "eve", "frank", "grace", "hank", "irene", "jack", "kathy",
    "luke", "mike", "nina", "olivia", "paul", "quinn", "rachel", "sam", "tina",
];
const FAKE_MESSAGES: &[&str] = &[
    "Hey, how's it going?",
    "Did you catch the game last night?",
    "I'm running late, sorry!",
    "Let's grab lunch tomorrow.",
    "Can you send me the files?",
    "That was hilarious!",
    "I'll call you back in a bit.",
    "Don't forget the meeting at 3 PM.",
    "What time are we meeting?",
    "Thanks for your help earlier!",
    "I can't believe that happened!",
    "Are you free this weekend?",
    "Let me know when you're ready.",
    "Just got home, finally!",
    "Can you believe this weather?",
    "I'll be there in 10 minutes.",
    "Do you need anything from the store?",
    "That sounds like a great idea!",
    "I'm so tired today.",
    "Let's catch up soon!",
    "Did you hear the news?",
    "I'll send you the details later.",
    "Thanks, I owe you one!",
    "What do you think about this?",
    "I'm not sure about that.",
    "Let's figure it out together.",
    "I'll check and get back to you.",
    "That was really impressive!",
    "I appreciate your support.",
    "Can we reschedule our meeting?",
    "Looking forward to it!",
    "Let me double-check and confirm.",
    "I hope you're doing well.",
    "That place was amazing!",
    "I'll let you know by tomorrow.",
    "Do you have a minute to talk?",
    "I'm so excited about this!",
    "Let's make it happen.",
    "Thanks for letting me know.",
    "I'll see you at the usual spot.",
    "Can you believe it's already Friday?",
    "That was a close call!",
    "I can't wait to tell you about it.",
    "Let me know if you need anything.",
    "I'm really happy for you!",
    "That makes so much sense now.",
    "I'll handle it, don't worry.",
    "You're not going to believe this!",
    "Let's plan something fun soon.",
    "Take care and stay safe!",
];

fn main() {
    let ctx = zmq::Context::new();
    let pub_soc = match ctx.socket(zmq::PUB) {
        Ok(socket) => socket,
        Err(_) => panic!("could not initialize socket"),
    };
    pub_soc.bind("tcp://127.0.0.1:3000").unwrap();

    loop {
        let msg = FAKE_MESSAGES[rand::random_range(0..FAKE_MESSAGES.len())];
        let usr = FAKE_USERS[rand::random_range(0..FAKE_USERS.len())];
        let final_msg = format!("{}: {}", usr, msg);

        //println!("{}", final_msg);

        pub_soc.send(&final_msg, 0).unwrap();

        sleep(time::Duration::from_secs(1));
    }
}
