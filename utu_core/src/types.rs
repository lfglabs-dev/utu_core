use serde::Serialize;
use std::error::Error;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum TxIdError {
    InvalidLength,
    InvalidHex,
}

impl fmt::Display for TxIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxIdError::InvalidLength => write!(f, "Hex string must be exactly 64 characters"),
            TxIdError::InvalidHex => write!(f, "Invalid hex string"),
        }
    }
}

impl Error for TxIdError {}

impl From<TxIdError> for JsValue {
    fn from(err: TxIdError) -> JsValue {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TxId([u8; 32]);

#[wasm_bindgen]
impl TxId {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<TxId, JsValue> {
        if bytes.len() != 32 {
            return Err(JsValue::from_str("TxId must be exactly 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Ok(TxId(arr))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_little_endian(&self) -> String {
        let mut acc = String::with_capacity(64);
        for byte in self.0.iter().rev() {
            use std::fmt::Write;
            write!(&mut acc, "{:02x}", byte).unwrap();
        }
        acc
    }

    pub fn from_little_endian(hex_str: &str) -> Result<TxId, JsValue> {
        if hex_str.len() != 64 {
            return Err(TxIdError::InvalidLength.into());
        }
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            let byte_str = &hex_str[2 * i..2 * i + 2];
            bytes[31 - i] = u8::from_str_radix(byte_str, 16).map_err(|_| TxIdError::InvalidHex)?;
        }
        Ok(TxId(bytes))
    }
}

impl TxId {
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for TxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_little_endian())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize)]
pub struct TxIdList(Vec<TxId>);

#[wasm_bindgen]
impl TxIdList {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        TxIdList(Vec::new())
    }

    pub fn add(&mut self, txid: TxId) {
        self.0.push(txid);
    }
}

impl TxIdList {
    pub fn iter(&self) -> std::slice::Iter<TxId> {
        self.0.iter()
    }

    pub fn from_vec(vec: Vec<TxId>) -> Self {
        TxIdList(vec)
    }
}

impl Default for TxIdList {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize)]
pub struct MerkleBranch(Vec<(TxId, bool)>);

#[wasm_bindgen]
impl MerkleBranch {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        MerkleBranch(Vec::new())
    }

    pub fn add(&mut self, hash: TxId, is_left: bool) {
        self.0.push((hash, is_left));
    }
}

impl MerkleBranch {
    // not exposed to WebAssembly
    pub fn from_vec(vec: Vec<(TxId, bool)>) -> Self {
        MerkleBranch(vec)
    }

    pub fn as_vec(&self) -> &Vec<(TxId, bool)> {
        &self.0
    }
}

impl Default for MerkleBranch {
    fn default() -> Self {
        Self::new()
    }
}
