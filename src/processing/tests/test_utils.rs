use crate::models::action::ActionType;
use crate::models::state::{Layer, NeighborMap, Pearl, Worker, Workers};
use super::super::utils::{build_neighbor_graph, get_empty_neighbors, get_time_to_process, get_worker_ids, get_worker_pearl_counts, make_nom, make_pass};
use crate::{models::ability_map::AbilityMap, processing::utils::get_ability_map};

/// Returns a simple Pearl object for use in testing
fn basic_pearl () -> Pearl {
    let p: Pearl = Pearl{
        id: 12345,
        layers: vec![
            Layer{color: String::from("Green"), thickness: 12}]
        };

    return p;
}

/// Returns a simple Workers object for use in testing
fn basic_workers () -> Workers {
    let w: Workers = vec![
        Worker{
            id: 0,
            desk: vec![],
            flavor: String::from("Vector")
        },
        Worker{
            id: 1,
            desk: vec![
                basic_pearl(),
                Pearl{id: 67890, layers: vec![]}
            ],
            flavor: String::from("Matrix")
        }
    ];

    return w;
}

#[test]
fn test_ability_map_values() {
    // Tests that we can index into an ability map properly
    // and that it grabs the correct values

    let map: AbilityMap = get_ability_map();
    assert_eq!(map["Vector"]["Green"], 5);
}

#[test]
fn test_worker_ids() {
    let ids = get_worker_ids(&basic_workers());
    
    assert_eq!(ids, [0, 1].to_vec());
}

#[test]
fn test_empty_workers() {
    let ids = get_worker_ids(&[].to_vec());
    let empty_vec: Vec<u32> = [].to_vec();

    assert_eq!(ids, empty_vec);
}

#[test]
fn test_worker_pearl_counts() {
    let pc = get_worker_pearl_counts(&basic_workers());

    assert_eq!(pc[&0], 0);
    assert_eq!(pc[&1], 2);
}

#[test]
fn test_neighbor_graph() {
    let ng = build_neighbor_graph(&vec![vec![0, 1]], &basic_workers());

    assert_eq!(ng[&0], vec![1]);
    assert_eq!(ng[&1], vec![0]);
}

#[test]
#[should_panic]
fn test_neighbor_graph_bad_neighbor_list() {
    let nm: NeighborMap = vec![
        vec![0, 1],
        vec![1, 2]
    ];
    let _ng = build_neighbor_graph(&nm, &basic_workers());
}

#[test]
fn test_make_pass() {
    let p = make_pass(0, 1, 2);

    match p {
        ActionType::Pass(_a) => {}
        ActionType::Nom(_a) => {panic!();}
    }
}

#[test]
fn test_make_nom() {
    let n = make_nom(0, 1);

    match n {
        ActionType::Pass(_a) => {panic!();}
        ActionType::Nom(_a) => {}
    }
}

#[test]
fn test_get_time_to_process() {
    let map = get_ability_map();

    let t1 = get_time_to_process(&basic_pearl(), &basic_workers()[0], &map);
    assert_eq!(t1, 3);

    let t2 = get_time_to_process(&basic_pearl(), &basic_workers()[1], &map);
    assert_eq!(t2, 6);
}

#[test]
fn test_get_empty_neighbors() {
    let pc = get_worker_pearl_counts(&basic_workers());
    let ng = build_neighbor_graph(&vec![vec![0, 1]], &basic_workers());

    let empty_vec: Vec<u32> = [].to_vec();

    let en0 = get_empty_neighbors(&basic_workers()[0], &pc, &ng);
    assert_eq!(en0, empty_vec);

    let en1 = get_empty_neighbors(&basic_workers()[1], &pc, &ng);
    assert_eq!(en1, vec![0]);
}