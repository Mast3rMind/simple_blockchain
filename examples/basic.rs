extern crate simple_blockchain;

use simple_blockchain::*;

fn main(){
    let mut blockchain = BlockChain::default();

    let new_block = Block::new(&blockchain, [0;256]);
    if let Ok(()) = blockchain.push_block(new_block){
        println!("Added new block to the blockchain.");
    }else{
        println!("Could not add new block to the blockchain.");
    }
}
