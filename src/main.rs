use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Add task
    #[arg(short, long)]
    task: String,

    /// List tasks
    #[arg(short, long, default_value_t = false)]
    list: bool,
}
// TODO create a sql like database to store current tasks
fn main() {
    let mut current_tasks: Vec<String> = Vec::new();
    let args = Args::parse();

    println!("Added {}!", args.task);
    current_tasks.push(args.task.to_string());
    if args.list {
        println!("Current tasks: ");
        for x in &current_tasks {
            println!("{}", x);
        }
    };
}
