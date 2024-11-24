use std::{collections::HashMap, io::stdin};

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

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users_vec() -> Vec<User> {
    vec![
        User::new("admin", "abc", LoginRole::Admin),
        User::new("user", "xyz", LoginRole::User),
        User::new("osaretin", "TestPassword@", LoginRole::User),
    ]
}

pub fn get_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    let vec_users = get_users_vec();
    for user in vec_users {
        let user = user;
        users.insert(user.username.to_string(), user.clone());
    }
    users
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
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
