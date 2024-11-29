use clap::{Parser, Subcommand};
use todo_lib::{add_todo, delete_todo, get_all_todos, get_todo, update_todo};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Get all todos
    GetAllTodos,

    /// Update Todo
    UpdateTodo {
        /// Todo ID to update
        #[arg(long)]
        id: u8,

        /// New Todo
        new_todo: Option<String>,

        /// Set to completed
        #[arg(long)]
        is_completed: Option<bool>,
    },

    /// Add a Todo
    AddTodo { task: String },

    /// Delete a Todo
    DeleteTodo {
        /// Todo ID to delete
        #[arg(long)]
        id: u8,
    },

    /// Get a Todo
    GetTodo {
        /// Todo ID to get
        #[arg(long)]
        id: u8,
    },
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Command::GetAllTodos => {
            println!("Get all todos");
            let todo_data = get_all_todos();
            let todos = match todo_data.get("todos") {
                Some(todos) => todos,
                None => &vec![],
            };
            println!("{:10} {:80} {:20}", "ID", "Task", "Status");
            println!("{:-<120}", "");
            todos.iter().for_each(|todo| {
                println!("{:<10} {:<80} {:20?}", todo.id, todo.task, todo.status);
            });
        }

        Command::UpdateTodo {
            id,
            new_todo,
            is_completed,
        } => {
            update_todo(id, new_todo, is_completed);
        }

        Command::AddTodo { task } => {
            add_todo(task);
        }

        Command::DeleteTodo { id } => {
            delete_todo(id);
        }

        Command::GetTodo { id } => {
            get_todo(id);
        }
    }
}
