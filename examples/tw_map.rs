fn main() {
    use mylib::base::gTransactionTypeMap;
    use mylib::base::TWHashMap;
    let key_from_value = gTransactionTypeMap.get_key_from_value(100);
    println!("key from global map : {:?}", key_from_value);
}