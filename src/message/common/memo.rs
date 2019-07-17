
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MemoData {
    #[serde(rename="MemoData")]
    pub memo_data: String,
}
impl MemoData {
    pub fn new(memo_data: String) -> Self {
        MemoData {
            memo_data: memo_data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Memo {
    #[serde(rename="Memo")]
    pub memo_data: MemoData,
}

impl Memo {
    pub fn new(memo_data: MemoData) -> Self {
        Memo {
            memo_data: memo_data,
        }
    }
}