// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use codec::Codec;
use common::{BalanceWrapper, InvokeRPCError};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay, MaybeFromStr};
use std::sync::Arc;

// Custom imports
pub use liquidity_proxy_runtime_api::LiquidityProxyAPI as LiquidityProxyRuntimeAPI;
use liquidity_proxy_runtime_api::SwapOutcomeInfo;

#[rpc]
pub trait LiquidityProxyAPI<
    BlockHash,
    DEXId,
    AssetId,
    Balance,
    SwapVariant,
    LiquiditySourceType,
    FilterMode,
    OutputTy,
>
{
    #[rpc(name = "liquidityProxy_quote")]
    fn quote(
        &self,
        dex_id: DEXId,
        input_asset_id: AssetId,
        output_asset_id: AssetId,
        amount: BalanceWrapper,
        swap_variant: SwapVariant,
        selected_source_types: Vec<LiquiditySourceType>,
        filter_mode: FilterMode,
        at: Option<BlockHash>,
    ) -> Result<OutputTy>;

    #[rpc(name = "liquidityProxy_isPathAvailable")]
    fn is_path_available(
        &self,
        dex_id: DEXId,
        input_asset_id: AssetId,
        output_asset_id: AssetId,
        at: Option<BlockHash>,
    ) -> Result<bool>;
}

pub struct LiquidityProxyClient<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> LiquidityProxyClient<C, B> {
    /// Construct default `Template`.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, DEXId, AssetId, Balance, SwapVariant, LiquiditySourceType, FilterMode>
    LiquidityProxyAPI<
        <Block as BlockT>::Hash,
        DEXId,
        AssetId,
        Balance,
        SwapVariant,
        LiquiditySourceType,
        FilterMode,
        Option<SwapOutcomeInfo<Balance>>,
    > for LiquidityProxyClient<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: LiquidityProxyRuntimeAPI<
        Block,
        DEXId,
        AssetId,
        Balance,
        SwapVariant,
        LiquiditySourceType,
        FilterMode,
    >,
    DEXId: Codec,
    AssetId: Codec,
    Balance: Codec + MaybeFromStr + MaybeDisplay,
    SwapVariant: Codec,
    LiquiditySourceType: Codec,
    FilterMode: Codec,
{
    fn quote(
        &self,
        dex_id: DEXId,
        input_asset_id: AssetId,
        output_asset_id: AssetId,
        amount: BalanceWrapper,
        swap_variant: SwapVariant,
        selected_source_types: Vec<LiquiditySourceType>,
        filter_mode: FilterMode,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<SwapOutcomeInfo<Balance>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or(
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash,
        ));
        api.quote(
            &at,
            dex_id,
            input_asset_id,
            output_asset_id,
            amount,
            swap_variant,
            selected_source_types,
            filter_mode,
        )
        .map_err(|e| RpcError {
            code: ErrorCode::ServerError(InvokeRPCError::RuntimeError.into()),
            message: "Unable to quote price.".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn is_path_available(
        &self,
        dex_id: DEXId,
        input_asset_id: AssetId,
        output_asset_id: AssetId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or(
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash,
        ));
        api.is_path_available(&at, dex_id, input_asset_id, output_asset_id)
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(InvokeRPCError::RuntimeError.into()),
                message: "Unable to query path availability.".into(),
                data: Some(format!("{:?}", e).into()),
            })
    }
}
