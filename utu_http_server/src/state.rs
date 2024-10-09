use std::sync::Arc;

use bitcoincore_rpc::{Auth, Client};

use crate::logger::Logger;

use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref RPC_URL: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
    static ref RPC_USER: String =
        env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    static ref RPC_PASS: String =
        env::var("BITCOIN_RPC_PASS").expect("BITCOIN_RPC_PASS must be set");
}

pub struct AppState {
    pub logger: Logger,
    pub client: Client,
}

impl AppState {
    pub async fn load() -> Arc<Self> {
        // Use the environment variables for RPC configuration
        let rpc_auth = Auth::UserPass(RPC_USER.to_string(), RPC_PASS.to_string());

        // Initialize the Bitcoin Core Client
        let client = Client::new(&RPC_URL, rpc_auth).expect("Failed to create RPC client");

        Arc::new(AppState {
            logger: Logger::new(),
            client,
        })
    }
}
