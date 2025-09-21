use std::io::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UIError {
	#[error("terminal window is too small")]
	TerminalTooSmall,
	#[error("crossterm terminal error")]
	TerminalError(#[from] Error),
	//	#[error("terminal io error")]
	//	IoError(#[from] std::io::Error),
}
