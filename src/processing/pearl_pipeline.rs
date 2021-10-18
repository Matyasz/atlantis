use core::panic;
use serde_json;
use std::collections::HashMap;
use std::io::{self, BufRead};

use super::utils::{build_neighbor_graph, determine_actions, get_ability_map, get_action_string};
use crate::models;
use models::ability_map::AbilityMap;
use models::state::State;

/// Runs the main loop for the pearl processing pipeline, which consists of
/// repeatedly reading lines form `stdin`, converting that JSON-like string
/// into our custom State type, and then passing that to the worker method
/// `determine_action`
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * ()
pub fn run_pearl_processing() {
    let ability_map: AbilityMap = get_ability_map();
    let mut pearl_paths: HashMap<u32, Vec<u32>> = HashMap::new();

    // Variables to handle looping over inputs from stdin until hitting the end
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    let mut eof = false;

    while !eof {
        match handle.read_line(&mut line) {
            Ok(0) => {
                // read_line returns the number of bytes read before finding the next line end.
                // So if it reads 0 bytes, then we know that nothing was there.
                eof = true;
            }
            Ok(_) => {
                let data: State = serde_json::from_str(&line).unwrap();
                let neighbor_graph = build_neighbor_graph(&data.neighbor_map, &data.workers);

                let actions =
                    determine_actions(&data, &ability_map, &neighbor_graph, &mut pearl_paths);

                let action_str = get_action_string(actions);
                println!("{}", action_str);

                line.clear();
            }
            Err(e) => {
                panic!("Error reading from stdin: {}", e);
            }
        }
    }
}
