use cubesat::{check_status, fetch_sat_ids, CubeSat, GroundStation, Mailbox, Message};

fn main() {
    let base = GroundStation::new();

    let mut mailbox = Mailbox { messages: vec![] };

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = Message {
            to: sat.id,
            content: format!("Hello sat {}", sat.id),
        };
        base.send(&mut mailbox, msg)
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);

        let msg = sat.receive(&mut mailbox);

        println!("{:?}: {:?}", sat, msg);
    }
}
