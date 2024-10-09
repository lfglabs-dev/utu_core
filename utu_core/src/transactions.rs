use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

use crate::types::{MerkleBranch, TxId, TxIdList};

fn double_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(Sha256::digest(data));
    hasher.finalize().into()
}

#[wasm_bindgen]
pub fn compute_merkle_root_from_branch(tx_id: &TxId, merkle_branch: &MerkleBranch) -> TxId {
    let mut current_hash = tx_id.bytes().to_vec();

    for (hash, is_left) in merkle_branch.as_vec() {
        let branch_hash_bytes = hash.bytes().to_vec();

        let concatenated = if *is_left {
            [branch_hash_bytes.as_slice(), current_hash.as_slice()].concat()
        } else {
            [current_hash.as_slice(), branch_hash_bytes.as_slice()].concat()
        };

        current_hash = double_sha256(&concatenated).to_vec();
    }

    TxId::new(&current_hash).unwrap()
}

#[wasm_bindgen]
pub fn compute_merkle_branch(txids: &TxIdList, txid: &TxId) -> MerkleBranch {
    let mut level: Vec<Vec<u8>> = txids.iter().map(|t| t.bytes().to_vec()).collect();
    let mut index = level
        .iter()
        .position(|t| t == &txid.bytes().to_vec())
        .expect("The transaction ID is not in the list of transaction IDs.");

    let mut branch = MerkleBranch::new();

    while level.len() > 1 {
        if level.len() % 2 != 0 {
            level.push(level.last().unwrap().clone());
        }

        let mut new_level = Vec::with_capacity(level.len() / 2);

        for chunk in level.chunks(2) {
            let left = &chunk[0];
            let right = &chunk[1];

            let concatenated: Vec<u8> = [left, right].into_iter().flatten().copied().collect();
            let hash = double_sha256(&concatenated).to_vec();

            new_level.push(hash);

            if chunk.contains(&level[index]) {
                let (sibling, is_left) = if index % 2 == 0 {
                    (right, false)
                } else {
                    (left, true)
                };
                branch.add(TxId::new(sibling).unwrap(), is_left);
            }
        }

        level = new_level;
        index /= 2;
    }

    branch
}

#[cfg(test)]
mod tests {
    use crate::types::TxIdList;

    use super::*;

    #[test]
    fn test_compute_merkle_root_from_branch() {
        let tx_id = TxId::from_little_endian(
            "fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4",
        )
        .unwrap();
        let merkle_branch = MerkleBranch::from_vec(vec![
            (
                TxId::from_little_endian(
                    "8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87",
                )
                .unwrap(),
                true,
            ),
            (
                TxId::from_little_endian(
                    "8e30899078ca1813be036a073bbf80b86cdddde1c96e9e9c99e9e3782df4ae49",
                )
                .unwrap(),
                false,
            ),
        ]);

        let expected_merkle_root = TxId::from_little_endian(
            "f3e94742aca4b5ef85488dc37c06c3282295ffec960994b2c0d5ac2a25a95766",
        )
        .unwrap();

        let computed_merkle_root = compute_merkle_root_from_branch(&tx_id, &merkle_branch);
        assert_eq!(
            computed_merkle_root.bytes(),
            expected_merkle_root.bytes(),
            "Merkle root from branch computation failed."
        );
    }

    #[test]
    fn test_compute_merkle_branch() {
        let txids = TxIdList::from_vec(
            vec![
                "8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87",
                "fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4",
                "6359f0868171b1d194cbee1af2f16ea598ae8fad666d9b012c8ed2b79a236ec4",
                "e9a66845e05d5abc0ad04ec80f774a7e585c6e8db975962d069a522137b80c1d",
            ]
            .into_iter()
            .map(|s| TxId::from_little_endian(s).unwrap())
            .collect(),
        );

        let tx_id = TxId::from_little_endian(
            "fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4",
        )
        .unwrap();
        let expected_merkle_branch = MerkleBranch::from_vec(vec![
            (
                TxId::from_little_endian(
                    "8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87",
                )
                .unwrap(),
                true,
            ),
            (
                TxId::from_little_endian(
                    "8e30899078ca1813be036a073bbf80b86cdddde1c96e9e9c99e9e3782df4ae49",
                )
                .unwrap(),
                false,
            ),
        ]);

        let computed_merkle_branch = compute_merkle_branch(&txids, &tx_id);
        assert_eq!(
            computed_merkle_branch.as_vec(),
            expected_merkle_branch.as_vec(),
            "Merkle branch computation failed."
        );
    }
}
