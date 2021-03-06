use chrono::NaiveDateTime;

use value::Value;

#[derive(Clone, Debug)]
pub struct RecordDataItem {
    pub value: Value,
    pub timestamp: NaiveDateTime,
}
