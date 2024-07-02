use drgn_knight::*;

fn find_member(obj: &Object, path: String) {
    let obj = obj.deref_member(path.clone());
    if obj.is_err() {
        println!("Can't find {path} under the given object");
        return;
    }

    let obj = obj.unwrap();
    if let Ok(n) = obj.to_num() {
        println!("Get {}: {:x}", path, n);
    } else {
        println!("Traslate {} to_num failed", path);
    }
}

fn main() {
    let prog = Program::new();

    let pid = 1;
    let task = prog.find_task(pid);
    if task.is_err() {
        println!("Can't find task with pid {pid}");
        return;
    }

    let task = task.unwrap();
    find_member(&task, "on_cpu".to_string());
    find_member(&task, "pid".to_string());
    find_member(&task, "se".to_string());
}
