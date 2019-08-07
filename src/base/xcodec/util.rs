pub fn seq_equal(left: &Vec<u8>, right: &Vec<u8>) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut idx = 0;
    while idx < left.len() {
        if left[idx] != right[idx] {
            return false;
        }

        idx += 1;
    }

    true
}

pub fn concat_args(so: &mut Vec<u8>, val: &Vec<u8>) {
    so.extend(val);
}

pub fn is_set<T>(val: &Option<T>) -> bool {
    val.is_some()
}
