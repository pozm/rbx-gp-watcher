use std::collections::HashMap;

use rbx_gp_watcher::{get_gamepasses, send_webhook, GamePassRep, get_thumbnail};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut gpid = 0u64;

    let universe_id = std::env::var("UNIVERSE_ID").expect("unable to get UNIVERSE_ID from env").parse::<u64>().expect("unable to parse universe id to u64");
    let webhook_url = std::env::var("WEBHOOK_URL").expect("unable to get WEBHOOK_URL from env");
    
    let req_client = reqwest::Client::new();

    let mut update_lookup = HashMap::new();

    loop {
        async {

            let Ok(gamepasses) = get_gamepasses(&req_client,universe_id).await else {
                return;
            };
            let Some(first_gamepass) = gamepasses.first() else {
                return;
            };
            for gp in &gamepasses {
                let Some(already_exists) = update_lookup.get(&gp.id) else {
                    let Ok(thumb) = get_thumbnail(&req_client,gp.id).await else {
                        continue;
                    };
                    let Ok(rep) = GamePassRep::from_gp(&req_client, gp, &thumb).await else {
                        continue;
                    };
                    update_lookup.insert(gp.id,rep);
                    continue;
                };
                let Ok(thumb) = get_thumbnail(&req_client,gp.id).await else {
                    continue;
                };
                let Ok(rep) = GamePassRep::from_gp(&req_client, gp, &thumb).await else {
                    continue;
                };
                if already_exists.get_hash() != rep.get_hash() {
                    update_lookup.insert(gp.id,rep);
                    println!("Updated gamepass: {}",gp.name);
                    if let Err(err_msg) = send_webhook(&req_client, &webhook_url, gp).await {
                        println!("Failed to send webhook. Error: {}",err_msg);
                    }
                }
                
            }
            if first_gamepass.id == gpid {
                println!("No new gamepasses found. ({} is still newest)",first_gamepass.name);
                return;
            }
            gpid = if gpid == 0 {
                println!("First gamepass: {}",first_gamepass.name);
                first_gamepass.id
            } else {
                println!("New gamepass: {}",first_gamepass.name);
                if let Err(err_msg) = send_webhook(&req_client, &webhook_url, first_gamepass).await {
                    println!("Failed to send webhook. Error: {}",err_msg);
                }
                first_gamepass.id
            }
        }.await;
        tokio::time::sleep(std::time::Duration::from_secs(60 * 8)).await;
    }
}