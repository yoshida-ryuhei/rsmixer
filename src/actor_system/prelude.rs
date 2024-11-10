use std::sync::Arc;

pub use async_trait::async_trait;
use tokio::sync::RwLock;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub use super::{
	actor::{Actor, ActorItem, BoxedResultFuture, ContinousActor, EventfulActor},
	context::Ctx,
	messages::{BoxedMessage, Shutdown},
	retry_strategy::PinnedClosure,
};

pub type LockedReceiver = Arc<RwLock<UnboundedReceiverStream<BoxedMessage>>>;
