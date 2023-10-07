#[derive(Debug)]
pub struct GroundStation {}

impl GroundStation {
    pub fn new() -> GroundStation {
        GroundStation {}
    }
    pub fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg)
    }

    pub fn connect(&self, sat_id: SatID) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

#[derive(Debug)]
pub struct CubeSat {
    pub id: u64,
}

impl CubeSat {
    pub fn new(id: u64) -> CubeSat {
        CubeSat { id }
    }

    pub fn receive(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
pub struct Mailbox {
    pub messages: Vec<Message>,
}

impl Mailbox {
    pub fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if recipient.id == self.messages[i].to {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

type Content = String;

#[derive(Debug)]
pub struct Message {
    pub to: u64,
    pub content: Content,
}

#[derive(Debug)]
pub enum StatusMessage {
    Ok,
}

pub fn check_status(sat: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat, StatusMessage::Ok);

    sat
}

type SatID = u64;
pub fn fetch_sat_ids() -> Vec<SatID> {
    vec![1, 2, 3]
}
