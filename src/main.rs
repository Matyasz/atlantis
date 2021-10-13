pub mod models;

pub mod processing;
use processing::pearl_pipeline::run_pearl_processing;

/// Main method for program, simply runs the `run_pearl_processing`
/// method defined in the `processing` module.
fn main() {
    // loop {
    //     let x = "{}";
    //     print!("{}", x);
    // }

    run_pearl_processing();
}
