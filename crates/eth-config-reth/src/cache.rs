use std::cell::RefCell;

use eth_config_api::EthConfigResponse;

/// Simple in-memory cache keyed by block number.
#[derive(Default)]
pub struct ForkConfigCache {
    entry: RefCell<Option<(u64, EthConfigResponse)>>,
}

impl ForkConfigCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Return cached value if valid for `block_number`; otherwise recompute with `resolver`.
    pub fn get<F>(&self, block_number: u64, resolver: F) -> EthConfigResponse
    where
        F: FnOnce(u64) -> EthConfigResponse,
    {
        let mut state = self.entry.borrow_mut();
        let needs_refresh = match state.as_ref() {
            Some((cached_block, _)) if *cached_block == block_number => false,
            _ => true,
        };

        if needs_refresh {
            let value = resolver(block_number);
            *state = Some((block_number, value.clone()));
            value
        } else {
            state.as_ref().unwrap().1.clone()
        }
    }

    /// Invalidate cached entry manually (e.g. on fork boundary detection).
    pub fn invalidate(&self) {
        *self.entry.borrow_mut() = None;
    }
}
