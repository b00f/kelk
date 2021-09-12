use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum CalcMsg {
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
