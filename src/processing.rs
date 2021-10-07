use std::io;
use serde_json;

use crate::models;
use models::state::State;

pub fn run_pearl_processing() {
    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(_n) => {
                let data: State = serde_json::from_str(&buffer).unwrap();
                process_state(data);
            }
            Err(e) => {
                print!("Error: {}", e);
            }
        }
    }
}

/// The main worker method for processing the state of the pearl processing
/// pipeline.
///
/// # Arguments
/// * `state`: - This is a State variable that has been 
pub fn process_state(state: State) -> () {
    for w in state.workers {
        println!("{}", w.id);
    }
}

// fn get_state() -> State {

// }