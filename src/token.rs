#[derive(Debug, Clone)]
pub struct Token(pub(crate) String);

impl Token {
    pub(crate) fn build_authorization_header(&self) -> String {
        format!("Authorization: Bearer {}", self.0)
    }
}