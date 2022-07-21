use kelk::blockchain::address::Address;
use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum ProcMsg {
    #[n(0)]
    TransferFrom {
        #[n(0)]
        from: Address,
        #[n(1)]
        to: Address,
        #[n(2)]
        amount: i64,
    },
    #[n(1)]
    Transfer {
        #[n(0)]
        to: Address,
        #[n(1)]
        amount: i64,
    },
}
#[derive(Clone, Debug, Encode, Decode)]
pub struct InstantiateMsg {
    #[n(0)]
    pub name: String,
    #[n(1)]
    pub symbol: String,
    #[n(2)]
    pub total_supply: i64,
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryMsg {
    #[n(0)]
    Name,
    #[n(1)]
    Symbol,
    #[n(2)]
    Balance {
        #[n(0)]
        addr: Address,
    },
    #[n(3)]
    TotalSupply,
    #[n(4)]
    Allowance {
        #[n(0)]
        owner: Address,
        #[n(1)]
        spender: Address,
    },
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryRsp {
    #[n(0)]
    Name {
        #[n(0)]
        res: String,
    },
    #[n(1)]
    Symbol {
        #[n(1)]
        res: String,
    },
    #[n(2)]
    Balance {
        #[n(2)]
        res: i64,
    },
    #[n(3)]
    TotalSupply {
        #[n(2)]
        res: i64,
    },
    #[n(4)]
    Allowance {
        #[n(0)]
        res: i64,
    },
}
