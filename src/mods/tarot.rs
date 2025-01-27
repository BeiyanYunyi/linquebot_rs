use std::time::Duration;

use log::warn;
use msg_context::Context;
use rand::seq::IteratorRandom;
use rand::Rng;
use teloxide_core::prelude::*;
use teloxide_core::types::*;

use crate::assets::tarot;
use crate::linquebot::*;
use crate::utils::telegram::prelude::*;
use crate::Consumption;

pub fn on_message(ctx: &mut Context, message: &Message) -> Consumption {
    let text = ctx.cmd?.content;
    let Some(from) = message.from.clone() else {
        warn!("No reply target.");
        return Consumption::Stop;
    };
    let num;
    if text.is_empty() {
        num = 3;
    } else {
        if let Ok(parsed) = text.parse::<usize>() {
            num = parsed;
        } else {
            return Consumption::StopWith(Box::pin(
                ctx.task()
                    .reply("数字不对，不准乱玩琳酱呀")
                    .send()
                    .warn_on_error("tarot"),
            ));
        };
    };
    if num == 0 {
        return Consumption::StopWith(Box::pin(
            ctx.task()
                .reply("不给你牌可以，可以给你一拳")
                .send()
                .warn_on_error("tarot"),
        ));
    }
    if num > 21 {
        return Consumption::StopWith(Box::pin(
            ctx.task()
                .reply("牌都给你摸完了，不准乱玩琳酱")
                .send()
                .warn_on_error("tarot"),
        ));
    }

    let ctx = ctx.task();
    return Consumption::StopWith(Box::pin(async move {
        ctx.reply(&format!(
            "{}最近遇到了什么烦心事吗？让琳酱给你算一算:",
            from.full_name()
        ))
        .send()
        .warn_on_error("tarot")
        .await;

        ctx.app
            .bot
            .send_chat_action(ctx.chat_id, ChatAction::Typing)
            .send()
            .warn_on_error("tarot")
            .await;

        tokio::time::sleep(Duration::from_millis(2500)).await;

        let text = tarot::MAJORS
            .iter()
            .choose_multiple(&mut rand::thread_rng(), num)
            .into_iter()
            .map(|arcana| {
                let cis: bool = rand::thread_rng().gen();
                if cis {
                    format!("{}正位: {}", arcana.name, arcana.cis)
                } else {
                    format!("{}逆位: {}", arcana.name, arcana.trans)
                }
            })
            .collect::<Vec<_>>();

        ctx.reply(&format!(
            "{} 抽到的牌组是: \n{}",
            from.full_name(),
            text.join("\n")
        ))
        .send()
        .warn_on_error("tarot")
        .await;
    }));
}

pub static MODULE: Module = Module {
    kind: ModuleKind::Command(ModuleDesctiption {
        name: "tarot",
        description: "抽取塔罗牌",
        description_detailed: Some(concat!("可选参数：数量\n", "默认摸 3 张。\n",)),
    }),
    task: on_message,
};
