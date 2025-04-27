use thiserror::Error;
use std::io::ErrorKind;

#[derive(Debug, Error)]
 pub enum UIError {
     #[error("terminal window is too small")]
     TerminalTooSmall,
     #[error("crossterm terminal error")]
     TerminalError(#[from] ErrorKind),
	//	#[error("terminal io error")]
	//	IoError(#[from] std::io::Error),
}

