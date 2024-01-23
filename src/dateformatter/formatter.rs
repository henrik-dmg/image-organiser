use crate::dateformatter::strategy::DateGroupingStrategy;
use chrono::{DateTime, Utc};

pub struct DateFormatter {
    pub strategy: DateGroupingStrategy,
}

impl DateFormatter {
    pub fn make_folder_name(&self, datetime: DateTime<Utc>) -> String {
        let date_format = match self.strategy {
            DateGroupingStrategy::Year => "%Y",
            DateGroupingStrategy::Month => "%Y/%m",
            DateGroupingStrategy::Week => "%Y/%W",
        };
        return datetime.format(&date_format).to_string();
    }
}
