
pub const NODE_PUBLIC  : usize = 28;
pub const NODE_PRIVATE : usize = 32;
pub const ACCOUNT_ID   : usize = 0;
pub const FAMILY_SEED  : usize = 33;
pub const ED25519_SEED : [u8; 3]   = [0x01, 0xe1, 0x4b];
pub const SIGN_TYPE    : [&'static str; 2] = ["ed25519", "secp256k1"]; 

#[derive(Debug)]
pub struct EdSeed {
    pub expected_length: usize,
    pub version: [u8; 3],
}
impl Default for EdSeed {
    fn default() -> Self {
        EdSeed { 
            expected_length: 16,
            version: ED25519_SEED,
        }
    }
}

#[derive(Debug)]
pub struct Seed {
    pub version_types: [&'static str; 2],
    pub version: usize,
    pub expected_length: usize,
}
impl Default for Seed {
    fn default() -> Self {
        Seed {
            version_types: SIGN_TYPE,
            version: FAMILY_SEED,
            expected_length: 16,
        }
    }
}

#[derive(Debug)]
pub struct AccountID { 
    pub version: usize,
    pub expected_length: usize, 
}
impl Default for AccountID {
    fn default() -> Self {
        AccountID {
            version: ACCOUNT_ID,
            expected_length: 20,
        }
    }
}

#[derive(Debug)]
pub struct Address { 
    pub version: usize, 
    pub expected_length: usize, 
}
impl Default for Address {
    fn default() -> Self {
        Address {
            version: ACCOUNT_ID,
            expected_length: 20,
        }
    }
}

#[derive(Debug)]
pub struct NodePublic { 
    pub version: usize, 
    pub expected_length: usize 
}
impl Default for NodePublic {
    fn default() -> Self {
        NodePublic {
            version: NODE_PUBLIC,
            expected_length: 33,
        }
    }
}

#[derive(Debug)]
pub struct NodePrivate { 
    pub version: usize, 
    pub expected_length: usize,
}
impl Default for NodePrivate {
    fn default() -> Self {
        NodePrivate {
            version: NODE_PRIVATE,
            expected_length: 32,
        }
    }
}

#[derive(Debug)]
pub struct K256Seed { 
    pub version: usize, 
    pub expected_length: usize, 
}
impl Default for K256Seed {
    fn default() -> Self {
        K256Seed {
            version: FAMILY_SEED,
            expected_length: 16,
        }
    }
}

#[derive(Debug)]
pub enum BASEStates {
    EdSeed(EdSeed),
    Seed(Seed),
    AccountID(AccountID),
    Address(Address),
    NodePublic(NodePublic),
    NodePrivate(NodePrivate),
    K256Seed(K256Seed),
}

//USE, reference commoms/Flags/get method.
//A::default()