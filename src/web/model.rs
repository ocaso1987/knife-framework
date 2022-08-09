use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pagination {
    pub page: u16,
    pub limit: u16,
}
