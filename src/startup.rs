use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// For future use when I implement proper task status parsing from the file
//struct Tasks {
//    index: i32,
//    status: bool,
//    content: String,
//}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn start() -> Vec<String> {
    let mut tasks_database: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("/home/moskas/.config/todo/tasks") {
        for line in lines {
            if let Ok(task) = line {
                //println!("{}", task);
                tasks_database.push(task.to_string());
            }
        }
    }
    return tasks_database;
}
