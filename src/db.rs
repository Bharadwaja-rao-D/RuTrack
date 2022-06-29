use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Users{
    pub users_table: Mutex<HashMap<String, String>>
}

pub enum LoginMessage{
    Success(String), 
    Error(String)
}

impl Users{
    pub fn new() -> Self{
        return Users{ users_table: Mutex::new(HashMap::new())};
    }

    pub fn add_user(self, uname: &str, pass: &str) -> LoginMessage{
        let mut users_table = self.users_table.lock().unwrap();
        if users_table.contains_key(uname) {
            return LoginMessage::Error("user is already present".to_string());
        }else{
            users_table.insert(uname.to_string(), pass.to_string()).unwrap();
            return LoginMessage::Success(format!("Added user {}", uname));
        }
    }

    pub fn verify_user(self, uname: &str, pass: &str) -> LoginMessage{
        let users_table = self.users_table.lock().unwrap();
        if let Some(password) = users_table.get(uname) {
           if pass == password {
              return LoginMessage::Success(format!("User {} logged in ", uname));
           }else{
            return LoginMessage::Error("Invalid password".to_string());
           } 
        }else {
            return LoginMessage::Error(format!("User {} not found", uname));
        }
    }

}
