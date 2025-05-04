#[derive(Debug, Clone)]
pub struct Nip(String);

impl Nip {
    pub fn new(nip: &str) -> Result<Self, String> {
        if nip.len() != 10 || !nip.chars().all(|c| c.is_digit(10)) {
            return Err("NIP must contain 10 letters".to_string());
        }
        
        Ok(Nip(nip.to_string()))
    }
}
