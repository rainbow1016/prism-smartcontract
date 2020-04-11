use crate::block::{Block, Content};
use crate::blockchain::BlockChain;
use crate::blockdb::BlockDatabase;
use crate::crypto::hash::Hashable;
use crate::transaction::Transaction;
use crate::experiment::performance_counter::PERFORMANCE_COUNTER;
use crate::miner::memory_pool::MemoryPool;

use crate::network::server::Handle as ServerHandle;

use std::sync::Mutex;

pub fn new_validated_block(
    block: &Block,
    mempool: &Mutex<MemoryPool>,
    _blockdb: &BlockDatabase,
    chain: &BlockChain,
    _server: &ServerHandle,
) {
    PERFORMANCE_COUNTER.record_process_block(&block);

    // if this block is a transaction, remove transactions from mempool
    if let Content::Transaction(content) = &block.content {
        let mut mempool = mempool.lock().unwrap();
        for tx in &content.transactions {
            mempool.remove_by_hash(&<Transaction as Hashable>::hash(&tx));
        }
        drop(mempool);
    }

    // insert the new block into the blockchain
    chain.insert_block(&block).unwrap();
}
