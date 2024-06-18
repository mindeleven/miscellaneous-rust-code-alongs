/// single file executable for the chat client
/// chat client's first responsibility: reading commands from the user
/// in this example: reading lines from standard input

use async_std::prelude::*;
use async_chat::utils::{self, ChatResult};
use async_std::io;
use async_std::net;

async fn send_commands() -> ChatResult<()> {
    
    Ok(())
}