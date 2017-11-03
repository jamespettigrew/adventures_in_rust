use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::iter;

struct Message {
    id: usize,
    expected_fragments: usize,
    fragments: Vec<Option<String>>
}

impl Message {
    fn new(id: usize, expected_fragments: usize) -> Message {
        let fragments = iter::repeat(None)
                             .take(expected_fragments)
                             .collect::<Vec<Option<String>>>();

        Message {
            id: id,
            expected_fragments: expected_fragments,
            fragments: fragments,
        }
    }

    fn add_fragment(&mut self, fragment_id: usize, fragment: String) {
        self.fragments[fragment_id] = Some(fragment);
    }

    fn is_complete(&self) -> bool {
        self.fragments
            .iter()
            .filter(|&f| f.is_some())
            .count() == self.expected_fragments
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, option) in self.fragments.iter().enumerate() {
            let fragment = option.clone().unwrap();
            try!(writeln!(f, "{} {} {} {}", self.id, index, self.expected_fragments, fragment));
        }
        Ok(())
    }
}

struct Packet {
    id: usize,
    message_id: usize,
    total_packets: usize,
    message_fragment: String
}

fn process_line(line: String) -> Packet {
    let mut packet_data_iter = line.split_whitespace();

    // Expecting perfectly formed packets
    let message_id = packet_data_iter.next().unwrap().parse::<usize>().unwrap();
    let packet_id = packet_data_iter.next().unwrap().parse::<usize>().unwrap();
    let total_packets = packet_data_iter.next().unwrap().parse::<usize>().unwrap();

    let buf = String::new();
    let message_fragment = packet_data_iter.fold(buf, |mut message_fragment, word| { 
        message_fragment.push_str(word);
        message_fragment.push(' ');
        message_fragment
    });

    Packet {
        id: packet_id,
        message_id: message_id,
        total_packets: total_packets,
        message_fragment: message_fragment
    }
}

fn process_packet(packet: Packet, message_map: &mut HashMap<usize, Message>) {
    let message = match message_map.entry(packet.message_id) {
        Occupied(entry) => entry.into_mut(),
        Vacant(entry) => {
            let mut new_message = Message::new(packet.message_id, packet.total_packets);
            entry.insert(new_message)
        }
    };

    message.add_fragment(packet.id, packet.message_fragment);
    if message.is_complete() {
        print!("{}", message);
        let id_to_remove = message.id.clone();

        // Why is this not permitted? :'(
        // message_map.remove(&id_to_remove);
    }
}

fn main() {
    let mut message_map: HashMap<usize, Message> = HashMap::new();

    let stdin = io::stdin();
    for result in stdin.lock().lines() {
        match result {
            Ok(line) => {
                let packet = process_line(line);
                process_packet(packet, &mut message_map);
            },
            Err(error) => println!("error: {}", error),
        }
    }
}