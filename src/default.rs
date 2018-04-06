use hmac::Hmac;
use hkdf::Hkdf;
use util;
use options::ShaVariantOption;

/// HMAC with SHA512
pub fn hmac(secret_key: Vec<u8>, message: Vec<u8>) -> Vec<u8> {

    if secret_key.len() < 64 {
        panic!("The secret_key must be equal to, or above 64 bytes in length.");
    }


    let hmac_512_res = Hmac {
        secret_key: secret_key,
        message: message,
        sha2: ShaVariantOption::SHA512
    };

    hmac_512_res.hmac_compute()
}

/// HKDF with HMAC-SHA512
pub fn hkdf(salt: Vec<u8>, data: Vec<u8>, info: Vec<u8>, length: usize) -> Vec<u8> {

    if salt.len() < 64 {
        panic!("The salt must be equal to, or above, 64 bytes in length.");
    }


    let hkdf_512_res = Hkdf {
        salt: salt,
        ikm: data,
        info: info,
        hmac: ShaVariantOption::SHA512,
        length: length
    };

    hkdf_512_res.hkdf_compute()
}

pub fn hmac_validate(expected_hmac: &[u8], secret_key: &[u8], message: &[u8]) -> bool {

    let rand_key = util::gen_rand_key(64);

    let own_hmac = hmac(secret_key.to_vec(), message.to_vec());

    let nd_round_own = hmac(rand_key.clone(), own_hmac);

    let nd_round_expected = hmac(rand_key.clone(), expected_hmac.to_vec());

    util::compare_ct(&nd_round_own, &nd_round_expected)
}


#[cfg(test)]
mod test {

    extern crate hex;
    use self::hex::decode;
    use default;

    #[test]
    #[should_panic]
    fn hmac_secretkey_too_short() {
        default::hmac(vec![0x61; 10], vec![0x61; 10]);
    }

    #[test]
    fn hmac_secretkey_allowed_len() {
        default::hmac(vec![0x61; 64], vec![0x61; 10]);
        default::hmac(vec![0x61; 78], vec![0x61; 10]);
    }

    #[test]
    fn hmac_result() {

        let sec_key = decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaa").unwrap();
        let msg = decode("54657374205573696e67204c6172676572205468616e20426c6f636b2d53697a\
              65204b6579202d2048617368204b6579204669727374").unwrap();

        let expected_hmac_512 = decode(
            "80b24263c7c1a3ebb71493c1dd7be8b49b46d1f41b4aeec1121b013783f8f352\
            6b56d037e05f2598bd0fd2215d6a1e5295e64f73f63f0aec8b915a985d786598").unwrap();

        assert_eq!(default::hmac(sec_key, msg), expected_hmac_512);
    }

    #[test]
    // Test that hmac_validate() returns true if signatures match and false if not
    fn hmac_validate() {

        let sec_key_correct = decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaa").unwrap();
              // Two additional a's in the end
        let sec_key_false = decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
              aaaaaaaa").unwrap();
        let msg = "what do ya want for nothing?".as_bytes().to_vec();

        let hmac_bob = default::hmac(sec_key_correct.clone(), msg.clone());

        assert_eq!(default::hmac_validate(&hmac_bob, &sec_key_correct, &msg), true);
        assert_eq!(default::hmac_validate(&hmac_bob, &sec_key_false, &msg), false);
    }

    #[test]
    #[should_panic]
    fn hkdf_salt_too_short() {
        default::hkdf(vec![0x61; 10], vec![0x61; 10], vec![0x61; 10], 20);
    }

    #[test]
    fn hkdf_salt_allowed_len() {
        default::hkdf(vec![0x61; 67], vec![0x61; 10], vec![0x61; 10], 20);
        default::hkdf(vec![0x61; 89], vec![0x61; 10], vec![0x61; 10], 20);
    }
}
