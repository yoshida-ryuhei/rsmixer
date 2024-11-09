use super::{callbacks, common::*};

pub fn handle_command(
	cmd: PulseAudioAction,
	context: &Rc<RefCell<PAContext>>,
	info_sx: &mpsc::UnboundedSender<EntryIdentifier>,
) -> Option<()> {
	match cmd {
		PulseAudioAction::RequestPulseAudioState => {
			callbacks::request_current_state(Rc::clone(context), info_sx.clone()).unwrap();
		}
		PulseAudioAction::MuteEntry(ident, mute) => {
			set_mute(ident, mute, context);
		}
		PulseAudioAction::MoveEntryToParent(ident, parent) => {
			move_entry_to_parent(ident, parent, context, info_sx.clone());
		}
		PulseAudioAction::ChangeCardProfile(ident, profile) => {
			change_card_profile(ident, profile, context);
		}
		PulseAudioAction::SetVolume(ident, vol) => {
			set_volume(ident, vol, context);
		}
		PulseAudioAction::SetSuspend(ident, suspend) => {
			set_suspend(ident, suspend, context);
		}
		PulseAudioAction::KillEntry(ident) => {
			kill_entry(ident, context);
		}
		PulseAudioAction::Shutdown => {
			//@TODO disconnect monitors
			return None;
		}
		_ => {}
	};
	Some(())
}

fn set_volume(
	ident: EntryIdentifier,
	vol: pulse::volume::ChannelVolumes,
	context: &Rc<RefCell<PAContext>>,
) {
	let mut introspector = context.borrow_mut().introspect();
	match ident.entry_type {
		EntryType::Sink => {
			introspector.set_sink_volume_by_index(ident.index, &vol, None);
		}
		EntryType::SinkInput => {
			introspector.set_sink_input_volume(ident.index, &vol, None);
		}
		EntryType::Source => {
			introspector.set_source_volume_by_index(ident.index, &vol, None);
		}
		EntryType::SourceOutput => {
			introspector.set_source_output_volume(ident.index, &vol, None);
		}
		_ => {}
	};
}

fn change_card_profile(ident: EntryIdentifier, profile: String, context: &Rc<RefCell<PAContext>>) {
	if ident.entry_type != EntryType::Card {
		return;
	}
	context
		.borrow_mut()
		.introspect()
		.set_card_profile_by_index(ident.index, &profile[..], None);
}

fn move_entry_to_parent(
	ident: EntryIdentifier,
	parent: EntryIdentifier,
	context: &Rc<RefCell<PAContext>>,
	info_sx: mpsc::UnboundedSender<EntryIdentifier>,
) {
	let mut introspector = context.borrow_mut().introspect();

	match ident.entry_type {
		EntryType::SinkInput => {
			introspector.move_sink_input_by_index(
				ident.index,
				parent.index,
				Some(Box::new(move |_| {
					info_sx.send(parent).unwrap();
					info_sx.send(ident).unwrap();
				})),
			);
		}
		EntryType::SourceOutput => {
			introspector.move_source_output_by_index(
				ident.index,
				parent.index,
				Some(Box::new(move |_| {
					info_sx.send(parent).unwrap();
					info_sx.send(ident).unwrap();
				})),
			);
		}
		_ => {}
	};
}

fn set_suspend(ident: EntryIdentifier, suspend: bool, context: &Rc<RefCell<PAContext>>) {
	let mut introspector = context.borrow_mut().introspect();
	match ident.entry_type {
		EntryType::Sink => {
			introspector.suspend_sink_by_index(ident.index, suspend, None);
		}
		EntryType::Source => {
			introspector.suspend_source_by_index(ident.index, suspend, None);
		}
		_ => {}
	};
}

fn kill_entry(ident: EntryIdentifier, context: &Rc<RefCell<PAContext>>) {
	let mut introspector = context.borrow_mut().introspect();
	match ident.entry_type {
		EntryType::SinkInput => {
			introspector.kill_sink_input(ident.index, |_| {});
		}
		EntryType::SourceOutput => {
			introspector.kill_source_output(ident.index, |_| {});
		}
		_ => {}
	};
}

fn set_mute(ident: EntryIdentifier, mute: bool, context: &Rc<RefCell<PAContext>>) {
	let mut introspector = context.borrow_mut().introspect();
	match ident.entry_type {
		EntryType::Sink => {
			introspector.set_sink_mute_by_index(ident.index, mute, None);
		}
		EntryType::SinkInput => {
			introspector.set_sink_input_mute(ident.index, mute, None);
		}
		EntryType::Source => {
			introspector.set_source_mute_by_index(ident.index, mute, None);
		}
		EntryType::SourceOutput => {
			introspector.set_source_output_mute(ident.index, mute, None);
		}
		_ => {}
	};
}
