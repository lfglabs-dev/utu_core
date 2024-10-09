pub trait ToUtuCoreTxId {
    fn utuize(&self) -> utu_core::types::TxId;
}

impl ToUtuCoreTxId for bitcoin::Txid {
    fn utuize(&self) -> utu_core::types::TxId {
        utu_core::types::TxId::new(self.to_raw_hash().as_ref()).unwrap()
    }
}

pub trait ToUtuCoreTxIdList {
    fn utuize(&self) -> utu_core::types::TxIdList;
}

impl ToUtuCoreTxIdList for Vec<bitcoin::Transaction> {
    fn utuize(&self) -> utu_core::types::TxIdList {
        let mut txid_list = utu_core::types::TxIdList::new();
        for tx in self {
            txid_list.add(tx.compute_txid().utuize());
        }
        txid_list
    }
}
