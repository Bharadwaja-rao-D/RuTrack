use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

use crate::message::Message;

#[derive(Debug, Deserialize, Serialize)]
pub struct Users {
    pub users_table: Mutex<HashMap<String, String>>, //uname, password
    pub messages_table: Mutex<HashMap<String, Vec<Message>>>, //uname, vec of messages
}

pub enum DBMessage {
    Success(String),
    Error(String),
}

impl Users {
    pub fn new() -> Self {
        return Users {
            users_table: Mutex::new(HashMap::new()),
            messages_table: Mutex::new(HashMap::new()),
        };
    }

    pub fn add_user(self, uname: &str, pass: &str) -> DBMessage {
        let mut users_table = self.users_table.lock().unwrap();
        let mut msg_table = self.messages_table.lock().unwrap();
        if users_table.contains_key(uname) {
            return DBMessage::Error("user is already present".to_string());
        } else {
            users_table
                .insert(uname.to_string(), pass.to_string())
                .unwrap();
            msg_table.insert(uname.to_string(), vec![]).unwrap();
            return DBMessage::Success(format!("Added user {}", uname));
        }
    }

    pub fn verify_user(self, uname: &str, pass: &str) -> DBMessage {
        let users_table = self.users_table.lock().unwrap();
        if let Some(password) = users_table.get(uname) {
            if pass == password {
                return DBMessage::Success(format!("User {} logged in ", uname));
            } else {
                return DBMessage::Error("Invalid password".to_string());
            }
        } else {
            return DBMessage::Error(format!("User {} not found", uname));
        }
    }

    pub fn add_msg(self, uname: &str, msg: Message) -> DBMessage {
        let mut msg_table = self.messages_table.lock().unwrap();
        if let Some(msgs) = msg_table.get_mut(uname) {
            msgs.push(msg);
            return DBMessage::Success(format!("Msg added to the {} table", uname));
        } else {
            return DBMessage::Error(format!("Failed to add msg to the {} table", uname));
        }
    }

    pub fn get_msgs(self, uname: &str) -> Option<Vec<Message>> {
        let mut msg_table = self.messages_table.lock().unwrap();
        return msg_table.remove(uname);
    }
}
