use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CalcMsg {
    Add {
        a: i32,
        b: i32,
    },
    Sub {
        a: i32,
        b: i32,
    },
    Mul {
        a: i32,
        b: i32,
    },
    Div {
        a: i32,
        b: i32,
    },
}
