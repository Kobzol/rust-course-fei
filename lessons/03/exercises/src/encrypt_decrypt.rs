//! TODO: Implement storage of data in both encrypted and unencrypted forms.
//! Create a struct called `Data`, which will hold a Vec of bytes (`u8`) and it will allow
//! reading these bytes one by one through a `read(&mut self)` method. Once the data runs out, it
//! should return `None`.
//!
//! **`Data` and all its methods have to be public!**
//!
//! It will be possible to encrypt the data using an `encrypt` method, which receives a `u8` key
//! and returns a struct representing encrypted data. It will be possible to read encrypted data,
//! but every read byte will be encrypted, by XORing it with the provided key.
//! Hint: you don't need to encrypt everything eagerly, you can do it on-demand in the `read` method.
//!
//! It will be possible to decrypt encrypted data using a `decrypt` method on the encrypted data
//! type. `decrypt` receives a key, if the key matches the original key used for encryption, the
//! method will return the original unencrypted data. If it doesn't match, it will return the
//! encrypted data, so that users can still continue working with it.
//!
//! This exercise is designed so that you can play around with ownership:
//! - After data is encrypted, it should not be possible to access the original unencrypted data anymore.
//! - After data is decrypted, it should not be possible to access encrypted data anymore.
//!
//! Use ownership and move semantics to guarantee the rules described above.

// The doctests below should fail to compile.
// We need to use doctests, otherwise we could not check that the code does not compile.
// To debug the tests, try to remove the `compile_fail` attribute and run `cargo test` to see what
// happens. Once they fail because of the correct error (move semantics), put `compile_fail` back.

/// ```compile_fail
/// use week03::encrypt_decrypt::Data;
///
/// let mut data = Data::new(vec![1, 2, 3]);
/// let encrypted: Data = data.encrypt(5); // `encrypt` should return a different type
/// ```
#[allow(unused)]
fn encrypted_data_has_different_type() {}

/// ```compile_fail
/// use week03::encrypt_decrypt::Data;
///
/// let mut data = Data::new(vec![1, 2, 3]);
/// let encrypted = data.encrypt(5);
/// let encrypted = encrypted.encrypt(4); // `encrypt` should not be available here
/// ```
#[allow(unused)]
fn encrypted_data_cannot_encrypt() {}

/// ```compile_fail
/// use week03::encrypt_decrypt::Data;
///
/// let mut data = Data::new(vec![1, 2, 3]);
/// let encrypted = data.encrypt(5);
/// data.read(); // should be an error, `data` is not accessible anymore
/// ```
#[allow(unused)]
fn access_data_after_encrypting() {}

/// ```compile_fail
/// use week02::encrypt_decrypt::Data;
///
/// let mut data = Data::new(vec![1, 2, 3]);
/// let mut encrypted = data.encrypt(5);
/// encrypted.decrypt(5);
/// encrypted.read(); // should be an error, `encrypted` is not accessible anymore
/// ```
#[allow(unused)]
fn access_encrypted_data_after_decrypting() {}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn empty_data() {
        let mut data = Data::new(vec![]);
        assert_eq!(data.read(), None);
    }

    #[test]
    fn read_data() {
        let mut data = Data::new(vec![1, 2, 3]);
        assert_eq!(data.read(), Some(1));
        assert_eq!(data.read(), Some(2));
        assert_eq!(data.read(), Some(3));
        assert_eq!(data.read(), None);
    }

    #[test]
    fn encrypted_read_past_end() {
        let mut data = Data::new(vec![1, 2, 3]);
        assert_eq!(data.read(), Some(1));

        let mut encrypted = data.encrypt(0xCE);
        assert_eq!(encrypted.read(), Some(204));
        assert_eq!(encrypted.read(), Some(205));
        assert_eq!(encrypted.read(), None);
    }

    #[test]
    fn decrypt_wrong_key() {
        let mut data = Data::new(vec![1, 2, 3]);
        assert_eq!(data.read(), Some(1));

        let encrypted = data.encrypt(0xCE);
        match encrypted.decrypt(5) {
            Ok(_) => {
                panic!("Data was decrypted with a wrong key");
            }
            Err(mut decrypted) => {
                // Wrong key was used, `decrypt` returned the original encrypted data
                assert_eq!(decrypted.read(), Some(204));
            }
        }
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let mut data = Data::new(vec![1, 2, 3]);
        assert_eq!(data.read(), Some(1));

        let mut encrypted = data.encrypt(0xCE);
        assert_eq!(encrypted.read(), Some(204));

        let mut data = match encrypted.decrypt(0xCE) {
            Ok(d) => d,
            Err(_) => {
                panic!("Wrong key was used");
            }
        };
        assert_eq!(data.read(), Some(3));
        assert_eq!(data.read(), None);
    }
}
