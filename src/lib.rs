use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use data::game_pass::{self, GamePass};
use reqwest::Client;
use sha2::{Sha256, Digest};

pub mod data;


pub async fn get_thumbnail(client:&Client,pass_id:u64) -> reqwest::Result<String> {
    let jsn = client.get(format!("https://thumbnails.roblox.com/v1/game-passes?gamePassIds={}&size=150x150&format=Png&isCircular=false",pass_id))
    .send().await?.json::<serde_json::Value>().await?;
    Ok(jsn.get("data").unwrap().as_array().unwrap().first().unwrap().get("imageUrl").unwrap().as_str().unwrap().to_string())
}
pub async fn get_gamepasses(client:&Client,universe_id:u64) -> reqwest::Result<Vec<game_pass::GamePass>> {
    let jsn = client.get(format!("https://games.roblox.com/v1/games/{}/game-passes?limit=10&sortOrder=Desc",universe_id))
    .send().await?.json::<game_pass::EndpointRoot>().await?;
    Ok(jsn.data)
}
pub async fn send_webhook(client:&Client,webhook_url:&str,pass:&game_pass::GamePass) -> reqwest::Result<String> {
    let thumbnail = get_thumbnail(client,pass.id).await?;
    let seller_url = format!("https://www.roblox.com/users/{}/profile",pass.seller_id.unwrap_or(-1));
    let json = serde_json::json!({
        "embeds": [
            {
                "title": pass.name,
                "description": pass.display_name,
                "url": format!("https://www.roblox.com/game-pass/{}/_",pass.id),
                "color": 0xe569c4,
                "type": "rich",
                "thumbnail": {
                    "url": thumbnail
                },
                "footer": {
                    "text": "rbx-gp-watcher ❤️ from @aixeria"
                },
                "fields": [
                    {
                        "name": "Price",
                        "value": format!("<:rx:1095962867448168608> {}",pass.price.unwrap_or(-1)),
                        "inline": true
                    },
                    {
                        "name": "Seller",
                        "value": format!("[{} ({})]({seller_url})",pass.seller_name,pass.seller_id.unwrap_or(-1)),
                        "inline": true
                    }
                ]
            }
        ]
    });
    let response = client.post(webhook_url).json(&json).send().await?.text().await?;
    Ok(response)
}

#[derive(Debug,Hash)]
pub struct GamePassRep {
    pub id: u64,
    pub name: String,
    pub price: Option<i64>,
    pub on_sale: bool,
    pub picture_hash: Vec<u8>,
}
impl GamePassRep {
    pub async fn from_gp(client: &Client, gp:&GamePass, img_url : &str) -> reqwest::Result<Self> {
        let img_bytes = client.get(img_url).send().await?.bytes().await?;
        let mut hasher = Sha256::new();
        hasher.update(&img_bytes);
        let hash : Vec<u8> = hasher.finalize().to_vec();
        Ok(GamePassRep {
            id: gp.id,
            name: gp.name.clone(),
            price: gp.price,
            on_sale: gp.price.is_some(),
            picture_hash: hash,
        })
    }
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}