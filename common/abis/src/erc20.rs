use common_exceptions::Result;
use web3::contract::{Contract, Options};
use web3::futures::future::err;
use web3::transports::{Batch, Http};
use web3::types::{Address, U256, U64};
use web3::Web3;


pub struct Erc20Abi {
    address: Address,
    abi: Vec<u8>,
}

impl Erc20Abi {
    pub fn new(_address: Address) -> Erc20Abi {
        let erc20_abi = include_bytes!("erc20.abi");
        Self {
            address: _address,
            abi: erc20_abi.into_vec(),
        }
    }

    pub fn is_valid_contract(&self) -> Result<bool>{
        if self.abi.len() <= 0 {
            Err("contract abi not init")
        }

        if self.address.is_zero() {
            Err("contract address not init")
        }

        Ok(true)
    }

    pub fn get_abi(&self) -> Result<Vec<u8>>{
        Ok(self.abi.into_vec())
    }

    pub fn get_addr(&self) -> Result<Address>{
        Ok(self.address)
    }


    pub fn total_supply(&self, web3: Web3<Batch<Http>>, block: U64) -> Result<U256> {
        let token_contract = Contract::from_json(web3.eth(), self.address, self.abi.as_slice())?;
        let res = token_contract.query("totalSupply", (),
                                       None, Options::default(), block)?;
        Ok(res)
    }

    pub fn balance_of(&self, web3: Web3<Batch<Http>>, account: Address,block: U64) -> Result<U256> {
        let token_contract = Contract::from_json(web3.eth(), self.address, self.abi.as_slice())?;
        let res = token_contract.query("balanceOf", account,
                                       None, Options::default(), block)?;
        Ok(res)
    }
}
