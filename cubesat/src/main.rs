use std::collections::HashMap;


#[derive(Debug, Copy, Clone)]
struct CubeSat {
    id: u64
}

impl CubeSat {
    pub fn new(id: u64) -> CubeSat {
        CubeSat { id }
    }

    pub fn recv(&mut self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct MailBox {
    messages: HashMap<u64, Vec<Message>>
}

impl MailBox {
    pub fn new() -> MailBox {
        MailBox {
            messages: Default::default()
        }
    }

    pub fn post(&mut self, msg: Message) {
        self.messages.entry(msg.to)
            .or_default()
            .push(msg)
    }

    pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        if let Some(msgs) = self.messages.get_mut(&recipient.id) {
            return msgs.pop();
        }

        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String
}

#[derive(Debug)]
enum StatusMessage {
    Ok
}

struct GroundStation;

impl GroundStation {
    pub fn connect(&self, id: u64) -> CubeSat {
        CubeSat::new(id)
    }

    pub fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};
    let mut mailbox = MailBox::new();
    
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message { to: sat.id, content: "Hello, there".to_string()};
        base.send(&mut mailbox, msg)
    }


    println!("{:?}", mailbox);

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = sat.recv(&mut mailbox);
        println!("{:?}: {:?}", sat, msg);
    }

}
