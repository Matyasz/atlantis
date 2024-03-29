use crate::models;
use crate::models::action::ActionType;
use crate::models::state::{NeighborGraph, Pearl, State, Worker, WorkerPearlIDs};
use models::ability_map::AbilityMap;
use models::action::{Nom, Pass};
use models::state::{NeighborMap, Workers};
use serde_json;
use std::collections::HashMap;

/// Reads the data from the JSON file containing information about how the
/// different flavors of nautiloid are able to dissolve different layers
/// of pearls.
///
/// Also, this method ensures that the JSON data will be included in the
/// final binary created at compile time.
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * `AbilityMap` - A data structure detailing how quickly each flavor
///                  of nautiloid can process each color of pearl
pub fn get_ability_map() -> AbilityMap {
    // This line will include the .json file in the compiled binary, allowing
    // it to run anywhere after compilation
    let json_data = include_str!("../static_files/ability_map.json");

    let map: AbilityMap = match serde_json::from_str(json_data) {
        Ok(m) => m,
        Err(e) => {
            panic!("File read error: {}", e)
        }
    };

    return map;
}

/// Takes a vector of Worker objects and returns a vector of just the IDs
///
/// # Arguments
///
/// * `Workers` - A reference to a Workers object.
///
/// # Returns
///
/// * `Vec<u32>` - A list of the IDs of the workers in the Workers input,
///                sorted by ID.
pub fn get_worker_ids(workers: &Workers) -> Vec<u32> {
    let mut worker_ids: Vec<u32> = Vec::new();

    for w in workers {
        worker_ids.push(w.id);
    }

    // Sort the IDs so that 0 (gate keeper's ID) will come first
    worker_ids.sort();
    return worker_ids;
}

/// This method creates a HashMap where the keys are the worker nautiloid IDs
/// and the values are the current number of pearls on that worker's desk.
///
/// (This method could be rewritten to use a vector instead, however that would
/// either assume that the worker IDs all come in sequential order, starting
/// at 0 and incrementing without skipping any natural numbers, or it would
/// have to be written to use unnecessary space to accommodate unused IDs.
/// For the sake of robustness and minimizing space, a HashMap is used. We
/// don't want our code failing when a nautiloid quits mid shift...)
///
/// # Arguments
///
/// * `Workers` - A reference to a Workers object.
///
/// # Returns
///
/// * `HashMap<u32, u32>` - A HashMap where the keys are worker IDs and
///                         the values are the number of pearls on that
///                         worker's desk
pub fn get_worker_pearl_counts(workers: &Workers) -> HashMap<u32, u32> {
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for w in workers {
        counts.insert(w.id, w.desk.len() as u32);
    }

    return counts;
}

/// This method will build a HashMap where the keys are the nautiloid IDs,
/// and the values are a Vec<u32> of the IDs of all the nautiloids to which
/// they are adjacent. This is intended to make looking up neighbors easier
/// when deciding to pass a pearl or not.
///
/// With this method, we can construct the map only one time, instead of
/// having to iterate over the entire neighbor map each time we want to
/// find adjacent nautiloids.
///
/// Using a method like this, the code becomes much more general and allows
/// the future option of having different flavors of worker and colors of
/// pearl, simply by updating a single JSON file.
///
/// # Arguments
///
/// * `neighbors` - This is the NeighborMap that is read directly from the
///                 raw JSON file
///
/// # Returns
///
/// * `HashMap<u32, Vec<u32>>` - A HashMap where the keys are nautiloid IDs,
///                              and the values are vectors of the IDs of
///                              the neighbor nautiloids.
pub fn build_neighbor_graph(neighbors: &NeighborMap, workers: &Workers) -> NeighborGraph {
    let mut neighbor_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let worker_ids = get_worker_ids(workers);

    for id in &worker_ids {
        let mut nb_list: Vec<u32> = Vec::new();

        for pair in neighbors {
            // check to make sure the
            for ind in [0, 1] {
                if !worker_ids.contains(&pair[ind]) {
                    panic!(
                        "Neighbor Graph Error: {} is not a valid worker ID",
                        &pair[ind]
                    );
                }
            }

            if id == &pair[0] {
                nb_list.push(pair[1]);
            } else if id == &pair[1] {
                nb_list.push(pair[0]);
            }
        }
        neighbor_map.insert(*id, nb_list);
    }

    return neighbor_map;
}

/// Takes information about a worker passing a pearl to another worker
/// and constructs an ActionType object of the Pass variant.
///
/// # Arguments
///
/// * `from_id` - The ID of the worker passing the pearl
/// * `pearl_id` - The ID of the pearl being passed
/// * `to_id` - The ID of the worker receiving the pearl
///
/// # Returns
///
/// * `ActionType::Pass` - Ano object describing the passing of a pearl
pub fn make_pass(from_id: u32, pearl_id: u32, to_id: u32) -> ActionType {
    return ActionType::Pass(Pass {
        from_id: from_id,
        pearl_id: pearl_id,
        to_id: to_id,
    });
}
/// Takes information about a worker nomming a pearl and constructs an
/// ActionType object of the Nom variant.
///
/// # Arguments
///
/// * `nautiloid_id` - The worker nomming a pearl
/// * `pearl_id` - The pearl being nommed
///
/// # Returns
///
/// * `ActionType::Nom` - Ano object describing the nomming of a pearl
pub fn make_nom(nautiloid_id: u32, pearl_id: u32) -> ActionType {
    return ActionType::Nom(Nom {
        nautiloid_id: nautiloid_id,
        pearl_id: pearl_id,
    });
}

/// Determines how long it will take a given nautiloid to process a given pearl,
/// based on the information in a given ability map.
///
/// # Arguments
///
/// * `pearl` - A reference to the pearl in question
/// * `worker` - A reference to the worker in question
/// * `ability_map` - A reference to the ability map describing how different
///                   flavors of worker can process different colors of pearls
///
/// # Returns
///
/// * `u32` - How long it would take that worker to process the pearl
pub fn get_time_to_process(pearl: &Pearl, worker: &Worker, ability_map: &AbilityMap) -> u32 {
    let mut total_time: u32 = 0;

    for layer in &pearl.layers {
        let layer_time = ((layer.thickness as f32)
            / (ability_map[&worker.flavor][&layer.color] as f32))
            .ceil() as u32;

        total_time += layer_time;
    }

    return total_time;
}

/// Given a worker, finds which of its neighbors have an empty desk
///
/// # Arguments
///
/// * `worker` - A reference to the worker in question
/// * `pearl_counts` - The number of pearls each worker has
/// * `neighbor_graph` - A HashMap detailing the neighbors of each worker
///
/// # Returns
///
/// * `Vec<u32>` - A vector of the IDs of which neighbors have an empty desk
pub fn get_empty_neighbors(
    worker: &Worker,
    pearl_counts: &HashMap<u32, u32>,
    neighbor_graph: &NeighborGraph,
) -> Vec<u32> {
    let mut empty_nbrs = neighbor_graph.get(&worker.id).unwrap().clone();
    empty_nbrs.retain(|&i| pearl_counts[&i] == 0);

    return empty_nbrs;
}

/// Given the state, and a particular worker, determine the best option
/// for passing a pearl to a neighbor.
///
/// # Arguments
///
/// * `state` - A reference to the state of the pipeline
/// * `worker` - A reference to the worker deciding who to pass to
/// * `pearl_counts` - A reference to the pearl counts of the workers
/// * `ability_map` - A reference to the ability map describing how different
///                   flavors of worker can process different colors of pearls
/// * `neighbor_graph` - A HashMap detailing the neighbors of each worker
///
/// # Returns
///
/// * `Option<WorkerPearlIDs>` - An option, returning Some means that there is
///                              a good neighbor to pass a pearl to. Returning
///                              None means that there isn't, and the worker
///                              should nom a pearl instead.
pub fn get_best_neighbor(
    state: &State,
    worker: &Worker,
    pearl_counts: &HashMap<u32, u32>,
    ability_map: &AbilityMap,
    neighbor_graph: &NeighborGraph,
) -> Option<WorkerPearlIDs> {
    let empty_neighbors = get_empty_neighbors(worker, pearl_counts, neighbor_graph);
    let mut best_pair: Option<WorkerPearlIDs> = None;

    for e_nbr_id in &empty_neighbors {
        for p in &worker.desk {
            let best_time = get_time_to_process(p, worker, ability_map);

            let mut nbr = state.workers.clone();
            nbr.retain(|w| &w.id == e_nbr_id);

            let time = get_time_to_process(p, &nbr[0], ability_map);

            // Only pass to a neighbor if they can actually process it better, OR if
            // the worker has extra pearls to work on.
            if (time < best_time) || (time == best_time && worker.desk.len() > 1) {
                best_pair = Some(WorkerPearlIDs {
                    worker_id: *e_nbr_id,
                    pearl_id: p.id,
                });
            }
        }
    }

    return best_pair;
}

/// Given a worker and their abilities, determines the best pearl to nom.
///
/// # Arguments
///
/// * `worker` - A reference to the worker deciding which pearl to nom
/// * `ability_map` - A reference to the ability map describing how different
///                   flavors of worker can process different colors of pearls
///
/// # Returns
///
/// * `u32` - The ID of the optimal pearl for the worker to nom
///
pub fn get_best_pearl_to_nom(worker: &Worker, ability_map: &AbilityMap) -> Option<u32> {
    // Remove any finished pearls first
    let mut unfinished_pearls = worker.desk.clone();
    unfinished_pearls.retain(|p| p.layers.len() != 0);

    let mut best_time: Option<u32> = None;
    let mut best_pearl_id: Option<u32> = None;

    for pearl in &unfinished_pearls {
        let time = get_time_to_process(pearl, worker, ability_map);

        match best_time {
            Some(bt) => {
                if time < bt {
                    best_pearl_id = Some(pearl.id);
                    best_time = Some(time);
                }
            }
            None => {
                best_pearl_id = Some(pearl.id);
                best_time = Some(time);
            }
        }
    }

    return best_pearl_id;
}

/// The main worker method for processing the state of the pearl processing
/// pipeline. This will take in the state information, as well as some other
/// information that will remain constant for the duration of the pipeline
/// running (the ability map, the neighbor graph) and determine the
/// action each nautiloid should take.
///
/// If a worker does not currently have a pearl, then they will not be
/// assigned an action to take.
///
/// # Arguments
///
/// * `state` - This is a reference to a State that describes the current
///             state of the pearl processing pipeline
///
/// * `ability_map` - A reference to an AbilityMap that will describe how
///                   quickly each flavor of nautiloid can process each
///                   color of pearl
/// * `neighbor_graph` - A HashMap detailing the neighbors of each worker
/// * `pearl_paths` - A HashMap detailing how each pearl has been passed
///                   around by the nautiloids
///
/// # Returns
///
/// * `HashMap<u32, ActionType>` - A HashMap where teh keys are the IDs of
///                                the workers, and the keys are ActionType
///                                objects describing what action the worker
///                                should take
pub fn determine_actions(
    state: &State,
    ability_map: &AbilityMap,
    neighbor_graph: &NeighborGraph,
    pearl_paths: &mut HashMap<u32, Vec<u32>>,
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
                if prl.layers.len() == 0 && !already_passed {
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

                    actions.insert(wrkr.id, make_pass(wrkr.id, prl.id, next_worker));
                    already_passed = true;
                }
            }
        }

        // Second, if the worker hasn't already decided to pass a pearl back
        // towards the gate keeper, check for any other pearls that should be
        // passed so a neighboring worker. If it is determined that no pearls
        // should be passed, select the best pearl to nom.
        if !already_passed {
            let best_nbr = get_best_neighbor(
                state,
                &wrkr,
                &next_pearl_counts,
                ability_map,
                neighbor_graph,
            );

            match best_nbr {
                Some(bn) => {
                    actions.insert(wrkr.id, make_pass(wrkr.id, bn.pearl_id, bn.worker_id));

                    // Update the pearl counts and paths, so the other workers
                    // know that this worker is about to receive a pearl
                    pearl_paths
                        .entry(bn.pearl_id)
                        .or_insert(Vec::new())
                        .push(wrkr.id);

                    next_pearl_counts.insert(wrkr.id, next_pearl_counts[&wrkr.id] - 1);
                    next_pearl_counts.insert(bn.worker_id, next_pearl_counts[&bn.worker_id] + 1);
                }
                None => {
                    let best_pearl = get_best_pearl_to_nom(&wrkr, ability_map);
                    match best_pearl {
                        Some(bp) => {
                            actions.insert(wrkr.id, make_nom(wrkr.id, bp));
                        }
                        None => {
                            panic!("Nom Error: Could not find a best pearl to Nom");
                        }
                    }
                }
            }
        }
    }

    return actions;
}

/// Takes the set of actions and constructs a JSON-like string to be
/// printed to `sdtout`.
///
/// # Arguments
///
/// * `actions` - The HashMAp containing the actions of all the workers that
///               will make an action this turn.
///
/// # Returns
///
/// * `String` - The items in `actions` encoded into a JSON-like String.
pub fn get_action_string(actions: HashMap<u32, ActionType>) -> String {
    let mut action_str = "{".to_string();

    for id in actions.keys() {
        let act =
            match &actions[id] {
                ActionType::Pass(n) => {
                    format!(
                    "\"{id}\":{{\"Pass\":{{\"pearl_id\":{pearl_id},\"to_worker\":{to_worker}}}}}",
                    id=id, pearl_id=n.pearl_id, to_worker=n.to_id
                )
                }
                ActionType::Nom(n) => {
                    format!(
                        "\"{id}\":{{\"Nom\":{pearl_id}}}",
                        id = id,
                        pearl_id = n.pearl_id
                    )
                }
            };

        action_str += &act;
        action_str += ",";
    }

    if !actions.is_empty() {
        action_str.pop();
    } // remove the extra comma if there were actions to add
    action_str += "}";

    return action_str;
}
