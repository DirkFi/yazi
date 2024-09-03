use serde::{Deserialize, Serialize};

use anyhow::{bail, Error};
use std::convert::TryFrom;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct VisibilityRule {
	pub dir: String,
	pub hide: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(try_from = "Vec<VisibilityRule>")]
pub struct VisibilityRules {
	pub rules: Vec<VisibilityRule>,
}

impl TryFrom<Vec<VisibilityRule>> for VisibilityRules {
	type Error = Error;

	fn try_from(rules: Vec<VisibilityRule>) -> Result<Self, Self::Error> {
		if rules.is_empty() {
			bail!("Visibility rules cannot be empty");
		}

		// Add any additional validation if necessary, e.g., checking directory paths
		for rule in &rules {
			if rule.dir.is_empty() {
				bail!("Directory path cannot be empty in visibility rule");
			}

			// You could also validate the regex pattern here if desired
			if rule.hide.is_empty() {
				bail!("Hide pattern cannot be empty in visibility rule");
			}
		}

		Ok(Self { rules })
	}
}
