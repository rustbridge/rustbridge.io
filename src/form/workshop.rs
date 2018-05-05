use chrono::{NaiveDate, NaiveTime};

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

  pub fn date(&self) -> NaiveDate {
    self.date.parse::<NaiveDate>().unwrap()
  }

  pub fn start_time(&self) -> NaiveTime {
    self.start_time.parse::<NaiveTime>().unwrap()
  }

  pub fn end_time(&self) -> NaiveTime {
    self.end_time.parse::<NaiveTime>().unwrap()
  }

  pub fn private(&self) -> &bool {
    &self.private
  }
}
