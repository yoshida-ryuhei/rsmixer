use anyhow::Result;
use crossterm::event::{Event, EventStream, MouseEventKind};
use tokio_stream::StreamExt;

use crate::{
	actor_system::prelude::*,
	models::{ResizeScreen, UserInput},
};

pub struct InputActor {}

impl InputActor {
	pub fn factory() -> Actor {
		Actor::Continous(Box::new(Self {}))
	}

	pub fn item() -> ActorItem {
		ActorItem::new("input", &Self::factory)
			.on_panic(|_| -> PinnedClosure { Box::pin(async { true }) })
			.on_error(|_| -> PinnedClosure { Box::pin(async { true }) })
	}
}

#[async_trait]
impl ContinousActor for InputActor {
	async fn start(&mut self, _ctx: Ctx) {}
	async fn stop(&mut self) {}

	fn run(&mut self, ctx: Ctx, events_rx: LockedReceiver) -> BoxedResultFuture {
		Box::pin(start(events_rx, ctx))
	}
}

pub async fn start(rx: LockedReceiver, ctx: Ctx) -> Result<()> {
	let mut reader = EventStream::new();
	let mut rx = rx.write().await;

	loop {
		let input_event = reader.next();
		let recv_event = rx.next();

		tokio::select! {
			ev = input_event => {
				let ev = if let Some(ev) = ev { ev } else { continue; };
				let ev = if let Ok(ev) = ev { ev } else { continue; };

				match ev {
					Event::Key(_) => {
						ctx.send_to("event_loop", UserInput::new(ev));
					}
					Event::Mouse(me) => {
						if MouseEventKind::Moved != me.kind {
							ctx.send_to("event_loop", UserInput::new(ev));
						}
					}
					Event::Resize(_, _) => {
						ctx.send_to("event_loop", ResizeScreen::new());
					}
					_ => todo!(),
				};
			}
			ev = recv_event => {
				let ev = if let Some(ev) = ev { ev } else { continue; };
				if ev.is::<Shutdown>() {
					return Ok(());
				}
			}
		};
	}
}
