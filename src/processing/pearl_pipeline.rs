use serde_json;
use core::panic;
use std::collections::HashMap;
use std::io::{self, BufRead};

use super::utils::{build_neighbor_graph, get_ability_map, get_best_neighbor, get_best_pearl_to_nom, get_worker_pearl_counts, make_nom, make_pass};
use crate::models;
use crate::models::action::ActionType;
use crate::models::state::NeighborGraph;
use models::ability_map::AbilityMap;
use models::state::State;

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

                let actions = determine_actions(&data, &ability_map, &neighbor_graph, &mut pearl_paths);

                print_action(actions);
                line.clear();
            }
            Err(e) => {
                panic!("Error reading from stdin: {}", e);
            }
        }
    }
}

/// The main worker method for processing the state of the pearl processing
/// pipeline. This will take in the state information and determine the
/// actions the nautiloids should take.
///
/// # Arguments
///
/// * `state` - This is a State variable that represents the current
///             state of the pearl processing pipeline
///
/// * `ability_map` - A borrowed reference to an AbilityMap that will describe
///                   how quickly each flavor of nautiloid can process each color
///                   of pearl
///
/// # Returns
///
/// * ()
fn determine_actions(
    state: &State,
    ability_map: &AbilityMap,
    neighbor_graph: &NeighborGraph,
    pearl_paths: &mut HashMap<u32, Vec<u32>>
) -> HashMap<u32, ActionType> {
    let current_pearl_counts = get_worker_pearl_counts(&state.workers);
    let mut next_pearl_counts = get_worker_pearl_counts(&state.workers);
    let mut actions: HashMap<u32, ActionType> = HashMap::new();

    let mut workers_with_pearls = state.workers.clone();
    workers_with_pearls.retain(|w| current_pearl_counts[&w.id] != 0);

    for wrkr in workers_with_pearls {
        // First, check for finished pearls to pass back
        // Don't do this for the gate keeper, finished pearls stay there
        let mut already_passed = false;
        
        if wrkr.id != 0 {
            for prl in &wrkr.desk {
                if prl.layers.len() == 0 && !already_passed{
                    // Send the pearl back the way it came
                    let old_path = pearl_paths.get_mut(&prl.id);
                    let next_worker;

                    match old_path {
                        Some(pth) => {
                            next_worker = pth.pop().unwrap();
                        }
                        None => {
                            panic!("PEarl Path Error: Could not retrieve pearl's path back to the gate keeper.");
                        }
                    }

                    actions.insert(
                        wrkr.id,
                        make_pass(wrkr.id, prl.id, next_worker)
                    );
                    already_passed = true;
                }
            }
        }
        
        // Second, if the worker hasn't already decided to pass a pearl back 
        // towards the gate keeper, check for any other pearls that should be
        // passed so a neighboring worker. If it is determined that no pearls
        // should be passed, select the best pearl to nom.
        if !already_passed{
            let best_nbr = get_best_neighbor(
                state, &wrkr, &next_pearl_counts, ability_map, neighbor_graph
            );

            match best_nbr {
                Some(bn) => {
                    actions.insert(
                        wrkr.id,
                        make_pass(wrkr.id, bn.pearl_id, bn.worker_id)
                    );

                    // Update the pearl counts and paths, so the other workers
                    // know that this worker is about to receive a pearl
                    pearl_paths.entry(bn.pearl_id).or_insert(Vec::new()).push(wrkr.id);

                    next_pearl_counts.insert(wrkr.id, next_pearl_counts[&wrkr.id] - 1);
                    next_pearl_counts.insert(bn.worker_id, next_pearl_counts[&bn.worker_id] + 1);
                },
                None => {
                    actions.insert(
                        wrkr.id,
                        make_nom(wrkr.id, get_best_pearl_to_nom(&wrkr, ability_map))
                    );
                }
            }
        }
    }

    return actions;
}

/// Takes the set of actions and constructs a JSON-like string to be
/// printed to `sdtout`.
fn print_action(actions: HashMap<u32, ActionType>) -> () {
    let mut action_str = "{".to_string();

    for id in actions.keys() {
        let act = match &actions[id]{
            ActionType::Pass(n) => {
                format!(
                    "\"{id}\":{{\"Pass\":{{\"pearl_id\":{pearl_id},\"to_worker\":{to_worker}}}}}",
                    id=id, pearl_id=n.pearl_id, to_worker=n.to_id
                )
            }
            ActionType::Nom(n) => {
                format!("\"{id}\":{{\"Nom\":{pearl_id}}}", id=id, pearl_id=n.pearl_id)
            }
        };

        action_str += &act;
        action_str += ",";
    }

    if !actions.is_empty() {action_str.pop();} // remove the extra comma if there were actions to add
    action_str += "}";

    println!("{}", action_str);
}
