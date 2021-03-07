pub enum VersionEncoding {
    VerNone               = 1,
    VerAccountId          = 0,
    VerFamilySeed         = 33,
    VerAccountPublic      = 35,
}

// 生成国密版本seed需要的常量
pub const PASS_PHRASE_LENGTH: usize   = 16;
