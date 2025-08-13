use reqwest::Client;
use serde_json::json;

// Bot Token （替换自己的）
const TELEGRAM_BOT_TOKEN: &str = "8419657234:AAErz3Z0zEWjn01No9X8e4aPRARqqgcWQK4";

// 群组ID（替换自己的）
const TELEGRAM_CHAT_ID: &str = "-1002856284231";
// 子话题ID，若无开启话题，可以忽略（替换自己的）
const TELEGRAM_TOPIC_ID: i64 = 3;

pub async fn send_to_tg(cex: &str, message: &str, url: Option<&str>) -> Result<(), anyhow::Error> {
    let text = if let Some(link) = url {
        format!(
            "<b>📢 {}上币</b>\n\n\
             {}\n\n\
             <a href=\"{}\">🔗 原文</a>",
            cex, message, link
        )
    } else {
        format!(
            "<b>📢 {}上币</b>\n\n\
             {}",
            cex, message
        )
    };

    let payload = json!({
        "chat_id": TELEGRAM_CHAT_ID,
        "text": text,
        "parse_mode": "HTML",
        "message_thread_id": TELEGRAM_TOPIC_ID,
    });

    let res = Client::new()
        .post(&format!(
            "https://api.telegram.org/bot{}/sendMessage",
            TELEGRAM_BOT_TOKEN
        ))
        .json(&payload)
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;
    if !status.is_success() {
        eprintln!("❌ Telegram 发送失败: {}", body);
    }

    Ok(())
}
