use std::io;
use serde_json;

use crate::models;
use crate::models::state::Pearl;
use models::state::State;
use models::ability_map::{AbilityMap, get_ability_map};

/// Runs the main loop for the pearl processing pipeline, which
/// consists of repeatedly reading lines form `stdin`, converting
/// that JSON-like string into our custom State type, and then
/// passing that to the worker method `determine_action`.
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * ()
pub fn run_pearl_processing() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let map: AbilityMap = get_ability_map();
    // println!("{}", map["General"]["Red"]);

    match stdin.read_line(&mut buffer) {
        Ok(_n) => {
            let data: State = serde_json::from_str(&buffer).unwrap();

            determine_action(data, &map);
            print_action();
        },
        Err(e) => {
            print!("Error reading from stdin: {}", e);
        }
    }
}

/// The main worker method for processing the state of the pearl processing
/// pipeline.
///
/// # Arguments
///
/// * `state` - This is a State variable that represents the current
///             state of the pearl processing pipeline
///
/// * `map` - A borrowed reference to an AbilityMap that will describe
///           how quickly each flavor of nautiloid can process each color
///           of pearl
///
/// # Returns
///
/// * ()
fn determine_action(state: State, _map: &AbilityMap) -> () {
    let mut acts: Vec<&Pearl> = [].to_vec();

    for w in &state.workers {
        // println!("flavor: {} - Red value: {}", w.flavor, map[w.flavor.as_str()]["Blue"]);
        for p in &w.desk {
            acts.insert(w.id as usize, p);
        }
    }

    for a in acts {
        println!("Pearl: {}", a.id);
    }
}

/// Takes the list of actions and constructs a JSON-like string to be
/// printed to `sdtout`.
fn print_action() -> () {
    let action: &str = "";

    println!("{{}}");
}