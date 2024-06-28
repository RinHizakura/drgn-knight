use drgn_knight::*;

fn main() {
    let prog = Program::new();

    let task = prog.find_task(1);
    task.find_member("on_cpu".to_string());
}
