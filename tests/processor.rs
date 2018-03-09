extern crate appa;

use appa::processor;

#[test]
fn should_processor_work_for_python_task() {
    let cmd = processor::Command{
        command: String::from("python"),
        path: String::from("tests/mocks/tasks/task.py"),
    };

    assert_eq!(processor::run(cmd, "1".to_string()), "test_okey 1");
}