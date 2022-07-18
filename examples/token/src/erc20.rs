use crate::error::Error;
use kelk::blockchain::address::Address;
use kelk::context::Context;
use kelk::storage::bst::StorageBST;

pub(crate) struct ERC20<'a> {
    // context to access to storage and blockchain APIs
    ctx: Context<'a>,

    /// Mapping from owner to number of owned token.
    _balances: StorageBST<'a, Address, i64>,

    /// Mapping of the token amount which an account is allowed to withdraw
    /// from another account.
    _allowances: StorageBST<'a, (Address, Address), i64>,
}

impl<'a> ERC20<'a> {
    pub fn instantiate(
        ctx: Context<'a>,
        owner: &Address,
        name: &str,
        symbol: &str,
        total_supply: &i64,
    ) -> Result<(), Error> {
        if name.len() > 64 {
            return Err(Error::InvalidMsg);
        }
        if symbol.len() > 4 {
            return Err(Error::InvalidMsg);
        }
        ctx.storage.write_struct(0, &owner.clone()).unwrap();
        ctx.storage.write_string(22, name, 64).unwrap();
        ctx.storage.write_string(86, symbol, 4).unwrap();
        ctx.storage.write_i64(90, total_supply).unwrap();
        let mut balances: StorageBST<'a, Address, i64> =
            StorageBST::create(ctx.storage, 128, 1000).unwrap();
        let _: StorageBST<'a, (Address, Address), i64> =
            StorageBST::create(ctx.storage, 12800, 1000).unwrap();

        balances.insert(owner.clone(), *total_supply).unwrap();

        Ok(())
    }

    pub fn lazy_load(ctx: Context<'a>) -> Result<Self, Error> {
        let balances = StorageBST::lazy_load(ctx.storage, 128).unwrap();
        let allowances = StorageBST::create(ctx.storage, 12800, 1000).unwrap();

        Ok(Self {
            ctx,
            _balances: balances,
            _allowances: allowances,
        })
    }

    pub fn transfer(&mut self, to: Address, amount: i64) -> Result<(), Error> {
        let from: Address = self.ctx.storage.read_struct(0).unwrap();
        self.transfer_from(from, to, amount)
    }

    pub fn name(&self) -> Result<String, Error> {
        Ok(self.ctx.storage.read_string(22, 64).unwrap())
    }

    pub fn symbol(&self) -> Result<String, Error> {
        Ok(self.ctx.storage.read_string(86, 4).unwrap())
    }

    pub fn total_supply(&self) -> Result<i64, Error> {
        Ok(self.ctx.storage.read_i64(90).unwrap())
    }

    pub fn balance(&self, addr: Address) -> Result<i64, Error> {
        let balance = self._balances.find(&addr).unwrap().unwrap_or(0);
        Ok(balance)
    }

    pub fn transfer_from(&mut self, from: Address, to: Address, amount: i64) -> Result<(), Error> {
        let tx_balance = self._balances.find(&from).unwrap().unwrap_or(0);
        let rx_balance = self._balances.find(&to).unwrap().unwrap_or(0);

        if tx_balance < amount {
            return Err(Error::InsufficientAmount);
        }

        self._balances.insert(from, tx_balance - amount).unwrap();
        self._balances.insert(to, rx_balance + amount).unwrap();

        Ok(())
    }
}

#[cfg(test)]
#[path = "./erc20_test.rs"]
mod tests;
