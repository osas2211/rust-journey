use authentication::{add_user, delete_user, get_users, update_user};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of User to greet
    #[arg(long, short, default_value_t = String::from("Frank"))]
    name: String,
    /// Number of times to greet
    #[arg(long, short, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ///List all users
    List,

    /// Add a user
    Add {
        /// User username
        username: String,

        /// User password
        password: String,

        /// Optional - Is user an admin
        #[arg(long, short)]
        admin: Option<bool>,
    },

    /// Delete a User
    Delete {
        /// User to delete
        username: String,
    },

    /// Update a user
    Update {
        /// User to update
        username: String,

        /// New User name
        #[arg(long)]
        new_username: Option<String>,

        /// New Password
        #[arg(long)]
        new_password: Option<String>,
    },
}

fn list_users() {
    println!("{:40} {:20}", "Username", "Role",);
    println!("{:-<70}", "");
    let users = get_users();
    users.iter().for_each(|(_, user)| {
        println!("{:40} {:20?}", user.username, user.role);
    });
}
fn main() {
    let cli = Args::parse();
    // for _ in 0..cli.count {
    //     println!("{}", cli.name)
    // }

    match cli.command {
        Some(Commands::List) => {
            list_users();
        }

        #[allow(unused)]
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => {
            add_user(username, password, admin.unwrap_or(false));
        }

        Some(Commands::Delete { username }) => {
            delete_user(username);
        }

        Some(Commands::Update {
            username,
            new_username,
            new_password,
        }) => {
            update_user(username, new_username, new_password);
        }

        None => {
            println!("Run with -- --help to see instructions")
        }
    };
}
