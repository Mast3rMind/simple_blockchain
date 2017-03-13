
use chrono::{DateTime, UTC};
use crypto::sha2::Sha256;
use crypto::digest::Digest;


use BlockChain;

pub struct Block{
    index: u64,
    timestamp: DateTime<UTC>,
    data: [u8;256],
    hash: String,
    previous_hash: String

}

impl PartialEq for Block{
    fn eq(&self, other: &Block) -> bool{
        if self.data.len() != other.data.len(){
            return false
        }
        for i in 0..self.data.len(){
            if self.data[i] != other.data[i]{
                return false;
            }
        }
        self.index == other.index &&
            self.timestamp == other.timestamp &&
            self.hash == other.hash &&
            self.previous_hash == other.previous_hash
    }
}

impl Clone for Block{
    fn clone(&self) -> Self{
        let mut new_data = [0;256];
        new_data.copy_from_slice(&self.data);
        Block{
            index: self.index,
            timestamp: self.timestamp,
            data: new_data,
            hash: self.hash.clone(),
            previous_hash: self.previous_hash.clone()
        }
    }
}

impl Block{
    pub fn new(block_chain: &BlockChain, block_data: [u8;256]) -> Block{
        let previous = block_chain.retrieve_latest_block();
        let index = previous.index + 1;
        let timestamp = UTC::now();

        let hash = compute_hash(index, &timestamp, &block_data, &previous.hash);
        Block{
            index: index,
            timestamp: timestamp,
            data: block_data,
            hash: hash,
            previous_hash: previous.hash.clone()
        }
    }


    pub fn has_valid_hash(&self) -> bool{
        let hash = compute_hash(self.index, &self.timestamp, &self.data, &self.previous_hash);
        hash == self.hash
    }

    pub fn is_valid_successor_of(&self, previous_block: &Block) -> bool {
        previous_block.index +1 == self.index &&
        previous_block.hash == self.previous_hash &&
        self.has_valid_hash()
    }
}

pub fn genesis_block() -> Block{
    let timestamp = UTC::now();
    let data = [0;256];
    let hash = compute_hash(0, &timestamp, &data, "0");

    Block{
        index: 0,
        timestamp: timestamp,
        data: data,
        hash: hash,
        previous_hash: "0".to_string()
    }
}

fn compute_hash(index: u64, timestamp: &DateTime<UTC>, block_data: &[u8;256], previous_hash: &str) -> String{
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}|{}|", index, timestamp));
    hasher.input(block_data);
    hasher.input_str(&format!("|{}", previous_hash));

    hasher.result_str()
}





#[cfg(test)]
mod tests {
    use super::*;
    use BlockChain;

    #[test]
    fn new_blocks_are_valid() {
        let blockchain = BlockChain::default();
        let genesis = blockchain.retrieve_latest_block();
        let new_block = Block::new(&blockchain, [2;256]);
        assert!(new_block.is_valid_successor_of(&genesis))
    }
    #[test]
    fn other_blocks_are_not_valid() {
        let mut blockchain = BlockChain::default();
        let genesis = blockchain.retrieve_latest_block().clone();
        let new_block = Block::new(&blockchain, [2;256]);
        assert!(blockchain.push_block(new_block).is_ok());
        let new_block = Block::new(&blockchain, [2;256]);
        assert!(!new_block.is_valid_successor_of(&genesis))
    }


    #[test]
    fn block_is_equal_to_itself() {
        let blockchain = BlockChain::default();
        let block = Block::new(&blockchain, [3;256]);

        assert!(block == block);
    }

    #[test]
    fn different_blocks_are_not_equal() {
        let blockchain = BlockChain::default();
        let block = Block::new(&blockchain, [3;256]);
        let block2 = Block::new(&blockchain, [3;256]);

        // These two blocks will have a different hash and timestamp
        assert!(block != block2);

        // Check for different data
        let block = Block::new(&blockchain, [3;256]);
        let mut block2 = block.clone();
        block2.data = [1;256];

        assert!(block != block2);
    }

}
