use std::fmt::Display;

use serde::{Deserialize, Serialize};
use spin_sdk::config;
use anyhow;
pub struct Configuration
{
    pub channel: String,
    pub is_markdown: bool,
    pub slack_webhook_url: String,
}

impl Configuration {
    pub fn new() -> Result<Self, anyhow::Error> {
        let channel =  config::get("channel")?;
        let is_markdown =  config::get("is_markdown")?.trim().parse()?;
        let slack_url = config::get("slack_webhook_url")?;
        Ok(Configuration {
            channel,
            is_markdown,
            slack_webhook_url: slack_url
        })
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Slack URL: {}\r\nChannel: {}\r\nIsMarkdown: {}\r\n", 
            &self.slack_webhook_url,
            &self.channel,
            &self.is_markdown)
    }
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message: String
}

#[derive(Serialize, Debug)]
pub struct SlackMessage {
    pub channel: String,
    pub username: String, 
    #[serde(rename="mrkdwn")]
    pub is_markdown: bool,
    pub text: String,
}

impl SlackMessage {
    pub fn new (channel: String, message: String, markdown: bool) -> Self {
        SlackMessage {
            username: String::from("Spin to Slack"),
            channel: channel,
            is_markdown: markdown,
            text: message,
        }
    }
}
