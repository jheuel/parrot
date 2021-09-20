use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::utils::send_simple_message;

#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.unwrap().id;
    let manager = songbird::get(ctx).await.expect("Could not retrieve Songbird voice client");

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;
        let queue = handler.queue();

        if queue.is_empty() {
            send_simple_message(&ctx.http, msg, "The queue is already empty!").await;
        } else {
            queue.stop();
            send_simple_message(&ctx.http, msg, "Stopped!").await;
        }
    } else {
        send_simple_message(&ctx.http, msg, "I'm not connected to any voice channel!").await;
    }

    Ok(())
}
