use crate::error::Error;
use crate::message::{InstantiateMsg, ProcMsg, QueryMsg, QueryRsp};
use kelk::blockchain::address::Address;
use kelk::context::Context;
use kelk::kelk_derive;
use kelk::storage::bst::StorageBST;

fn transfer(ctx: Context, to: Address, amount: i64) -> Result<(), Error> {
    let from: Address = ctx.storage.read_struct(0).unwrap();
    transfer_from(ctx, from, to, amount)
}

fn name(ctx: Context) -> Result<String, Error> {
    Ok(ctx.storage.read_string(22, 64).unwrap())
}

fn symbol(ctx: Context) -> Result<String, Error> {
    Ok(ctx.storage.read_string(86, 4).unwrap())
}

fn total_supply(ctx: Context) -> Result<i64, Error> {
    Ok(ctx.storage.read_i64(90).unwrap())
}

fn balance(ctx: Context, addr: Address) -> Result<i64, Error> {
    let bst: StorageBST<Address, i64> = StorageBST::lazy_load(ctx.storage, 128).unwrap();
    let balance = bst.find(&addr).unwrap().unwrap_or(0);
    Ok(balance)
}

fn transfer_from(ctx: Context, from: Address, to: Address, amount: i64) -> Result<(), Error> {
    let mut bst: StorageBST<Address, i64> = StorageBST::lazy_load(ctx.storage, 128).unwrap(); // FIXME: no unwrap
    let tx_balance = bst.find(&from).unwrap().unwrap_or(0);
    let rx_balance = bst.find(&to).unwrap().unwrap_or(0);

    if tx_balance < amount {
        return Err(Error::InsufficientAmount);
    }

    bst.insert(from, tx_balance - amount).unwrap();
    bst.insert(to, rx_balance + amount).unwrap();

    Ok(())
}
/*
instantiate creates a new contract and deployment code.
*/
#[kelk_derive(instantiate)]
pub fn instantiate(ctx: Context, msg: InstantiateMsg) -> Result<(), Error> {
    if msg.name.len() > 64 {
        return Err(Error::InvalidMsg);
    }
    if msg.symbol.len() > 4 {
        return Err(Error::InvalidMsg);
    }
    ctx.storage.write_struct(0, &msg.owner).unwrap();
    ctx.storage.write_string(22, &msg.name, 64).unwrap();
    ctx.storage.write_string(86, &msg.symbol, 4).unwrap();
    ctx.storage.write_i64(90, msg.total_supply).unwrap();
    let mut bst: StorageBST<Address, i64> = StorageBST::create(ctx.storage, 128, 1000).unwrap();
    // FIXME unwrap()
    bst.insert(msg.owner, msg.total_supply).unwrap();
    Ok(())
}

/*
process executes the contract associated with the addr with the given input as
parameters. It also handles any necessary value transfer required and takes
the necessary steps to create accounts and reverses the state in case of an
execution error or failed value transfer.
*/
#[kelk_derive(process)]
pub fn process(ctx: Context, msg: ProcMsg) -> Result<(), Error> {
    match msg {
        ProcMsg::Transfer { to, amount } => transfer(ctx, to, amount),
        ProcMsg::TransferFrom { from, to, amount } => transfer_from(ctx, from, to, amount),
    }
}

/*
query executes the contract associated with the addr with the given input
as parameters while disallowing any modifications to the state during the call.
*/
#[kelk_derive(query)]
pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, Error> {
    let res = match msg {
        QueryMsg::Name => QueryRsp::Name { res: name(ctx)? },
        QueryMsg::Symbol => QueryRsp::Symbol { res: symbol(ctx)? },
        QueryMsg::TotalSupply => QueryRsp::TotalSupply {
            res: total_supply(ctx)?,
        },
        QueryMsg::Balance { addr } => QueryRsp::Balance {
            res: balance(ctx, addr)?,
        },
    };

    Ok(res)
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod tests;
