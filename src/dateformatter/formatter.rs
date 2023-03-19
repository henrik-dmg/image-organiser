use chrono::{DateTime, Utc};

pub trait DateFormatter {
    fn make_folder_name(&self, datetime: DateTime<Utc>) -> String;
}
