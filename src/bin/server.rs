use std::{sync::Arc, collections::HashMap};

use rutrack::{db::Users, message::MessageType};
use futures_channel::mpsc::UnboundedSender;

type Tx = UnboundedSender<MessageType>;

pub type Clients = Arc<HashMap<Users, Tx>>;

#[tokio::main]
async fn main(){
    let clients: Clients = Arc::new(HashMap::new());
}

