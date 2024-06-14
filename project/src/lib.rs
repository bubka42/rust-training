pub mod drat;

use drat::{State, SymmKey};
use getrandom::getrandom;

pub fn new_drat_state_pair() -> (State, State) {
    let mut key_bytes = [0u8; 32];
    getrandom(&mut key_bytes).unwrap();
    let shr_k = SymmKey::new(key_bytes);
    let (bob_dhsk, bob_dhpk) = State::dh_keygen();
    (
        State::ratchet_init_alice(&shr_k, bob_dhpk),
        State::ratchet_init_bob(&shr_k, bob_dhsk, bob_dhpk),
    )
}

pub fn process_message_list(
    alice_state: &mut State,
    bob_state: &mut State,
    messages: Vec<(bool, &[u8; 10])>,
) {
    for (from_alice, message) in messages {
        if from_alice {
            println!("Alice sends: {:?}", std::str::from_utf8(message).unwrap());
            let aad = b"Empty AD";
            let (header, sent_msg) = alice_state.ratchet_encrypt(message, aad);
            let rcvd_msg = bob_state.ratchet_decrypt(&header, &sent_msg, aad);
            println!(
                "Bob receives: {:?}",
                std::str::from_utf8(&rcvd_msg).unwrap()
            );
        } else {
            println!("Bob sends: {:?}", std::str::from_utf8(message).unwrap());
            let aad = b"Empty AD";
            let (header, sent_msg) = bob_state.ratchet_encrypt(message, aad);
            let rcvd_msg = alice_state.ratchet_decrypt(&header, &sent_msg, aad);
            println!(
                "Alice receives: {:?}",
                std::str::from_utf8(&rcvd_msg).unwrap()
            );
        }
    }
}
