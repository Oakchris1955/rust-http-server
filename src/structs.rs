use std::fmt;

pub struct HttpVersion {
	major: usize,
	minor: usize
}

impl HttpVersion {
	pub fn new(version: &str) -> Option<Self> {
		if version.len() >= 5 {
			if &version[0..4] == "HTTP" && &version[4..5] == "/" {
				let version_split = &mut version[5..].split(".");
				if version_split.clone().count() == 2 {
					let parse_int = |option_input: Option<&str>| -> Option<usize> {
						let Some(string_num) = option_input else {
							return None;
						};

						let Ok(number) = string_num.parse::<usize>() else {
							return None;
						};

						Some(number)
					};

					let Some(major) = parse_int(version_split.next()) else {return None};
					let Some(minor) = parse_int(version_split.next()) else {return None};

					return Some(Self {
						major,
						minor
					})
				}
			}
		}

		None
	}
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP/{}.{}", self.major, self.minor)
    }
}
