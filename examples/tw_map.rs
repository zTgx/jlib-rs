fn main() {
    use mylib::base::G_TRANSACTION_TYPE_MAP;
    use mylib::base::TWHashMap;
    let key_from_value = G_TRANSACTION_TYPE_MAP.get_key_from_value(100);
    println!("key from global map : {:?}", key_from_value);
}