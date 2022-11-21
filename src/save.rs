pub fn add_task(task: String, mut task_list: Vec<String>) -> Vec<String> {
    task_list.push(task);
    return task_list;
}
pub fn delete_task(mut task_list: Vec<String>, row: usize) {
    task_list.remove(row);
}
pub fn list(task_list: Vec<String>) {
    for x in task_list {
        println!("{}", x)
    }
}
