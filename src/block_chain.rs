use block::*;

pub struct BlockChain{
    block_chain: Vec<Block>
}

impl Default for BlockChain{
    fn default() -> Self {
        BlockChain{
            block_chain: vec![genesis_block()]
        }
    }
}

impl BlockChain{

    pub fn retrieve_latest_block(&self) -> &Block{
        self.block_chain.iter().last().unwrap()
    }


    pub fn push_block(&mut self, block: Block) -> Result<(), Block>{
        if block.is_valid_successor_of(self.retrieve_latest_block()){
            self.block_chain.push(block);
            Ok(())
        }else{
            Err(block)
        }
    }

    pub fn is_valid_chain(&self) -> bool{
        self.block_chain.windows(2).all(|blocks| blocks[1].is_valid_successor_of(&blocks[0]))
    }
    pub fn length(&self) -> usize{
        self.block_chain.len()
    }

    pub fn integrate_chain(&mut self, blockchain: BlockChain) -> Result<(), BlockChain>  {
        if blockchain.is_valid_chain() && blockchain.length() > self.length(){
            self.block_chain = blockchain.block_chain;
            Ok(())
        } else {
            Err(blockchain)
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adding_new_blocks_generates_a_valid_blockchain() {
        let mut blockchain = BlockChain::default();
        let new_block = Block::new(&blockchain, [2;256]);
        assert!(blockchain.push_block(new_block).is_ok());
        let new_block = Block::new(&blockchain, [2;256]);
        assert!(blockchain.push_block(new_block).is_ok());
        let new_block = Block::new(&blockchain, [2;256]);
        assert!(blockchain.push_block(new_block).is_ok());

        assert!(blockchain.is_valid_chain());
    }

    #[test]
    fn invalid_blockchain_is_detected(){
        let mut blockchain = BlockChain::default();
        let new_block = Block::new(&blockchain, [2;256]);
        assert!(blockchain.push_block(new_block.clone()).is_ok());

        blockchain.block_chain.push(new_block);
        assert!(!blockchain.is_valid_chain());
    }


    #[test]
    fn test_successfull_chain_integration() {
        let mut default_chain = BlockChain::default();
        let mut chain = BlockChain::default();
        let block = Block::new(&chain, [2;256]);
        assert!(chain.push_block(block).is_ok());
        let chain_length = chain.length();
        let chain_last_block = chain.retrieve_latest_block().clone();
        assert!(default_chain.integrate_chain(chain).is_ok());
        assert_eq!(default_chain.length(), chain_length);
        assert!(*default_chain.retrieve_latest_block() == chain_last_block);
    }

    #[test]
    fn test_unsuccessfull_chain_integration() {
        let mut default_chain = BlockChain::default();
        let mut chain = BlockChain::default();
        let block = Block::new(&chain, [2;256]);
        assert!(chain.push_block(block).is_ok());

        let block = Block::new(&default_chain, [2;256]);
        assert!(default_chain.push_block(block).is_ok());

        let chain_last_block = chain.retrieve_latest_block().clone();
        assert!(default_chain.integrate_chain(chain).is_err());
        assert!(*default_chain.retrieve_latest_block() != chain_last_block);
    }
}
