use serde::{Serialize, Deserialize};

use crate::contracts;
use crate::types::TxRef;
use crate::TransactionStatus;

/// HelloWorld contract states.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HelloWorld {
    whisper: Vec<u8>,
}

/// The commands that the contract accepts from the blockchain. Also called transactions.
/// Commands are supposed to update the states of the contract.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Increments the whisper in the contract by some number
    SetWhisper {
        value: Vec<u8>,
    },
}

/// The errors that the contract could throw for some queries
#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    NotAuthorized,
    SomeOtherError,
}

/// Query requests. The end users can only query the contract states by sending requests.
/// Queries are not supposed to write to the contract states.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Ask for the value of the whisper
    GetWhisper,
}

/// Query responses.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    /// Returns the value of the whisper
    GetWhisper {
        whisper: Vec<u8>,
    },
    /// Something wrong happened
    Error(Error)
}


impl HelloWorld {
    /// Initializes the contract
    pub fn new() -> Self {
        Default::default()
    }
}

impl contracts::Contract<Command, Request, Response> for HelloWorld {
    // Returns the contract id
    fn id(&self) -> contracts::ContractId { contracts::HELLO_WORLD }

    // Handles the commands from transactions on the blockchain. This method doesn't respond.
    fn handle_command(&mut self, _origin: &chain::AccountId, _txref: &TxRef, cmd: Command) -> TransactionStatus {
        match cmd {
            // Handle the `Increment` command with one parameter
            Command::SetWhisper { value } => {
                // Simply increment the whisper by some value.
                self.whisper = value;
                // Returns TransactionStatus::Ok to indicate a successful transaction
                TransactionStatus::Ok
            },
        }
    }

    // Handles a direct query and responds to the query. It shouldn't modify the contract states.
    fn handle_query(&mut self, _origin: Option<&chain::AccountId>, req: Request) -> Response {
        let inner = || -> Result<Response, Error> {
            match req {
                // Hanlde the `GetWhisper` request.
                Request::GetWhisper => {
                    // Respond with the whisper in the contract states.
                    Ok(Response::GetWhisper { whisper: self.whisper })
                },
            }
        };
        match inner() {
            Err(error) => Response::Error(error),
            Ok(resp) => resp
        }
    }
}

