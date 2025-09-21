use std::{
	collections::{BTreeMap, HashMap},
	io::{Result, Write},
	iter::Iterator,
};

use crossterm::{
	cursor, queue,
	style::{self, ContentStyle},
};

use super::Rect;
use crate::models::Style;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Pixel {
	pub text: Option<char>,
	pub style: Style,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Pixels(Vec<Pixel>);

impl Pixels {
	pub fn next(mut self, style: Style, s: char) -> Self {
		self.0.push(Pixel {
			style,
			text: Some(s),
		});
		self
	}
	pub fn string(mut self, style: Style, s: &str) -> Self {
		for c in s.chars() {
			self.0.push(Pixel {
				style,
				text: Some(c),
			});
		}
		self
	}
	pub fn get_mut(&mut self, index: usize) -> Option<&mut Pixel> {
		self.0.get_mut(index)
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pixel> + '_ {
		self.0.iter_mut()
	}
}

impl From<Vec<Pixel>> for Pixels {
	fn from(s: Vec<Pixel>) -> Self {
		Self(s)
	}
}
impl From<Pixels> for Vec<Pixel> {
	fn from(s: Pixels) -> Self {
		s.0
	}
}

#[derive(Default)]
pub struct Buffer {
	pub width: u16,
	pub height: u16,
	pixels: Vec<Pixel>,
	changes: BTreeMap<usize, Pixel>,
	pub styles: HashMap<Style, ContentStyle>,
}

impl Buffer {
	pub fn set_styles(&mut self, styles: HashMap<Style, ContentStyle>) {
		self.styles = styles;
	}

	pub fn resize(&mut self, width: u16, height: u16) {
		self.width = width;
		self.height = height;
		self.pixels = (0..width * height).map(|_| Pixel::default()).collect();
	}

	pub fn draw_changes<W: Write>(&mut self, stdout: &mut W) -> Result<()> {
		let mut last_style = None;
		let mut last_coord = None;
		let mut text = "".to_string();

		for (k, v) in self.changes.iter() {
			if let Some(pixel) = self.pixels.get(*k) {
				if *pixel != *v {
					self.pixels[*k] = *v;
				} else {
					continue;
				}
			} else {
				continue;
			}

			if last_style != Some(v.style) || *k == 0 || last_coord != Some(*k - 1) {
				if !text.is_empty() {
					let style = match self.styles.get(&last_style.unwrap()) {
						Some(s) => *s,
						None => ContentStyle::default(),
					};

					queue!(stdout, style::PrintStyledContent(style.apply(text)))?;
				}

				let (x, y) = self.coord_to_xy(*k);
				queue!(stdout, cursor::MoveTo(x, y))?;

				text = v.text.unwrap_or(' ').to_string();
				last_style = Some(v.style);
			} else {
				text = format!("{}{}", text, v.text.unwrap_or(' '));
			}

			last_coord = Some(*k);
		}

		if !text.is_empty() {
			let style = match self.styles.get(&last_style.unwrap()) {
				Some(s) => *s,
				None => ContentStyle::default(),
			};

			queue!(stdout, style::PrintStyledContent(style.apply(text)))?;
		}

		self.changes.clear();

		stdout.flush()?;

		Ok(())
	}

	fn coord_to_xy(&self, coord: usize) -> (u16, u16) {
		let y = (coord as f32 / self.width as f32).floor() as usize;
		let x = coord - (y * self.width as usize);
		(x as u16, y as u16)
	}

	fn xy_to_coord(&self, x: u16, y: u16) -> usize {
		(y * self.width + x) as usize
	}

	pub fn rect(&mut self, rect: Rect, text: char, style: Style) {
		let text: String = (0..rect.width).map(|_| text).collect();
		for y in 0..rect.height {
			self.string(rect.x, rect.y + y, text.clone(), style);
		}
	}

	pub fn string(&mut self, x: u16, y: u16, text: String, style: Style) {
		let coord = self.xy_to_coord(x, y);

		for (i, c) in text.chars().enumerate() {
			if i + coord >= self.pixels.len() {
				break;
			}

			self.pixel(
				coord + i,
				Pixel {
					text: Some(c),
					style,
				},
			);
		}
	}

	pub fn pixels(&mut self, x: u16, y: u16, pixels: &Pixels) {
		let coord = self.xy_to_coord(x, y);

		for (i, p) in pixels.0.iter().enumerate() {
			if i + coord >= self.pixels.len() {
				break;
			}

			self.pixel(coord + i, *p);
		}
	}

	pub fn pixel(&mut self, coord: usize, pixel: Pixel) {
		if self.pixels[coord] != pixel {
			self.changes.insert(coord, pixel);
		} else {
			self.changes.remove(&coord);
		}
	}
}
