#![allow(clippy::upper_case_acronyms)]

extern crate crossbeam_channel as cb_channel;
extern crate libpulse_binding as pulse;

static LOGGING_MODULE: &str = "Main";

mod action_handlers;
mod actor_system;
mod actors;
mod cli_options;
mod config;
mod help;
mod models;
mod multimap;
mod pa;
mod prelude;
mod ui;
mod util;

use std::collections::HashMap;

use actors::*;
use cli_options::CliOptions;
use config::{RsMixerConfig, Variables};
use crossterm::style::ContentStyle;
use lazy_static::lazy_static;
use models::{entry, InputEvent, Style, UserAction};
use multimap::MultiMap;
use prelude::*;
use state::InitCell;
use tokio::runtime;

lazy_static! {
	pub static ref STYLES: InitCell<Styles> = InitCell::new();
	pub static ref VARIABLES: InitCell<Variables> = InitCell::new();
	pub static ref BINDINGS: InitCell<MultiMap<InputEvent, UserAction>> = InitCell::new();
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Styles = HashMap<Style, ContentStyle>;

fn load_config_and_options() -> Result<()> {
	info!("Checking command line options and config");

	CliOptions::check()?;
	debug!("CLI options checked");

	let mut config = RsMixerConfig::load()?;
	let (styles, bindings, variables) = config.interpret()?;

	STYLES.set(styles);
	BINDINGS.set(bindings);
	VARIABLES.set(variables);
	debug!("Config loaded");

	Ok(())
}

async fn run() -> Result<()> {
	load_config_and_options()?;

	debug!("Starting actor system");
	let (mut context, worker) = actor_system::new();

	let actor_system_handle = worker.start();

	EventLoopActor::item().register_and_start(&mut context);
	PulseActor::item().register_and_start(&mut context);
	InputActor::item().register_and_start(&mut context);

	debug!("Actor system started");
	actor_system_handle.await?
}

fn main() -> Result<()> {
	info!("Starting RsMixer");

	let threaded_rt = runtime::Builder::new_multi_thread().enable_time().build()?;
	threaded_rt.block_on(async {
		debug!("Tokio runtime started");

		if let Err(e) = run().await {
			println!("{e:#?}");
		}
	});

	Ok(())
}
