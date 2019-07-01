use crate::base::inverse_fields_map::INVERSE_FIELDS_MAP;
use crate::base::serialized_type::*;    

//TypeObjBulder usage.
//TypeObj: 通过TxJson中任意字段来计算在序列化过程中的【坐标】
//let type_obj = TypeObjBuilder::new("key").build();
//println!("type_obj: {:?}", type_obj);

//header 
pub trait SerializeHeader {
    fn serialize_header(&self, so: &mut Vec<u8>);
}

#[derive(Debug)]
pub struct TypeObj {
    pub type_tag:   u8,
    pub type_bits:  u8                                                                                      ,
    pub field_bits: u8,
}
impl TypeObj {
    pub fn new(type_tag: u8, type_bits: u8, field_bits: u8) -> Self {
        TypeObj {
            type_tag: type_tag,
            type_bits: type_bits,
            field_bits: field_bits,
        }
    }
}
impl SerializeHeader for TypeObj {
    fn serialize_header(&self, so: &mut Vec<u8>) {
        println!("tag: {} / type: {} / field: {}", self.type_tag, self.type_bits, self.field_bits);
        let mut s8 = STInt8::serialize(self.type_tag);
        so.append(&mut s8);

        if (self.type_bits >= 16) {
            let mut s = STInt8::serialize(self.type_bits);
            so.append(&mut s);
        }

        if (self.field_bits >= 16) {
            let mut x = STInt8::serialize(self.field_bits);
            so.append(&mut x);
        }
    }
}



pub struct TypeObjBuilder {
    pub key: &'static str,

    //inner status
    type_bits: Option<u8>,
    field_bits: Option<u8>,
    type_tag: Option<u8>,
}
impl TypeObjBuilder {
    pub fn new(key: &'static str) -> Self {
        TypeObjBuilder {
            key: key,

            type_bits: None,
            field_bits: None,
            type_tag: None,
        }
    }

    pub fn build(&mut self) -> Option<TypeObj> {
        //type_bits
        self.calc_type_bits();

        //field_bits
        self.calc_field_bits();

        //type_tag
        self.calc_type_tag();

        //New TypeObj
        Some(TypeObj::new(self.type_tag.unwrap(), self.type_bits.unwrap(), self.field_bits.unwrap()))
    }

    //Inner methods
    fn calc_type_bits(&mut self) {
        let field_coordinates = INVERSE_FIELDS_MAP.get(self.key).unwrap();
        self.type_bits  = Some(field_coordinates[0]);
    }

    fn calc_field_bits(&mut self) {
        let field_coordinates = INVERSE_FIELDS_MAP.get(self.key).unwrap();
        self.field_bits = Some(field_coordinates[1]);
    }

    fn calc_type_tag(&mut self) {
        let left = if self.type_bits.unwrap() < 16 { self.type_bits.unwrap() << 4 } else { 0 };
        let right = if self.field_bits.unwrap() < 16 { self.field_bits.unwrap() } else { 0 };
        self.type_tag = Some(left | right);
    }
}
