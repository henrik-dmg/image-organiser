use crate::dateformatter::strategy::DateGroupingStragegy;
use chrono::{DateTime, Utc};

pub struct DateFormatter {
    pub strategy: DateGroupingStragegy,
}

impl DateFormatter {
    pub fn make_folder_name(&self, datetime: DateTime<Utc>) -> String {
        let date_format = match self.strategy {
            DateGroupingStragegy::Year => "%Y",
            DateGroupingStragegy::Month => "%Y/%m",
            DateGroupingStragegy::Week => "%Y/%m/%W",
        };
        return datetime.format(&date_format).to_string();
    }
}
