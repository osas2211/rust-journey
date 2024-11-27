use authentication::{self, get_users, login, read_input, LoginAction, LoginRole};
fn main() {
    let mut tries: u8 = 0;
    loop {
        let username = read_input("Username");
        let password = read_input("Password");
        let login_response = login(&username, &password);
        match login_response {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Welcome Admin!"),
                    LoginRole::User => println!("Welcome User!"),
                }

                break;
            }

            Some(LoginAction::Denied) => {
                tries += 1;
                println!("Login Tries {tries}/3");
                if tries > 3 {
                    println!("Too many failed login attempts!");
                    break;
                }
            }

            None => println!("New User"),
        }
    }

    let users = get_users();
    let usernames: Vec<String> = users
        .into_iter()
        .map(|user| user.1.username.clone())
        .collect();
    println!("{usernames:?}");
}
