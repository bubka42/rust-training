use project::{new_drat_state_pair, process_message_list};

fn main() {
    let messages = vec![
        (true, b"Hello Bob!"),
        (true, b"How are u?"),
        (false, b"Hi, Alice!"),
        (false, b"I am fine!"),
        (true, b"Wat's new?"),
    ];
    let (mut alice_state, mut bob_state) = new_drat_state_pair();
    println!("Processing messages...");
    process_message_list(&mut alice_state, &mut bob_state, messages);
}
