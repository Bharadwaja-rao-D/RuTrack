use std::{collections::HashMap, sync::Arc};

use futures::StreamExt;
use futures_channel::mpsc::UnboundedSender;
use log::info;
use rutrack::{error::RutrackError, message::MessageType, opts::Opts};
use structopt::StructOpt;
use tokio::net::{TcpListener, TcpStream};

type Tx = UnboundedSender<MessageType>;

//TODO: Pool of Tx ?
pub type Clients = Arc<HashMap<String, Tx>>;

pub async fn handle_connection(stream: TcpStream) -> Result<(), RutrackError> {
    //upgrade it to the websocket

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("websocket handshake failed");

    info!("websocket handshake done");

    let (outgoing, incoming) = ws_stream.split();

    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), RutrackError> {
    let opts = Opts::from_args();

    let _clients: Clients = Arc::new(HashMap::new());

    let listener = TcpListener::bind(opts.address)
        .await
        .expect("Binding failed");

    while let Ok((stream, addr)) = listener.accept().await {
        info!("Connected to {}", addr);
        handle_connection(stream).await?;
    }

    return Ok(());
}
