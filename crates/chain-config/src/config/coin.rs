use crate::GenesisCommitment;
use fuel_core_storage::MerkleRoot;
use fuel_core_types::{
    entities::coins::coin::CompressedCoin,
    fuel_crypto::Hasher,
    fuel_tx::{
        TxPointer,
        UtxoId,
    },
    fuel_types::{
        Address,
        AssetId,
        BlockHeight,
        Bytes32,
    },
    fuel_vm::SecretKey,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Default, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CoinConfig {
    /// auto-generated if None
    pub tx_id: Bytes32,
    pub output_index: u8,
    /// used if coin is forked from another chain to preserve id & tx_pointer
    pub tx_pointer_block_height: BlockHeight,
    /// used if coin is forked from another chain to preserve id & tx_pointer
    /// The index of the originating tx within `tx_pointer_block_height`
    pub tx_pointer_tx_idx: u16,
    pub owner: Address,
    pub amount: u64,
    pub asset_id: AssetId,
}

/// Generates a new coin config with a unique utxo id for testing
#[derive(Default, Debug)]
pub struct CoinConfigGenerator {
    count: usize,
}

impl CoinConfigGenerator {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn generate(&mut self) -> CoinConfig {
        let mut bytes = [0u8; 32];
        bytes[..std::mem::size_of::<usize>()].copy_from_slice(&self.count.to_be_bytes());

        let config = CoinConfig {
            tx_id: Bytes32::from(bytes),
            ..Default::default()
        };
        self.count = self.count.checked_add(1).expect("Max coin count reached");

        config
    }

    pub fn generate_with(&mut self, secret: SecretKey, amount: u64) -> CoinConfig {
        let owner = Address::from(*secret.public_key().hash());

        CoinConfig {
            amount,
            owner,
            ..self.generate()
        }
    }
}

impl CoinConfig {
    pub fn utxo_id(&self) -> UtxoId {
        UtxoId::new(self.tx_id, self.output_index)
    }

    pub fn tx_pointer(&self) -> TxPointer {
        TxPointer::new(self.tx_pointer_block_height, self.tx_pointer_tx_idx)
    }
}

#[cfg(all(test, feature = "random", feature = "std"))]
impl crate::Randomize for CoinConfig {
    fn randomize(mut rng: impl ::rand::Rng) -> Self {
        Self {
            tx_id: super::random_bytes_32(&mut rng).into(),
            output_index: rng.gen(),
            tx_pointer_block_height: rng.gen(),
            tx_pointer_tx_idx: rng.gen(),
            owner: Address::new(super::random_bytes_32(&mut rng)),
            amount: rng.gen(),
            asset_id: AssetId::new(super::random_bytes_32(rng)),
        }
    }
}

impl GenesisCommitment for CompressedCoin {
    fn root(&self) -> anyhow::Result<MerkleRoot> {
        let owner = self.owner();
        let amount = self.amount();
        let asset_id = self.asset_id();
        let tx_pointer = self.tx_pointer();

        let coin_hash = *Hasher::default()
            .chain(owner)
            .chain(amount.to_be_bytes())
            .chain(asset_id)
            .chain(tx_pointer.block_height().to_be_bytes())
            .chain(tx_pointer.tx_index().to_be_bytes())
            .finalize();

        Ok(coin_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fuel_core_types::{
        fuel_types::Address,
        fuel_vm::SecretKey,
    };

    #[test]
    fn test_generate_unique_utxo_id() {
        let mut generator = CoinConfigGenerator::new();
        let config1 = generator.generate();
        let config2 = generator.generate();

        assert_ne!(config1.utxo_id(), config2.utxo_id());
    }

    #[test]
    fn test_generate_with_owner_and_amount() {
        let mut rng = rand::thread_rng();
        let secret = SecretKey::random(&mut rng);
        let amount = 1000;

        let mut generator = CoinConfigGenerator::new();
        let config = generator.generate_with(secret, amount);

        assert_eq!(config.owner, Address::from(*secret.public_key().hash()));
        assert_eq!(config.amount, amount);
    }
}
