// Copyright 2021-2022 Jean-Philippe Cugnet
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! ERCP Device errors.

use std::string::FromUtf8Error;

use ercp_basic::{
    adapter::SerialPortAdapter, command::NewCommandError, Adapter,
    CommandError, ReceivedCommandError,
};

/// An error that can happen when reading from or writing to the serial port.
pub type IoError = <SerialPortAdapter as Adapter>::Error;

/// The result of a command.
pub type CommandResult<T, E> = ercp_basic::CommandResult<T, E, IoError>;

/// An error that can happen when sending a custom command.
pub enum CustomCommandError {
    /// An error has happened while building the command.
    NewCommandError(NewCommandError),
    /// An error has happened while sending the command or receiving the reply.
    CommandError(CommandError<IoError>),
}

/// An error that can happen when receiving a log notification.
pub enum LogNotificationError {
    /// An error has occured while receiving the notification.
    ReceivedCommandError(ReceivedCommandError<IoError>),
    /// A frame has been received, but it is not a log notification.
    UnexpectedFrame,
    /// The received string is not valid UTF-8.
    FromUtf8Error(FromUtf8Error),
}

impl From<NewCommandError> for CustomCommandError {
    fn from(error: NewCommandError) -> Self {
        Self::NewCommandError(error)
    }
}

impl From<CommandError<IoError>> for CustomCommandError {
    fn from(error: CommandError<IoError>) -> Self {
        Self::CommandError(error)
    }
}

impl From<ReceivedCommandError<IoError>> for LogNotificationError {
    fn from(error: ReceivedCommandError<IoError>) -> Self {
        Self::ReceivedCommandError(error)
    }
}

impl From<FromUtf8Error> for LogNotificationError {
    fn from(error: FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}
