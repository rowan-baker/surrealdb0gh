use crate::sql::fmt::Fmt;
use crate::sql::idiom::Idiom;
use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

#[revisioned(revision = 1)]
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub struct Groups(pub Vec<Group>);

impl Deref for Groups {
	type Target = Vec<Group>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl IntoIterator for Groups {
	type Item = Group;
	type IntoIter = std::vec::IntoIter<Self::Item>;
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl Display for Groups {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if self.0.is_empty() {
			write!(f, "GROUP ALL")
		} else {
			write!(f, "GROUP BY {}", Fmt::comma_separated(&self.0))
		}
	}
}

impl From<Groups> for crate::expr::Groups {
	fn from(v: Groups) -> Self {
		Self(v.0.into_iter().map(Into::into).collect())
	}
}

impl From<crate::expr::Groups> for Groups {
	fn from(v: crate::expr::Groups) -> Self {
		Self(v.0.into_iter().map(Into::into).collect())
	}
}

#[revisioned(revision = 1)]
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub struct Group(pub Idiom);

impl Deref for Group {
	type Target = Idiom;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Display for Group {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<Group> for crate::expr::Group {
	fn from(v: Group) -> Self {
		Self(v.0.into())
	}
}

impl From<crate::expr::Group> for Group {
	fn from(v: crate::expr::Group) -> Self {
		Self(v.0.into())
	}
}
