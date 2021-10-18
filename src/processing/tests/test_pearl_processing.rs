/// Tests in this module will test the full range of the pipeline.
/// This will read sample versions of inputs (taken from actual
/// `single-run` outputs), deserialize them, then run them through
/// the pipeline.
///
/// We can then check that each of the expected instructions is in
/// the output. We can't just compare an entire raw string, since
/// sometimes the action strings get constructed (and hence
/// printed) in different orders.

#[test]
fn test_empty_output() {}

#[test]
fn test_output_with_actions() {}
