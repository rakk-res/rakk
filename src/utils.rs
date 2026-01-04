pub mod crypto {
    pub fn sign_message(msg: &[u8]) -> String {
        // Mock signing
        hex::encode(msg)
    }
}
