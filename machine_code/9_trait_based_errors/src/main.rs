use std::{fs, path::Path};

use serde_json::{Value, json};

use crate::{
    err::AppError,
    renderer::{ErrorMode, ErrorRenderer, create_error_renderer},
    req::{RawCommand, RequestType},
};

mod err;
mod renderer;
mod req;

//-------------------------------------------------------------------------------------------------------------
fn main() {
    let json_file_path = Path::new("sample.json");
    let input = match fs::read_to_string(json_file_path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("Could not read sample.json: {error}");
            return;
        }
    };

    // let error_mode = ErrorMode::Minimal;
    let error_mode = ErrorMode::Detailed;

    // Instead of if/else at the call site, use a strategy object chosen by config
    let renderer = create_error_renderer(error_mode);

    //here as_ref to pass without box
    let response = process_json(&input, renderer.as_ref());

    println!("{response}");
}

//-------------------------------------------------------------------------------------------------------------
//Box<dyn Trait> to own/store/return a trait object and &dyn Trait to borrow and call trait methods.
pub fn process_json(input: &str, error_renderer: &dyn ErrorRenderer) -> String {
    match process_json_result(input) {
        Ok(response) => response.to_string(),
        Err(error) => error_renderer.render(&error),
    }
}

//-------------------------------------------------------------------------------------------------------------
fn process_json_result(input: &str) -> Result<Value, AppError> {
    //? will directly return AppError because 

    // let value: Value = serde_json::from_str(input)?;
    let value: Value = serde_json::from_str(input).map_err(AppError::json)?;

    if value.is_array() {

        // let raw_commands: Vec<RawCommand> = serde_json::from_value(value)?;
        let raw_commands: Vec<RawCommand> = serde_json::from_value(value).map_err(AppError::json)?;
        
        let mut results = Vec::new();

        for raw_command in raw_commands {
            let command = RequestType::try_from(raw_command)?;
            results.push(acknowledge_command(command));
        }

        Ok(json!({
            "success": true,
            "results": results
        }))
    } else {

        // let raw_command: RawCommand = serde_json::from_value(value)?;
        let raw_command: RawCommand = serde_json::from_value(value).map_err(AppError::json)?;

        let command = RequestType::try_from(raw_command)?;
        Ok(acknowledge_command(command))
    }
}

//-------------------------------------------------------------------------------------------------------------
fn acknowledge_command(command: RequestType) -> Value {
    let command_name = match command {
        RequestType::CreateAccount(command) => {
            let _validated_fields = (command.account_id, command.owner);
            "create_account"
        }
        RequestType::Withdraw(command) => {
            let _validated_fields = (command.account_id, command.amount);
            "withdraw"
        }
        RequestType::Deposit(command) => {
            let _validated_fields = (command.account_id, command.amount);
            "deposit"
        }
    };

    json!({
        "success": true,
        "message": "Command accepted",
        "command": command_name
    })
}
