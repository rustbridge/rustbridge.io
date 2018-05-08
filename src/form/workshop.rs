use chrono::NaiveDateTime;
use std::str::FromStr;

#[derive(FromForm)]
pub struct Workshop {
  name: String,
  description: String,
  location: String,
  // Format YYYY-MM-DD
  date: String,
  // Format HH:MM:SS
  start_time: String,
  // Format HH:MM:SS
  end_time: String,
  private: bool,
}

impl Workshop {
  pub fn name(&self) -> &str {
    &self.name[..]
  }

  pub fn description(&self) -> &str {
    &self.description[..]
  }

  pub fn location(&self) -> &str {
    &self.location[..]
  }

  pub fn date(&self) -> NaiveDateTime {
    let date_str = format!("{} 00:00:00", &self.date[..]);
    NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").unwrap()
  }

  pub fn start_time(&self) -> NaiveDateTime {
    let time_str = format!("1970-01-01 {}:00", &self.start_time[..]);
    NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S").unwrap()
  }

  pub fn end_time(&self) -> NaiveDateTime {
    let time_str = format!("1970-01-01 {}:00", &self.end_time[..]);
    NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S").unwrap()
  }

  pub fn private(&self) -> &bool {
    &self.private
  }
}
