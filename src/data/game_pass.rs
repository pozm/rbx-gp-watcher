use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePass {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub product_id: Option<i64>,
    pub price: Option<i64>,
    pub seller_name: String,
    pub seller_id: Option<i64>,
    pub is_owned: bool,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointRoot {
    pub data: Vec<GamePass>,
    pub next_page_cursor: Option<String>,
    pub previous_page_cursor: Option<String>,
}