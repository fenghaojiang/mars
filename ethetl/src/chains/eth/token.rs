// Copyright 2022 BohuTANG.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use std::vec;
use web3::contract::Contract;

use common_exceptions::Result;
use common_exceptions::Retryable;
use web3::types::Address;
use web3::types::Bytes;
use web3::types::{U64,U256};
use web3::types::CallRequest;

use crate::contexts::ContextRef;


pub struct TokenRequest {
    address:Address,
    block_number: U64,
}


pub struct TokenFetcher {
    ctx: ContextRef,
    address_requests: Vec<TokenRequest>,
}


pub struct TokenInfo {
    pub token_address: Address,
    pub total_supply: U256,
    pub block_number: U64,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
}

impl TokenFetcher{
    pub fn create(ctx: &ContextRef) -> TokenFetcher {
        Self {
            ctx: ctx.clone(),
            address_requests: vec![],
        }
    }

    pub fn push(&mut self, req: TokenRequest) -> Result<()> {
        self.address_requests.push(req);
        Ok(())
    }

    pub fn push_batch(&mut self, reqs: Vec<TokenRequest>) -> Result<()> {
        self.address_requests.extend(reqs);
        Ok(())
    }


    pub async fn fetch(&self) -> Result<Vec<Bytes>> {
        let notify = |e, duration| {
            log::warn!(
                "Fetch token info error at duration {:?}, error:{:?}",
                duration,
                e
            )
        };
        let op = || async {
            let res = self.fetch_with_no_retry().await?;
            Ok(res)
        };

        op.retry_with_notify(notify).await
    }



    async fn fetch_with_no_retry(&self) -> Result<Vec<Bytes>> {
        let http = web3::transports::Http::new(self.ctx.get_rpc_url())?;
        let web3 = web3::Web3::new(web3::transports::Batch::new(http));


        let mut tokens = vec![];

        for chunks in self.address_requests.chunks(self.ctx.get_web3_batch_size()) {

            let mut callbacks = vec![];


            for token_req in chunks {

                let erc20_contrac = Contract::from_json(web3.eth())

            }
        }
        Ok(vec![])
    }

}