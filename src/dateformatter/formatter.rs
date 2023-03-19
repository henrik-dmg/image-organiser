use crate::dateformatter::strategy::DateGroupingStragegy;
use chrono::{DateTime, Utc};

pub struct DateFormatter {
    pub strategy: DateGroupingStragegy,
}

impl DateFormatter {
    pub fn make_folder_name(&self, datetime: DateTime<Utc>) -> String {
        // TODO: Use strategy here
        let year = datetime.format("%Y/%m").to_string();
        println!("{}", year);
        return year;
    }
}
