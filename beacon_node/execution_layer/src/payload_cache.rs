use lru::LruCache;
use parking_lot::Mutex;
use std::sync::Arc;
use tree_hash::TreeHash;
use types::{
    BeaconBlock, BlindedPayload, EthSpec, ExecutionPayload, FullPayload, Hash256, SignedBeaconBlock,
};

pub const DEFAULT_PAYLOAD_CACHE_SIZE: usize = 10;

/// A cache mapping execution payloads by transaction roots.
pub struct PayloadCache<T: EthSpec> {
    payloads: Mutex<LruCache<PayloadCacheId, ExecutionPayload<T>>>,
}

#[derive(Hash, PartialEq, std::cmp::Eq)]
struct PayloadCacheId(Hash256);

impl<T: EthSpec> Default for PayloadCache<T> {
    fn default() -> Self {
        PayloadCache {
            payloads: Mutex::new(LruCache::new(DEFAULT_PAYLOAD_CACHE_SIZE)),
        }
    }
}

impl<T: EthSpec> PayloadCache<T> {
    pub fn put(&self, payload: ExecutionPayload<T>) -> Option<ExecutionPayload<T>> {
        let tx_root = payload.transactions.tree_hash_root();
        self.payloads.lock().put(PayloadCacheId(tx_root), payload)
    }

    pub fn pop(&self, tx_root: &Hash256) -> Option<ExecutionPayload<T>> {
        self.payloads.lock().pop(&PayloadCacheId(*tx_root))
    }
}
