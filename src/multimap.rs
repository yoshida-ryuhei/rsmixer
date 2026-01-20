use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::hash::Hash;

#[derive(Clone)]
enum Element<T> {
	Single(Vec<T>),
	Many(Vec<T>),
}

impl<T: Serialize> Serialize for Element<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			Self::Single(x) => x[0].serialize(serializer),
			Self::Many(xs) => xs.serialize(serializer),
		}
	}
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Element<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Use an untagged enum approach to try deserializing as T or Vec<T>.
		// Since we can't reuse the deserializer, we use a helper enum with #[serde(untagged)].
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum UntaggedElement<U> {
			Single(U),
			Many(Vec<U>),
		}

		let content = UntaggedElement::<T>::deserialize(deserializer)?;
		match content {
			UntaggedElement::Single(x) => Ok(Element::Single(vec![x])),
			UntaggedElement::Many(xs) => Ok(Element::Many(xs)),
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct MultiMap<K: Eq + Hash, V>(LinkedHashMap<K, Element<V>>);

impl<K: Eq + Hash, V> MultiMap<K, V> {
	pub fn new() -> Self {
		Self(LinkedHashMap::new())
	}

	pub fn insert(&mut self, k: K, v: V) {
		if let Some(e) = self.0.get_mut(&k) {
			match e {
				Element::Single(x) => {
					let old_v = x.remove(0);
					*e = Element::Many(vec![old_v, v]);
				}
				Element::Many(xs) => {
					xs.push(v);
				}
			};
		} else {
			self.0.insert(k, Element::Single(vec![v]));
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
		self.0.iter().flat_map(|(k, v)| {
			let vs = match v {
				Element::Single(x) => x,
				Element::Many(xs) => xs,
			};
			vs.iter().map(move |s| (k, s))
		})
	}

	pub fn iter_vecs(&self) -> impl Iterator<Item = (&K, &Vec<V>)> + '_ {
		self.0.iter().map(|(k, v)| match v {
			Element::Single(x) => (k, x),
			Element::Many(xs) => (k, xs),
		})
	}

	pub fn get_vec(&self, k: &K) -> Option<&Vec<V>> {
		match self.0.get(k) {
			Some(Element::Single(x)) => Some(x),
			Some(Element::Many(xs)) => Some(xs),
			None => None,
		}
	}
}
