/// single file executable for the chat client
/// chat client's first responsibility: reading commands from the user
/// in this example: reading lines from standard input

use async_std::prelude::*;
use async_client_server::utils::{self, ChatResult};
use async_std::io;
use async_std::net;

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    
    println!("Commands:\n\
              join GROUP\n\
              post GROUP MESSAGE...\n\
              Type Control-D (on Unix) or Control-Z (on Windows)\n\
              to close the connection.");
    
    // calling async_std::io::stdtin() to get an async handle on the clients std input
    // and wrapping in io::BufReader to buffer it
    // then process the user's input line by line with lines()
    let mut command_lines = 
        io::BufReader::new(io::stdin()).lines();
    
    // if sucessful, value gets sent to server
    while let Some(commad_result) = command_lines.next().await {
        // parsing each line as a command corresponding to some FromClient value

        let command = commad_result?; // getting the String out ot the Result

        // see github repo for definition of `parse_command``
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,            
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}

