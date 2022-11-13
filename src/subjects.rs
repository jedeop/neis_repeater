use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};

use crate::neis::time_table::TimeTableData;

#[derive(Debug, serde::Serialize)]
pub(crate) struct Subject {
    subject: String,
    time: Vec<(String, u8)>,
    classroom: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Subjects(HashMap<String, Subject>);

impl Subjects {
    pub(crate) fn from_time_table(time_tables: &[TimeTableData]) -> Subjects {
        let mut subjects: HashMap<String, Subject> = HashMap::new();
        for time_table in time_tables {
            let date = NaiveDate::parse_from_str(&time_table.date, "%Y%m%d").unwrap();
            let time = (
                date.weekday().to_string(),
                time_table.perio.parse().unwrap(),
            );
            subjects
                .entry(format!("{}({})", time_table.subject, time_table.classroom))
                .and_modify(|s| s.time.push(time.clone()))
                .or_insert(Subject {
                    subject: time_table.subject.clone(),
                    time: vec![time],
                    classroom: time_table.classroom.clone(),
                });
        }
        Subjects(subjects)
    }
}
