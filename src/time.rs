#![allow(unused)]
use std::str::FromStr;

use anyhow::Result;
use clap::{ArgMatches, FromArgMatches, Parser};

#[derive(Clone, Debug)]
pub struct Time(pub u32);

impl FromStr for Time {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut t = s;
        let (mut hour, mut minute, mut seconds): (&str, &str, &str);

        (hour, t) = t.split_once('h').unwrap_or(("", t));
        (minute, t) = t.split_once('m').unwrap_or(("", t));
        seconds = if t.ends_with("s") { &t[..1] } else { t };

        let hour = if hour.len() == 0 {
            0
        } else {
            hour.parse::<u32>()?
        };

        let minute = if minute.len() == 0 {
            0
        } else {
            minute.parse::<u32>()?
        };

        let seconds = if seconds.len() == 0 {
            0
        } else {
            seconds.parse::<u32>()?
        };

        Ok(Self((hour * 3600) + (minute * 60) + seconds))
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::Time;
    use anyhow::Result;

    #[test]
    fn hms() -> Result<()> {
        assert_eq!(Time::from_str("1h20m4s")?.0, (60 * 60) + (20 * 60) + 4);
        assert_eq!(Time::from_str("2h2m0s")?.0, (60 * 60 * 2) + (2 * 60));
        assert_eq!(Time::from_str("1h0m5s")?.0, (60 * 60) + 5);
        assert_eq!(Time::from_str("0h5m5s")?.0, (60 * 5) + 5);
        assert_eq!(Time::from_str("0h0m0s")?.0, 0);
        Ok(())
    }

    #[test]
    fn hm() -> Result<()> {
        assert_eq!(Time::from_str("1h20m")?.0, (60 * 60) + (20 * 60));
        assert_eq!(Time::from_str("2h2m")?.0, (60 * 60 * 2) + (2 * 60));
        assert_eq!(Time::from_str("1h0m")?.0, 60 * 60);
        assert_eq!(Time::from_str("0h5m")?.0, 60 * 5);
        assert_eq!(Time::from_str("0h0m")?.0, 0);
        Ok(())
    }

    #[test]
    fn ms() -> Result<()> {
        assert_eq!(Time::from_str("20m4s")?.0, (20 * 60) + 4);
        assert_eq!(Time::from_str("2m0s")?.0, 2 * 60);
        assert_eq!(Time::from_str("0m5s")?.0, 5);
        assert_eq!(Time::from_str("5m5s")?.0, (60 * 5) + 5);
        assert_eq!(Time::from_str("0m0s")?.0, 0);
        Ok(())
    }

    #[test]
    fn hs() -> Result<()> {
        assert_eq!(Time::from_str("1h4s")?.0, (60 * 60) + 4);
        assert_eq!(Time::from_str("2h0s")?.0, 60 * 60 * 2);
        assert_eq!(Time::from_str("1h5s")?.0, (60 * 60) + 5);
        assert_eq!(Time::from_str("0h5s")?.0, 5);
        assert_eq!(Time::from_str("0h0s")?.0, 0);
        Ok(())
    }

    #[test]
    fn h() -> Result<()> {
        assert_eq!(Time::from_str("1h")?.0, 60 * 60);
        assert_eq!(Time::from_str("0h")?.0, 0);
        Ok(())
    }

    #[test]
    fn m() -> Result<()> {
        assert_eq!(Time::from_str("20m")?.0, 20 * 60);
        assert_eq!(Time::from_str("0m")?.0, 0);
        Ok(())
    }

    #[test]
    fn s() -> Result<()> {
        assert_eq!(Time::from_str("2s")?.0, 2);
        assert_eq!(Time::from_str("2")?.0, 2);
        assert_eq!(Time::from_str("0")?.0, 0);
        Ok(())
    }
}
