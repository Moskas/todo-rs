use clap::Parser;
mod save;
mod startup;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List tasks
    #[arg(short, long, default_value_t = true)]
    list_tasks: bool,
    /// Add task
    #[arg(short, long, default_value_t = String::new(), hide_default_value = true)]
    task: String,
    /// Delete task
    #[arg(short, long, default_value_t = 0)]
    index: i32,
}
// TODO create a sql like database to store current tasks
fn main() {
    let current_tasks: Vec<String> = startup::start();
    let args = Args::parse();
    if args.index >= 0 {
        save::delete_task(current_tasks.clone(), args.index as usize);
        //save::list(current_tasks.clone());
    }
    if args.task != String::new() {
        println!("Added {}!", args.task);
        save::add_task(args.task, current_tasks.clone());
    }
    if args.list_tasks {
        println!("Current tasks: ");
        save::list(current_tasks.clone());
    };
}
