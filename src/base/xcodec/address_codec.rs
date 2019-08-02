
use crate::base::curve::sha256::JSha256;
use super::util::{is_set};
use crate::base::data::base_data::{BaseDataI};

pub trait XCodeI {
    fn encode();
    fn decode();

    fn encode_raw();
    fn decode_raw();

    fn encode_checked();
    fn decode_checked();

    fn encodeVersioned();
    fn decode_versioned();

    fn decode_multi_versioned();
    fn verify_checksum();
    fn find_prefix();
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
    fn encode(bytes: &mut Vec<u8>, arg: Box<dyn BaseDataI>) {
        let version = arg.get_version();
        if is_set(version) {
            CodecFactory::encode_versioned(bytes, version, arg.get_expected_length());
        } else {
            if  arg.get_checked() {
                CodecFactory::encodeChecked(bytes);
            } else {
                CodecFactory::encodeRaw(bytes);
            }
        }
    }

    fn decode(string: &String, opts: Box<dyn BaseDataI>) {
        let versions = opts.get_versions();
        if is_set(versions) {
            CodecFactory::decodeMultiVersioned(string, versions, opts.get_expected_length(), opts.get_version_type());
        } else {
            let version = opts.get_version();
            if isSet(version) {
                CodecFactory::decodeVersioned(string, version, opts.get_expected_length());
            } else {
                if opts.get_checked() {
                    CodecFactory::decodeChecked(string);
                } else {
                    CodecFactory::decodeRaw(string);
                }
            }
        }
    }

    fn encode_raw(&self, bytes: &Vec<u8>) -> String {
        BaseX::encode(version.as_mut_slice())
    }

    fn decode_raw(&self, string: String) -> Vec<u8> {
        BaseX::decode(string)
    }

    fn encode_checked(&self, buffer: &mut Vec<u8>) -> String {
        var check = sha256(sha256(buffer)).slice(0, 4);

        concat_args(buffer, check);

        CodecFactory::encode_raw(buffer)
    }

    fn decode_checked(&self, encoded: String) -> Vec<u8> {
        let buf = CodecFactory::decodeRaw(encoded);
        if buf.len() < 5 {
           panic!("invalid_input_size");
        }

        if CodecFactory::verifyCheckSum(buf) {
            panic!("checksum_invalid");
        }

        buf.slice(0, -4)
    }

    fn encode_versioned(bytes: &mut Vec<u8>, arg: Box<dyn BaseDataI>) -> String {
        if arg.get_expected_length().is_none() && bytes.len() != arg.get_expected_length() {
            panic!("unexpected_payload_length");
        }

        concat_args( bytes, version );

        CodecFactory::encode_checked()
    }

    fn decode_versioned(string: String, arg: Box<dyn BaseDataI>) -> Vec<u8> {
        CodecFactory::decode_multi_versioned(string, arg.get_version(), arg.get_expected_length(), None).bytes;
    }

    fn decode_multi_versioned(encoded: String, arg: Box<dyn BaseDataI>) -> Vec<u8> {
        let without_sum = CodecFactory::decodeChecked(encoded);
        var ret = { version: null, bytes: null };

        if (possibleVersions.length > 1 && !expectedLength) {
          throw new Error('must pass expectedLengthgth > 1 possibleVersions');
        }

        var versionLenGuess = possibleVersions[0].length || 1; // Number.length
        var payloadLength = expectedLength || withoutSum.length - versionLenGuess;
        var versionBytes = withoutSum.slice(0, -payloadLength);
        var payload = withoutSum.slice(-payloadLength);

        var foundVersion = possibleVersions.some(function (version, i) {
          var asArray = Array.isArray(version) ? version : [version];
          if (seqEqual(versionBytes, asArray)) {
            ret.version = version;
            ret.bytes = payload;
            if (types) {
              ret.type = types[i];
            }
            return true;
          }
        });

        if (!foundVersion) {
          throw new Error('version_invalid');
        }
        if (expectedLength && ret.bytes.length !== expectedLength) {
          throw new Error('unexpected_payload_length');
        }

        return ret;
      }
    fn verify_checksum(&self, bytes: &Vec<u8>) -> bool {
        let computed = sha256(sha256(bytes.slice(0, -4))).slice(0, 4);
        let checksum = bytes.slice(0..-4);

        seq_equal(computed, checksum)
    }

     fn find_prefix(&self, desiredPrefix, payloadLength) {
         if (this.base !== 58) {
          throw new Error('Only works for base58');
        }
        var totalLength = payloadLength + 4;
        var chars = Math.log(Math.pow(256, totalLength)) / Math.log(this.base);

        var requiredChars = Math.ceil(chars + 0.2);
        var padding = this.alphabet[Math.floor(this.alphabet.length / 2) - 1];
        var template = desiredPrefix + new Array(requiredChars + 1).join(padding);
        var bytes = this.decodeRaw(template);
        var version = bytes.slice(0, -totalLength);
        return version;
      }
