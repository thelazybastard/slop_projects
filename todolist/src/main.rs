// display options to add, remove, view tasks
// ask for user input
// fucntion to add task
use std::io;

struct Tasks {
    task: String,  
    description: String,
    start_date: String,
    end_date: String
}


fn main() {

    println!("Welcome, what would you like to do?");
    println!("A. Add Task");
    println!("B. Remove Task");
    println!("C. Update Task");
    println!("D. View Tasks");
    println!("E. Exit app");
    let mut user_choice = String::new();
    print!("Type A, B, C, D, E: ");
    io::stdin().read_line(&mut user_choice).expect("Unable to read input").to_string().trim();


    let user_tasks = Tasks {
        task: String,  
        description: String,
        start_date: String,
        end_date: String
    };
}




