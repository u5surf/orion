// Copyright (c) 2018 brycx

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Sodiumoxide test vector: https://github.com/sodiumoxide/sodiumoxide/blob/master/src/crypto/aead/xchacha20poly1305_ietf.rs
// Pulled at commit: https://github.com/sodiumoxide/sodiumoxide/commit/0be9734cc2ddb07ac0cd7cc67cf48afd6982cc91
#[cfg(test)]
mod other_aead_xchacha20_poly1305 {

    extern crate orion;
    use self::orion::hazardous::aead;

    #[test]
    fn test_case_0() {
        let key = [
            0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d,
            0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b,
            0x9c, 0x9d, 0x9e, 0x9f,
        ];
        let nonce = [
            0xf2, 0x8a, 0x50, 0xa7, 0x8a, 0x7e, 0x23, 0xc9, 0xcb, 0xa6, 0x78, 0x34, 0x66, 0xf8,
            0x03, 0x59, 0x0f, 0x04, 0xe9, 0x22, 0x31, 0xa3, 0x2d, 0x5d,
        ];
        let aad = [
            0x50, 0x51, 0x52, 0x53, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
        ];
        let plaintext = [
            0x4c, 0x61, 0x64, 0x69, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x47, 0x65, 0x6e,
            0x74, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
            0x63, 0x6c, 0x61, 0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x27, 0x39, 0x39, 0x3a, 0x20,
            0x49, 0x66, 0x20, 0x49, 0x20, 0x63, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x66,
            0x65, 0x72, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e,
            0x65, 0x20, 0x74, 0x69, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
            0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x2c, 0x20, 0x73, 0x75, 0x6e, 0x73, 0x63, 0x72,
            0x65, 0x65, 0x6e, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x69,
            0x74, 0x2e,
        ];
        let expected_ct = [
            0x20, 0xf1, 0xae, 0x75, 0xe1, 0xe5, 0xe0, 0x00, 0x40, 0x29, 0x4f, 0x0f, 0xb1, 0x0e,
            0xbb, 0x08, 0x10, 0xc5, 0x93, 0xc7, 0xdb, 0xa4, 0xec, 0x10, 0x4c, 0x1e, 0x5e, 0xf9,
            0x50, 0x7f, 0xae, 0xef, 0x58, 0xfc, 0x28, 0x98, 0xbb, 0xd0, 0xe4, 0x7b, 0x2f, 0x53,
            0x31, 0xfb, 0xc3, 0x67, 0xd3, 0xc2, 0x78, 0x4e, 0x36, 0x48, 0xce, 0x1e, 0xaa, 0x77,
            0x87, 0xad, 0x18, 0x6d, 0xb2, 0x68, 0x5e, 0xe8, 0x9a, 0xe4, 0xd3, 0x44, 0x1f, 0x6e,
            0xa0, 0xb2, 0x22, 0x4c, 0xd5, 0xa1, 0x34, 0x16, 0x1b, 0x55, 0x4d, 0x8b, 0x48, 0x35,
            0x0b, 0x4a, 0xd4, 0x01, 0x15, 0xdb, 0x81, 0xea, 0x82, 0x09, 0x68, 0xe9, 0x43, 0x89,
            0x2f, 0x2b, 0x80, 0x51, 0xcb, 0x5f, 0x7a, 0x86, 0x66, 0xe7, 0xe7, 0xef, 0x7f, 0x84,
            0xc0, 0xa2, 0xf8, 0x0a, 0x12, 0xd0, 0x66, 0x80, 0xc8, 0xee, 0xbb, 0xd9, 0x30, 0x04,
            0x10, 0x9d, 0xe8, 0x42,
        ];

        // This test vectors ciphertext already includes the tag
        // So the default test runner can't be used
        let mut dst_out_ct = vec![0u8; expected_ct.len()];
        let mut dst_out_pt = vec![0u8; plaintext.len()];

        aead::xchacha20poly1305::seal(
            &aead::xchacha20poly1305::SecretKey::from_slice(&key).unwrap(),
            &aead::xchacha20poly1305::Nonce::from_slice(&nonce).unwrap(),
            &plaintext,
            &aad,
            &mut dst_out_ct,
        ).unwrap();

        assert_eq!(
            dst_out_ct[..plaintext.len()].as_ref(),
            expected_ct[..plaintext.len()].as_ref()
        );
        // Verify tags
        assert_eq!(
            dst_out_ct[plaintext.len()..].as_ref(),
            expected_ct[plaintext.len()..].as_ref()
        );

        aead::xchacha20poly1305::open(
            &aead::xchacha20poly1305::SecretKey::from_slice(&key).unwrap(),
            &aead::xchacha20poly1305::Nonce::from_slice(&nonce).unwrap(),
            &dst_out_ct,
            &aad,
            &mut dst_out_pt,
        ).unwrap();

        assert_eq!(dst_out_pt[..].as_ref(), plaintext.as_ref());
    }
}
