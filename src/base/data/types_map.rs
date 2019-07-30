

//Mapping of type ids to data types. The type id is specified by the high

lazy_static! {
pub static ref TYPES_MAP: [&'static str; 20] = {
    let v: [&'static str; 20] = [
        "void(0)",

        // Common
        "Int16",    // 1
        "Int32",    // 2
        "Int64",    // 3
        "Hash128",  // 4
        "Hash256",  // 5
        "Amount",   // 6
        "VL",       // 7
        "Account",  // 8

        // 9-13 reserved
        "void(0)",    // 9
        "void(0)",    // 10
        "void(0)",    // 11
        "void(0)",    // 12
        "void(0)",    // 13

        "Object",   // 14
        "Array",    // 15

        // Uncommon
        "Int8",     // 16
        "Hash160",  // 17
        "PathSet",  // 18
        "Vector256" // 19
    ];

    v
};
}
