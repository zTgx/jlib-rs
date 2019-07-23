
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Memos {
    #[serde(rename="Memos")]
    pub memo: Memo,
}
impl Memos {
    pub fn new(memo: Memo) -> Self {
        Memos {
            memo: memo,
        }
    }
}

pub struct MemosBuilder {
    pub value: String,
}
impl MemosBuilder {
    pub fn new(value: String) -> Self {
        MemosBuilder {
            value: value,
        }
    }

    pub fn build(&self) -> Memos {
        let data = MemoData::new( String::from( self.value.as_str() ) );
        let memo = Memo::new(data);

        Memos {
            memo: memo,
        }
    }
}