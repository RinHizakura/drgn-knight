use drgn_knight::*;

fn find_member(task: &Object, path: String) {
    if let Ok(obj) = task.deref_member(path.clone()) {
        if let Ok(n) = obj.to_num() {
            println!("Find {}: {:x}", path, n);
        } else {
            println!("Find {} to_num failed", path);
        }
    } else {
        println!("Find {} to_num failed", path);
    }
}

fn main() {
    let prog = Program::new();

    let task = prog.find_task(1);
    find_member(&task, "on_cpu".to_string());
    find_member(&task, "pid".to_string());
    find_member(&task, "se".to_string());
}
