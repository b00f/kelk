use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum ProcMsg {
    #[n(0)]
    Add {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
    #[n(1)]
    Sub {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
    #[n(2)]
    Mul {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
    #[n(3)]
    Div {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryMsg {
    #[n(0)]
    LastResult,
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryRsp {
    #[n(0)]
    Result {
        #[n(0)]
        res: i32,
    },
}
