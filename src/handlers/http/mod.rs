use crate::commands::http::HttpCommands;

pub fn execute_http_command(command: &HttpCommands) {
    println!("Executing HTTP command.");

    match command {
        HttpCommands::Get {} => {}
        HttpCommands::Post {} => {}
        HttpCommands::Put {} => {}
        HttpCommands::Delete {} => {}
        HttpCommands::Patch {} => {}
        HttpCommands::Head {} => {}
        HttpCommands::Options {} => {}
    }
}
