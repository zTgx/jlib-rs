
use crate::base::curve::sha256::JSha256;
use crate::base::xcodec::util::{is_set, concat_args, seq_equal};
use crate::base::data::base_data::{BaseDataI};
use basex_rs::BaseX;
use cast_rs::hexcast;

pub trait XCodeI {
    fn encode(bytes: &mut Vec<u8>, arg: Box<dyn BaseDataI>) -> String;
    fn decode(string: &String, opts: Box<dyn BaseDataI>) -> Option<Vec<u8>>;

    fn encode_raw(bytes: &mut Vec<u8>) -> String ;
    fn decode_raw(string: String) -> Option<Vec<u8>>;

    fn encode_checked(buffer: &mut Vec<u8>) -> String;
    fn decode_checked(encoded: String) -> Option<Vec<u8>>;

    fn encode_versioned(bytes: &mut Vec<u8>, arg: Box<dyn BaseDataI>) -> String;
    fn decode_versioned(string: String, arg: Box<dyn BaseDataI>) -> Option<Vec<u8>>;

    fn decode_multi_versioned(encoded: &String, arg: Box<dyn BaseDataI>) -> Option<Vec<u8>> ;
    fn verify_checksum(bytes: &Vec<u8>) -> bool;
    // fn find_prefix();
}

pub struct CodecFactory {}
impl CodecFactory {
    pub fn new() -> Self {
        // this.alphabet = alphabet;
        // this.codec = baseCodec(alphabet);
        // alphabet_len = alphabet.length;

        CodecFactory {
        }
    }
}

impl XCodeI for CodecFactory {
    fn encode(bytes: &mut Vec<u8>, arg: Box<dyn BaseDataI>) -> String {
        let version = arg.get_version();
        if is_set(&version) {
            return CodecFactory::encode_versioned(bytes, arg);
        } else {
            if arg.get_checked().is_some() {
                return CodecFactory::encode_checked(bytes);
            } else {
                return CodecFactory::encode_raw(bytes);
            }
        }
    }

    fn decode(string: &String, opts: Box<dyn BaseDataI>) -> Option<Vec<u8>> {
        let versions = opts.get_versions();
        if versions.is_some() {
            return CodecFactory::decode_multi_versioned(string, opts);//opts.get_expected_length(), opts.get_version_type());
        } else {
            let version = opts.get_version();
            if version.is_some() {
                return CodecFactory::decode_versioned(string.to_string(), opts);
            } else {
                let checked = opts.get_checked();
                if checked.is_some() {
                    return CodecFactory::decode_checked(string.to_string());
                } else {
                    return CodecFactory::decode_raw(string.to_string());
                }
            }
        }
    }

    fn encode_raw(bytes: &mut Vec<u8>) -> String {
        BaseX::encode(bytes.as_mut_slice())
    }

    fn decode_raw(string: String) -> Option<Vec<u8>> {
        BaseX::decode(string)
    }

    fn encode_checked(buffer: &mut Vec<u8>) -> String {
        // var check = sha256(sha256(buffer)).slice(0, 4);

        let check: Vec<u8> = JSha256::sha256(&buffer);
        //6. take 0..4
        let token = check.get(..4).unwrap().to_vec();

        concat_args(buffer, &token);

        CodecFactory::encode_raw(buffer)
    }

    fn decode_checked(encoded: String) -> Option<Vec<u8>> {
        let bytes = CodecFactory::decode_raw(encoded).unwrap();
        if bytes.len() < 5 {
           panic!("invalid_input_size");
        }

        if CodecFactory::verify_checksum(&bytes) {
            panic!("checksum_invalid");
        }

        // buf.slice(0, -4)
        Some(bytes[0..bytes.len()-4].to_vec())
    }

    fn encode_versioned(bytes: &mut Vec<u8>, arg: Box<dyn BaseDataI>) -> String {
        let expected_length = arg.get_expected_length().unwrap();
        let version = arg.get_version().unwrap();
        if expected_length == 0 && bytes.len() != expected_length {
            panic!("unexpected_payload_length");
        }

        concat_args( bytes, &hexcast::decode(version).unwrap());

        CodecFactory::encode_checked(bytes)
    }

    fn decode_versioned(string: String, arg: Box<dyn BaseDataI>) -> Option<Vec<u8>> {
        CodecFactory::decode_multi_versioned(&string, arg)
    }
    //
    fn decode_multi_versioned(_encoded: &String, _arg: Box<dyn BaseDataI>) -> Option<Vec<u8>> {

        None
        // let without_sum = CodecFactory::decodeChecked(encoded);
        //
        // let expected_length = arg.get_expected_length().unwrap();
        //
        // var ret = { version: null, bytes: null };
        //
        // if possible_versions.len() > 1 && expected_length == 0 {
        //     panic!("must pass expectedLengthgth > 1 possibleVersions");
        // }
        //
        // let versionlen_guess = possible_versions[0].len() || 1; // Number.length
        // let payload_length = arg.get_expected_length() || without_sum.len() - versionlen_guess;
        // let version_bytes = without_sum.slice(0, -payload_length);
        // let payload = without_sum.slice(-payload_length);
        //
        // var foundVersion = possibleVersions.some(function (version, i) {
        //   var asArray = Array.isArray(version) ? version : [version];
        //   if (seqEqual(versionBytes, asArray)) {
        //     ret.version = version;
        //     ret.bytes = payload;
        //     if (types) {
        //       ret.type = types[i];
        //     }
        //     return true;
        //   }
        // });
        //
        // if ! found_version {
        //     panic!("version_invalid");
        // }
        //
        // if expected_length == 0 && ret.bytes.length !== expected_length) {
        //     panic!("unexpected_payload_length");
        // }
        //
        // return ret;
    }

    fn verify_checksum(bytes: &Vec<u8>) -> bool {
        // let computed = sha256(sha256(bytes.slice(0, -4))).slice(0, 4);
        // let checksum = bytes.slice(0..-4);

        let computed = JSha256::sha256(&bytes[0..bytes.len()-4].to_vec()).get(..4).unwrap().to_vec();
        let checksum = bytes[0..bytes.len()-4].to_vec();

        seq_equal(&computed, &checksum)
    }

    // fn find_prefix(&self, desired_prefix: &str, payload_length: usize) {
    //     if (this.base !== 58) {
    //         panic!("Only works for base58");
    //     }
    //
    //     let total_length = payload_length + 4;
    //     let chars = Math.log(Math.pow(256, total_length)) / Math.log(this.base);
    //
    //     let requiredChars = Math.ceil(chars + 0.2);
    //     let padding = this.alphabet[Math.floor(this.alphabet.length / 2) - 1];
    //     let template = desiredPrefix + new Array(requiredChars + 1).join(padding);
    //     let bytes = this.decodeRaw(template);
    //     let version = bytes.slice(0, -totalLength);
    //
    //     return version;
    // }
}
