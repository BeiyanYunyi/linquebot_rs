use log::warn;
use rand::seq::SliceRandom;
use teloxide_core::prelude::*;
use teloxide_core::types::*;

use crate::assets::answer_book;
use crate::linquebot::*;
use crate::Consumption;

fn on_message(app: &'static App, message: &Message) -> Consumption {
    let _ = app.parse_command(message.text()?, "answer")?;
    let chat_id = message.chat.id;
    let message_id = message.id;

    Consumption::StopWith(Box::pin(async move {
        let chosen = answer_book::ANSWERS
            .choose(&mut rand::thread_rng())
            .expect("not empty");
        let res = app
            .bot
            .send_message(chat_id, *chosen)
            .reply_parameters(ReplyParameters::new(message_id))
            .send()
            .await;
        if let Err(err) = res {
            warn!("Failed to send reply: {}", err.to_string());
        }
    }))
}

pub static MODULE: Module = Module {
    kind: ModuleKind::Command(ModuleDesctiption {
        name: "answer",
        description: "答案之书",
        description_detailed: None,
    }),
    task: on_message,
};
