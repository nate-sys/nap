use std::str::FromStr;

#[derive(Clone)]
pub struct Progress(String, String, String);

impl FromStr for Progress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if utf8_slice::len(s) != 3 {
            anyhow::bail!("invalid progress format: {}", s)
        }

        Ok(Self(
            utf8_slice::slice(s, 0, 1).to_string(),
            utf8_slice::slice(s, 1, 2).to_string(),
            utf8_slice::slice(s, 2, 3).to_string(),
        ))
    }
}

impl ToString for Progress {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.0, self.1, self.2)
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self("━".to_owned(), "━".to_owned(), "━".to_owned())
    }
}
