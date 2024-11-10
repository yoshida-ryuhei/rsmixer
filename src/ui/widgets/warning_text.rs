use super::Widget;
use crate::{
	models::Style,
	prelude::*,
	ui::{Buffer, Rect},
};

#[derive(Clone, Default)]
pub struct WarningTextWidget {
	pub text: String,
}

impl Widget for WarningTextWidget {
	fn resize(&mut self, _area: Rect) -> Result<()> {
		Ok(())
	}
	fn render(&mut self, buffer: &mut Buffer) -> Result<()> {
		buffer.rect(
			Rect::new(0, 0, buffer.width, buffer.height),
			' ',
			Style::Normal,
		);
		buffer.string(0, 0, self.text.clone(), Style::Normal);

		Ok(())
	}
}
