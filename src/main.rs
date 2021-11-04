use std::borrow::Cow;
use sedregex::ReplaceCommand;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting SadRustyBot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        if message.update.via_bot.is_some() { return respond(()); };
        if let Some(result) = try_apply(&message.update) {
            message.requester
                .send_message(message.chat_id(), result)
                .reply_to_message_id(message.update.reply_to_message().unwrap().id)
                .send()
                .await?;
        }
        respond(())
    }).await;
}


fn try_apply(msg: &Message) -> Option<Cow<str>> {
    let to_subst = msg.reply_to_message()?;
    let src = to_subst.text()?;
    msg.text()?
        .lines()
        .map(|line| ReplaceCommand::new(line).ok())
        .fold(Some(src.into()),
              |acc, cmd| acc.zip(cmd).map(|(acc, cmd)| cmd.execute(acc)))
}
