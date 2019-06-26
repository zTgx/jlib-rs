use typename::TypeName;

fn main() {
    let stint8 = STInt8::new(18u8);
    let type_name = stint8.type_name_of();
    println!("typename : {}", type_name);
    let x = stint8.serialize();
    println!("x : {:?}", x);
    println!("x.typename : {}", x.type_name_of());
}