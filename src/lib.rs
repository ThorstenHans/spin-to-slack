mod models;

use anyhow::Result;
use models::SlackMessage;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::models::{Configuration, Message};

#[http_component]
fn configuration_in_spin(req: Request) -> Result<Response> {
    let c = Configuration::new();
    match c {
        Ok(cfg) => {
            let body = req.body().clone().unwrap_or_default();
            let inbound_message : Message = serde_json::from_slice(&body)?;
            send_to_slack(inbound_message, &cfg)

        },
        Err(error) => {
            println!("Error while reading configuration: {}", error);
            Ok(http::Response::builder().status(500).body(None)?)
        }
    }
    
}

fn send_to_slack(inbound: Message, cfg: &Configuration) -> Result<Response> {
    let msg = SlackMessage::new(
        cfg.channel.clone(), 
        inbound.message,
        cfg.is_markdown);

        let payload = serde_json::to_string(&msg)?;

    let res = spin_sdk::http::send(
        http::Request::builder()
                .uri(cfg.slack_webhook_url.clone())
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Some(payload.into()))?,
    )?;

    Ok(res)
}
