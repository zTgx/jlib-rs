

pub trait BaseX {
    fn encodex(&self);
    fn decodex(&self);
}

pub trait CodeC {
    fn encodec(&self);
    fn decodec(&self);

    fn is_valide(&self) -> bool;
    //fn is_valid_address() -> bool;
    //fn is_valid_secret() -> bool;
    //...
}