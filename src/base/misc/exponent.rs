static TOKEN_E      : &'static str = "e";
static TOKEN_ZERO   : &'static str = "0";
static TOKEN_POINT  : &'static str = ".";

pub fn to_expo(s: &str) -> Result<String, &'static str> {
    let len = s.len();
    if len == 1 && s == "." {
        return Err("invalid input!");
    }

    for c in s.chars() {
        if c >= '0' && c <= '9' || c == '.' {

        } else {
            return Err("invalid input!");
        }
    }

    let mut idx: isize = - 1;
    if let Some(x) = s.find(".") {
        idx = x as isize;
    }

    match get_format(idx, len) {
        FormatType::AtHead => {
            if len > 2 {
                let ret = s.chars().nth(1).unwrap().to_string() + TOKEN_POINT + s.get(2..).unwrap() + TOKEN_E + "-1".to_string().as_str();
                Ok(ret)
            } else {
                let ret = s.chars().nth(1).unwrap().to_string() + TOKEN_POINT + TOKEN_ZERO + TOKEN_E + "-1".to_string().as_str();
                Ok(ret)
            }
        },

        FormatType::AtRear => {
            let left_len = len - 1;
            let move_steps = left_len - 1;
            let ret = s.chars().nth(0).unwrap().to_string() + TOKEN_POINT + s.get(1..left_len).unwrap() + TOKEN_E + move_steps.to_string().as_str();
            Ok(ret)
        },

        FormatType::AtMid => {
            let idx = idx as usize;
            let left = s.get(0..idx).unwrap();
            let right = s.get(idx+1..).unwrap();
            let left_len = left.len();
            let right_len = right.len();
            if s.chars().nth(0).unwrap() != '0' {
                let ret = s.chars().nth(0).unwrap().to_string() + TOKEN_POINT + left.get(1..).unwrap() + right + TOKEN_E + (left_len - 1).to_string().as_str();
                return Ok(ret);
            }

            let mut right_index = 0;
            while right_index < right_len {
                if right.chars().nth(right_index).unwrap() != '0' {
                    break;
                }

                right_index += 1;
            }

            if right_index == right_len - 1 {
                let ret = right.chars().nth(right_index).unwrap().to_string() + TOKEN_POINT + "0" + TOKEN_E + (-1 - right_index as isize).to_string().as_str();
                Ok(ret)
            } else {
                let ret = right.chars().nth(right_index).unwrap().to_string() + TOKEN_POINT + right.get(right_index+1..).unwrap() + TOKEN_E + (-1 - right_index as isize).to_string().as_str();
                Ok(ret)
            }
        }

        FormatType::AtNone => {
            let ret = s.chars().nth(0).unwrap().to_string() + TOKEN_POINT + s.get(1..).unwrap() + TOKEN_E + (len-1).to_string().as_str();
            Ok(ret)
        }
    }
}

pub fn get_exponent(s: &str) -> isize {
    let mut ret = 0isize;
    if let Some(x) = s.find("e") {
        if let Some(y) = s.get(x+1..) {
            if let Ok(z) = y.parse::<isize>() {
                ret = z;
            }
        }
    }

    ret
}

pub enum FormatType {
    //.开头格式 .11
    AtHead,

    //.在中间的格式 11.22
    AtMid,

    //.结尾的格式  11.
    AtRear,

    //没有.的格式 22
    AtNone,
}

pub fn get_format(idx: isize, len: usize) -> FormatType {
    if idx == 0 {
        return FormatType::AtHead;
    }

    if idx == (len - 1) as isize {
        return FormatType::AtRear;
    }

    if idx == -1 {
        return FormatType::AtNone;
    }

    return FormatType::AtMid;
}