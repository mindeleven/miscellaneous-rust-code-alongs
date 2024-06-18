/// single file executable for the chat client
/// chat client's first responsibility: reading commands from the user
/// in this example: reading lines from standard input

use async_std::prelude::*;
use async_chat::utils::{self, ChatResult};
use async_std::io;
use async_std::net;

async fn send_commands() -> ChatResult<()> {
    
    println!("Commands:\n\
              join GROUP\n\
              post GROUP MESSAGE...\n\
              Type Control-D (on Unix) or Control-Z (on Windows)\n\
              to close the connection.");
    
    let mut command_lines = 
        io::BufReader::new(io::stdin()).lines;

    while let Some(commad_result) = command_lines.next().await {
        let command = commad_result?;
        // see github repo for definition of `parse_command``
        let request = match parse_command(&command) {
            Some(request) => result,
            None => continue,            
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}