use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    io::stdin,
    path::Path,
};

pub fn return_str(name: &str) -> String {
    let mut new_str = String::from("Hello ");
    new_str.push_str(name);
    return new_str;
}

pub fn read_input(label: &str) -> String {
    let mut value = String::new();
    println!("Enter {label}");
    match stdin().read_line(&mut value) {
        Ok(value) => value,
        Err(_) => {
            panic!("Please input a value")
        }
    };
    return value.trim().to_string();
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

pub fn save_users(users: HashMap<String, User>) {
    let file_path = Path::new("users.json");
    let users_string = serde_json::to_string(&users).unwrap();
    write(file_path, users_string).unwrap();
}

pub fn get_users_from_json() -> HashMap<String, User> {
    let file_path = Path::new("users.json");
    let file = read_to_string(file_path).unwrap();
    let users: HashMap<String, User> = serde_json::from_str(&file).unwrap();
    users
}

pub fn add_user(username: String, password: String, admin: bool) {
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    let mut users = get_users();
    users.insert(username, user);
    save_users(users);
}

pub fn delete_user(username: String) {
    let mut users = get_users();
    match users.remove(&username) {
        Some(user) => user,
        None => {
            println!("User does not exist");
            return ();
        }
    };
    save_users(users);
}

pub fn update_user(username: String, new_username: Option<String>, new_password: Option<String>) {
    let mut users = get_users();
    match users.get_mut(&username) {
        Some(user) => {
            // Check if New Password was given
            match new_password {
                Some(password) => user.password = hash_password(&password),
                None => (),
            }

            // Check if New Username was given
            match new_username {
                Some(username) => user.username = username,
                None => (),
            }

            save_users(users);
        }
        None => {
            println!("User not found");
            return ();
        }
    };
}

pub fn get_users_vec() -> Vec<User> {
    vec![
        User::new("admin", "abc", LoginRole::Admin),
        User::new("user", "xyz", LoginRole::User),
        User::new("osaretin", "TestPassword@", LoginRole::User),
    ]
}

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    let vec_users = get_users_vec();
    for user in vec_users {
        let user = user;
        users.insert(user.username.to_string(), user.clone());
    }
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users = get_default_users();
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load File
        let file_string = read_to_string(users_path).unwrap();
        let json_str: HashMap<String, User> = serde_json::from_str(&file_string).unwrap();
        json_str
    } else {
        let user_json = serde_json::to_string(&users).unwrap();
        write(users_path, user_json).unwrap();
        users
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);
    let users = get_users();
    // match users.iter().find(|user| user.username == username) {
    //     Some(user) => {
    //         if user.password == password {
    //             return Some(LoginAction::Granted(user.role.clone()));
    //         }
    //         return Some(LoginAction::Denied);
    //     }
    //     None => return Some(LoginAction::Denied),
    // };

    match users.get(&username.to_lowercase()) {
        Some(user) => {
            if user.password == password {
                Some(LoginAction::Granted(user.role.clone()))
            } else {
                Some(LoginAction::Denied)
            }
        }
        None => Some(LoginAction::Denied),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_return_str() {
        let result = return_str("Osaretin Frank");
        assert_eq!(result, String::from("Hello Osaretin Frank"))
    }

    #[test]
    pub fn test_login() {
        assert_eq!(
            login("AdMin", "abc"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(login("anyother", "password"), Some(LoginAction::Denied));
        assert_eq!(
            login("user", "xyz"),
            Some(LoginAction::Granted(LoginRole::User))
        );
    }
}
