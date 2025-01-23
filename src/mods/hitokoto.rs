//! hitokoto  
//! Send a hitokoto  
//! Usage:
//! ```
//! /hitokoto
//! ```

use log::trace;
use log::warn;
use serde::Deserialize;
use teloxide_core::prelude::*;
use teloxide_core::types::*;

use crate::linquebot::*;
use crate::utils::*;
use crate::Consumption;

#[derive(Deserialize, Debug)]
struct Hitokoto {
    hitokoto: String,
    from: String,
}

async fn get_hitokoto(args: &str) -> Hitokoto {
    let res: Result<_, reqwest::Error> = try {
        reqwest::get(format!("https://v1.hitokoto.cn/?c={}", args))
            .await?
            .json::<Hitokoto>()
            .await?
    };

    match res {
        Ok(res) => {
            trace!("Successfully fetched hitokoto: {res:?}");
            res
        }
        Err(err) => {
            warn!("Failed to fetch hitokoto: {}", err.to_string());
            Hitokoto {
                hitokoto: "网络错误".to_string(),
                from: "琳酱".to_string(),
            }
        }
    }
}

async fn hitokoto(bot: &Bot, chat_id: ChatId, message_id: MessageId, args: String) {
    let res = get_hitokoto(&args).await;

    let res = bot
        .send_message(chat_id, format!("{} ——{}", res.hitokoto, res.from))
        .reply_parameters(ReplyParameters::new(message_id))
        .send()
        .await;

    if let Err(err) = res {
        warn!("Failed to send reply: {}", err.to_string());
    }
}

fn on_message(app: &'static App, message: &Message) -> Consumption {
    let args = parse_command(message.text()?, "hitokoto")?;
    let args = args.split_whitespace().collect::<Vec<_>>().join("&c=");
    let chat_id = message.chat.id;
    let message_id = message.id;

    Consumption::StopWith(Box::pin(hitokoto(&app.bot, chat_id, message_id, args)))
}

pub static MODULE: Module = Module {
    kind: ModuleKind::Command(ModuleDesctiption {
        name: "hitokoto",
        description: "获取一言",
        description_detailed: None,
    }),
    task: on_message,
};
