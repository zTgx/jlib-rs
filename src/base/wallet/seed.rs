
//Seed
#[derive(Debug, Clone)]
pub struct Seed {
    pub seed_property: SeedProperty,
}
impl Seed {
    pub fn new(seed_property: SeedProperty) -> Self {
        Seed {
            seed_property: seed_property,
        }
    }
}

// impl BaseX for Seed {
//     fn encodex(&self) {

//     }
//     fn decodex(&self) {

//     }
// }

// impl CodeC for Seed {
//     fn encodec(&self) {

//     }
//     fn decodec(&self) {

//     }

//     fn is_valide(&self) -> bool {

//     }
// }



#[derive(Debug)]
pub struct SeedProperty {
    pub seed: String, //hex string
    pub length: u32,
    //pub bytes: [u8],
    //pub hex_upper_case: String,
}                                                                                                                                                                                                                                      
impl SeedProperty {
    pub fn new(seed: &str, length: u32) -> Self {
        SeedProperty {
            seed: String::from(seed),
            length: length,
        }
    }
}
impl Clone for SeedProperty {
    fn clone(&self) -> SeedProperty {
        match self {
            _ =>
            SeedProperty {
                seed: self.seed.to_owned(),
              
                length: self.length,   
            },
        }
    }
}

//复杂seed的创建，销毁，混淆，编码等各种操作
pub struct SeedBuilder {
    pub seed_property: SeedProperty,
}
impl SeedBuilder {
    pub fn new(seed_property: SeedProperty) -> Self {
        SeedBuilder {
            seed_property: seed_property,
        }
    }

    pub fn build(&self) -> Seed {
        Seed {
            seed_property: self.seed_property.clone(),
        }
    }

    //compress

    //to_hex

    //to_bytes

    //...
}