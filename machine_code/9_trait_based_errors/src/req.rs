use serde::{Deserialize, Serialize};

use crate::err::AppError;

//-------------------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
pub struct RawCommand {
    #[serde(rename = "type")]
    pub req_type: Option<String>,
    pub account_id: Option<String>,
    pub owner: Option<String>,
    pub initial_balance: Option<i64>,
    pub amount: Option<i64>,
}

pub enum RequestType {
    CreateAccount(CreateAccount),
    Withdraw(Withdraw),
    Deposit(Deposit),
}

//-------------------------------------------------------------------------------------------------------------
pub struct CreateAccount {
    pub account_id: String,
    pub owner: String,
}

pub struct Withdraw {
    pub account_id: String,
    pub amount: i64,
}

pub struct Deposit {
    pub account_id: String,
    pub amount: i64,
}

//-------------------------------------------------------------------------------------------------------------

impl TryFrom<RawCommand> for RequestType {
    type Error = AppError;

    fn try_from(raw: RawCommand) -> Result<Self, Self::Error> {
        let req_type = raw.req_type.ok_or_else(|| missing_field("type"))?;

        match req_type.as_str() {
            "create_account" => {
                let account_id = required_field(raw.account_id, "account_id", "create_account")?;
                let owner = required_field(raw.owner, "owner", "create_account")?;
                let initial_balance =
                    required_field(raw.initial_balance, "initial_balance", "create_account")?;

                let account_id = validate_non_empty(account_id, "account_id", "create_account")?;
                let owner = validate_non_empty(owner, "owner", "create_account")?;
                validate_non_negative(initial_balance, "initial_balance", "create_account")?;

                Ok(RequestType::CreateAccount(CreateAccount {
                    account_id,
                    owner,
                }))
            }
            "withdraw" => {
                let account_id = required_field(raw.account_id, "account_id", "withdraw")?;
                let amount = required_field(raw.amount, "amount", "withdraw")?;

                Ok(RequestType::Withdraw(Withdraw {
                    account_id: validate_non_empty(account_id, "account_id", "withdraw")?,
                    amount: validate_positive_amount(amount, "withdraw")?,
                }))
            }
            "deposit" => {
                let account_id = required_field(raw.account_id, "account_id", "deposit")?;
                let amount = required_field(raw.amount, "amount", "deposit")?;

                Ok(RequestType::Deposit(Deposit {
                    account_id: validate_non_empty(account_id, "account_id", "deposit")?,
                    amount: validate_positive_amount(amount, "deposit")?,
                }))
            }
            command => Err(AppError::UnsupportedCommand {
                command: command.to_string(),
            }),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------

fn missing_field(field: &'static str) -> AppError {
    AppError::Validation {
        code: "MISSING_FIELD",
        message: format!("{field} is required"),
    }
}

fn required_field<T>(
    value: Option<T>,
    field: &'static str,
    command: &'static str,
) -> Result<T, AppError> {
    value.ok_or_else(|| AppError::Validation {
        code: "MISSING_FIELD",
        message: format!("{field} is required for {command}"),
    })
}

fn validate_non_empty(
    value: String,
    field: &'static str,
    command: &'static str,
) -> Result<String, AppError> {
    if value.trim().is_empty() {
        return Err(AppError::Validation {
            code: "INVALID_FIELD",
            message: format!("{field} cannot be empty for {command}"),
        });
    }

    Ok(value)
}

fn validate_positive_amount(amount: i64, command: &'static str) -> Result<i64, AppError> {
    if amount <= 0 {
        return Err(AppError::Validation {
            code: "INVALID_AMOUNT",
            message: format!("amount must be greater than zero for {command}"),
        });
    }

    Ok(amount)
}

fn validate_non_negative(
    value: i64,
    field: &'static str,
    command: &'static str,
) -> Result<i64, AppError> {
    if value < 0 {
        return Err(AppError::Validation {
            code: "INVALID_AMOUNT",
            message: format!("{field} cannot be negative for {command}"),
        });
    }

    Ok(value)
}
