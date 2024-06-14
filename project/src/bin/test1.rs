use project::{new_drat_state_pair, process_message_list};

fn main() {
    let messages = vec![
        (true, b"Hello Bob!".as_ref()),
        (true, b"How are you?".as_ref()),
        (false, b"Hi, Alice!".as_ref()),
        (false, b"I am fine, thanks!".as_ref()),
        (true, b"So, what's new?".as_ref()),
    ];
    let (mut alice_state, mut bob_state) = new_drat_state_pair();
    println!("Processing messages...");
    process_message_list(&mut alice_state, &mut bob_state, messages);
}
