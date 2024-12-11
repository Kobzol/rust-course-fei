use crate::file::{to_encrypted_file, to_file, EncryptedFile, OpenedFile};

mod file {
    pub struct OpenedFile {
        pub fd: u32,
    }

    pub struct EncryptedFile {
        file: OpenedFile,
        key: u32,
    }
    pub fn to_encrypted_file(file: OpenedFile, key: u32) -> EncryptedFile {
        EncryptedFile { file, key }
    }

    pub fn to_file(file: EncryptedFile) -> OpenedFile {
        file.file
    }
}

fn main() {
    let mut file = OpenedFile { fd: 1 };
    // file.write(...);

    let mut encrypted_file = to_encrypted_file(file, 123);
    // file.write(...); // Error
    // encrypted_file.write(...); // OK

    let mut file = to_file(encrypted_file);
    // file.write();
}
