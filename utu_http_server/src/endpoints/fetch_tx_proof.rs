use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_auto_routes::route;
use bitcoin::{block::Header, BlockHash};
use bitcoincore_rpc::RpcApi;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use utu_core::{transactions::compute_merkle_branch, types::MerkleBranch};

use crate::{errors::ApiError, utils::type_conversion::ToUtuCoreTxIdList};
use crate::{state::AppState, utils::type_conversion::ToUtuCoreTxId};

#[derive(Debug, Deserialize)]
pub struct TxProofQuery {
    #[serde(deserialize_with = "deserialize_txid")]
    txid: bitcoin::Txid,
}

fn deserialize_txid<'de, D>(deserializer: D) -> Result<bitcoin::Txid, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let txid = bitcoin::Txid::from_str(&s).map_err(serde::de::Error::custom)?;
    Ok(txid)
}

#[derive(Debug, Serialize)]
struct TxProofResponse {
    block_hash: BlockHash,
    block_header: Header,
    merkle_branch: HexMerkleBranch,
}

#[derive(Debug)]
struct HexMerkleBranch(MerkleBranch);

impl Serialize for HexMerkleBranch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let hex_branch: Vec<(String, bool)> = self
            .0
            .as_vec()
            .iter()
            .map(|(hash, flag)| {
                (
                    // This will be displayed in little-edian
                    // as this is standard for Bitcoin
                    hash.to_string(),
                    *flag,
                )
            })
            .collect();
        hex_branch.serialize(serializer)
    }
}

impl IntoResponse for TxProofResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[route(get, "/fetch_tx_proof")]
async fn fetch_tx_proof(
    Query(query): Query<TxProofQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<TxProofResponse, ApiError> {
    let tx_info = state.client.get_raw_transaction_info(&query.txid, None)?;
    let block_hash = tx_info.blockhash.ok_or(ApiError::TransactionNotFound)?;

    // Fetch the full block
    let block: bitcoin::Block = state.client.get_block(&block_hash)?;

    // Generate the Merkle proof using the utility function
    let merkle_branch = compute_merkle_branch(&block.txdata.utuize(), &query.txid.utuize());

    Ok(TxProofResponse {
        block_header: block.header,
        block_hash: block.block_hash(),
        merkle_branch: HexMerkleBranch(merkle_branch),
    })
}
