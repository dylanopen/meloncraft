#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DatapackMetadata {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
