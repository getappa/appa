extern crate appa;

use appa::processor;

#[test]
fn should_processor_work_for_python_task() {
    let cmd = processor::Command{
        command: String::from("python"),
        path: String::from("tests/mocks/tasks/task.py"),
        data: String::from("1")
    };

    assert_eq!(processor::run(cmd));
}