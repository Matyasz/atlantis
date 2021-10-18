pub mod models;

pub mod processing;
use processing::pearl_pipeline::run_pearl_processing;

#[cfg(test)]
mod tests;

/// Main method for program, simply runs the `run_pearl_processing`
/// method defined in the `processing` module.
fn main() {
    run_pearl_processing();
}
