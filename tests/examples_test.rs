use std::process::Command;

fn run_example(example: &str) {
    let status = Command::new("cargo")
        .args(["run", "--example", example, "--quiet"])
        .status()
        .expect("failed to spawn cargo to run example");

    assert!(status.success(), "example '{}' did not run successfully", example);
}

#[test]
fn example_chat_basic_runs() {
    run_example("chat_basic");
}

#[test]
fn example_mcp_list_tools_runs() {
    run_example("mcp_list_tools");
}

#[test]
fn example_multiagent_graph_runs() {
    run_example("multiagent_graph");
}