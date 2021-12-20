//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use carpooling_runtime_api::CarpoolingApi as CarpoolingRuntimeApi;

#[rpc]
pub trait CarpoolingApi<BlockHash> {
	#[rpc(name = "carpooling_getDriver")]
	fn get_driver(&self, at: Option<BlockHash>) -> Result<u32>;
}

/// A struct that implements the `CarpoolingApi`.
pub struct Driver<C, M> {
	// If you have more generics, no need to SumStorage<C, M, N, P, ...>
	// just use a tuple like SumStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> Driver<C, M> {
	/// Create new `Driver` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}


impl<C, Block> CarpoolingApi<<Block as BlockT>::Hash> for Driver<C, Block>
	where
		Block: BlockT,
		C: Send + Sync + 'static,
		C: ProvideRuntimeApi<Block>,
		C: HeaderBackend<Block>,
		C::Api: CarpoolingRuntimeApi<Block>,
{
	fn get_driver(&self, at: Option<<Block as BlockT>::Hash>) -> Result<u32> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_driver(&at,12);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876),
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
