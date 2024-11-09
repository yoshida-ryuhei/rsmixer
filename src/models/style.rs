#[derive(Clone, Copy, PartialEq, Debug, Hash)]
#[derive(Default)]
pub enum Style {
	#[default]
 Normal,
	Muted,
	Bold,
	Inverted,
	Red,
	Green,
	Orange,
}
impl Eq for Style {}

impl From<&String> for Style {
	fn from(s: &String) -> Self {
		match &s[..] {
			"normal" => Style::Normal,
			"muted" => Style::Muted,
			"bold" => Style::Bold,
			"inverted" => Style::Inverted,
			"red" => Style::Red,
			"green" => Style::Green,
			"orange" => Style::Orange,
			_ => Style::Normal,
		}
	}
}

