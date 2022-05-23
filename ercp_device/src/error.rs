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

use std::{fmt::Display, string::FromUtf8Error};

use ercp_basic::{
    adapter::SerialPortAdapter, command::NewCommandError, Adapter,
};

/// An error that can happen when reading from or writing to the serial port.
pub type IoError = <SerialPortAdapter as Adapter>::Error;

/// A system-level error that can happen while sending / receiving a command.
pub type CommandError = ercp_basic::CommandError<IoError>;

/// A system-level error that can happen while receiving a command.
pub type ReceivedCommandError = ercp_basic::ReceivedCommandError<IoError>;

/// The result of a command.
pub type CommandResult<T, E> = ercp_basic::CommandResult<T, E, IoError>;

/// An error that can happen when sending a custom command.
pub enum CustomCommandError {
    /// An error has happened while building the command.
    NewCommandError(NewCommandError),
    /// An error has happened while sending the command or receiving the reply.
    CommandError(CommandError),
}

/// An error that can happen when receiving a log notification.
pub enum LogNotificationError {
    /// An error has occured while receiving the notification.
    ReceivedCommandError(ReceivedCommandError),
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

impl From<CommandError> for CustomCommandError {
    fn from(error: CommandError) -> Self {
        Self::CommandError(error)
    }
}

impl From<ReceivedCommandError> for LogNotificationError {
    fn from(error: ReceivedCommandError) -> Self {
        Self::ReceivedCommandError(error)
    }
}

impl From<FromUtf8Error> for LogNotificationError {
    fn from(error: FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}

impl Display for CustomCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewCommandError(NewCommandError::TooLong) => {
                write!(f, "the value is too long")
            }
            Self::CommandError(error) => write!(f, "{error}"),
        }
    }
}

impl Display for LogNotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReceivedCommandError(error) => {
                write!(f, "{error}")
            }
            Self::UnexpectedFrame => write!(f, "unexpected frame"),
            Self::FromUtf8Error(_) => {
                write!(f, "the received string is not valid UTF-8")
            }
        }
    }
}
