use serde_json;
use core::panic;
use std::collections::HashMap;
use std::io::{self, BufRead};

use super::utils::{
    build_neighbor_graph,
    get_ability_map,
    get_best_neighbor,
    get_best_pearl_to_nom,
    get_worker_pearl_counts,
    make_nom,
    make_pass};
use crate::models;
use crate::models::action::{ActionType, Nom, Pass};
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
                // let actions: HashMap<u32, ActionType> = HashMap::new();
                print_action(actions);
                // let x = "{}";
                // println!("{}", x);
                
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
    let mut pearl_counts = get_worker_pearl_counts(&state.workers);
    let mut actions: HashMap<u32, ActionType> = HashMap::new();

    for wrkr in &state.workers {
        if pearl_counts[&wrkr.id] != 0 {
            // First, check for finished pearls to pass back
            // Don't do this for the gate keeper, finished pearls stay there
            if wrkr.id != 0 {
                for prl in &wrkr.desk {
                    let already_found = false;
                    if prl.layers.len() == 0 && !already_found{
                        // Send the pearl back the way it came
                        let mut old_path = pearl_paths.get(&wrkr.id).unwrap().clone();
                        let next_wrkr = old_path.pop().unwrap();
                        pearl_paths.insert(wrkr.id, old_path);

                        actions.insert(
                            wrkr.id,
                            make_pass(wrkr.id, prl.id, next_wrkr)
                        );
                    }
                }
            } else if pearl_counts[&wrkr.id] > 1 {
                // Second, check for adjacent workers that need pearls            
                // Determine which pearl to pass to which neighbor
                let best_nbr = get_best_neighbor(state, wrkr, &pearl_counts, ability_map, neighbor_graph);

                actions.insert(
                    wrkr.id,
                    make_pass(wrkr.id, best_nbr.pearl_id, best_nbr.worker_id)
                );

                // Make sure to update the pearl counts, so the other workers
                // know that this worker is about to receive a pearl
                pearl_counts.insert(wrkr.id, pearl_counts[&wrkr.id] - 1);
                pearl_counts.insert(best_nbr.worker_id, pearl_counts[&best_nbr.worker_id] + 1);
            } else {
                // Finally, if not going to pass, determine which pearl to nom
                actions.insert(
                    wrkr.id,
                    make_nom(wrkr.id, get_best_pearl_to_nom(&wrkr, ability_map))
                );
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
