use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    futures::StreamExt,
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    NetworkBehaviour, PeerId, Transport,
};
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::{fs, io::AsyncBufReadExt, sync::mpsc};

const STORAGE_FILE_PATH: &str = "./message.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
type Messages = Vec<Message>; // Changed 'message' to 'Messages' for clarity

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("message"));

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: usize,
    name: String, // Added comma between fields
    sender: String,
    content: String,
    timestamp: u64,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse {
    mode: ListMode,
    data: Messages, // Changed 'message' to 'Messages'
    receiver: String,
}

enum EventType {
    Response(ListResponse),
    Input(String),
}

#[derive(NetworkBehaviour)]
struct MessageBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
    #[behaviour(ignore)]
    response_sender: mpsc::UnboundedSender<ListResponse>,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                if let Ok(resp) = serde_json::from_slice::<ListResponse>(&msg.data) {
                    if resp.receiver == PEER_ID.to_string() {
                        info!("Response from {}:", msg.source);
                        resp.data.iter().for_each(|r| info!("{:?}", r));
                    }
                } else if let Ok(req) = serde_json::from_slice::<ListRequest>(&msg.data) {
                    match req.mode {
                        ListMode::ALL => {
                            info!("Received ALL req: {:?} from {:?}", req, msg.source);
                            respond_with_public_message(
                                self.response_sender.clone(),
                                msg.source.to_string(),
                            );
                        }
                        ListMode::One(ref peer_id) => {
                            if peer_id == &PEER_ID.to_string() {
                                info!("Received req: {:?} from {:?}", req, msg.source);
                                respond_with_public_message(
                                    self.response_sender.clone(),
                                    msg.source.to_string(),
                                );
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

// Function to respond with public messages
fn respond_with_public_message(sender: mpsc::UnboundedSender<ListResponse>, receiver: String) {
    tokio::spawn(async move {
        match read_local_messages().await {
            Ok(messages) => {
                let resp = ListResponse {
                    mode: ListMode::ALL,
                    receiver,
                    data: messages.into_iter().filter(|m| m.public).collect(), // Changed 'r' to 'm'
                };
                if let Err(e) = sender.send(resp) {
                    error!("error sending response via channel, {}", e);
                }
            }
            Err(e) => error!("error fetching local messages to answer ALL request, {}", e),
        }
    });
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(discovered_list) => {
                for (peer, _addr) in discovered_list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(expired_list) => {
                for (peer, _addr) in expired_list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

// Function to create a new message
async fn create_new_message(name: &str, sender: &str, content: &str) -> Result<()> {
    let mut local_messages = read_local_messages().await?; // Changed 'message' to 'messages'
    let new_id = match local_messages.iter().max_by_key(|m| m.id) {
        Some(v) => v.id + 1,
        None => 0,
    };
    local_messages.push(Message {
        id: new_id,
        name: name.to_owned(),
        sender: sender.to_owned(),
        content: content.to_owned(),
        timestamp: 0,
        public: false,
    });
    write_local_messages(&local_messages).await?; // Changed 'message' to 'messages'

    info!("Created message:");
    info!("Name: {}", name);
    info!("sender: {}", sender);
    info!("content: {}", content);

    Ok(())
}

// Function to publish a message
async fn publish_message(id: usize) -> Result<()> {
    let mut local_messages = read_local_messages().await?; // Changed 'message' to 'messages'
    local_messages
        .iter_mut()
        .filter(|m| m.id == id)
        .for_each(|m| m.public = true); // Changed 'r' to 'm'
    write_local_messages(&local_messages).await?; // Changed 'message' to 'messages'
    Ok(())
}

// Function to read local messages
async fn read_local_messages() -> Result<Messages> { // Changed 'message' to 'Messages'
    let content = fs::read(STORAGE_FILE_PATH).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}

// Function to write local messages
async fn write_local_messages(messages: &Messages) -> Result<()> { // Changed 'message' to 'Messages'
    let json = serde_json::to_string(&messages)?;
    fs::write(STORAGE_FILE_PATH, &json).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Peer Id: {}", PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel::<String>();

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("can create auth keys");

    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated());
}