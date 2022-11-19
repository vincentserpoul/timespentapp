use super::scale::Scale;
use chrono::{Datelike, Duration, Months, NaiveDate, NaiveDateTime, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone, TS, Hash)]
#[ts(export)]
pub struct XSegment {
    pub scale: Scale,
    pub start_datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
}

impl std::fmt::Display for XSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.scale {
            Scale::Day => write!(
                f,
                "{}-{:0>2}-{:0>2}",
                self.start_datetime.year(),
                self.start_datetime.month(),
                self.start_datetime.day()
            ),
            Scale::Week => write!(
                f,
                "{}-w{:0>2}",
                self.start_datetime.year(),
                self.start_datetime.iso_week().week()
            ),
            Scale::Month => write!(
                f,
                "{}-{:0>2}",
                self.start_datetime.year(),
                self.start_datetime.month()
            ),
            Scale::Year => write!(f, "{}", self.start_datetime.year()),
            Scale::All => write!(f, "all"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct ScaleXSegments {
    pub values: HashMap<Scale, Vec<XSegment>>,
    index: HashMap<Scale, HashMap<String, usize>>,
}

impl ScaleXSegments {
    pub fn new(start_date: &NaiveDate, end_date: &NaiveDate) -> ScaleXSegments {
        let start_datetime = start_date.and_hms_opt(0, 0, 0).unwrap();
        let end_datetime = (*end_date + Duration::days(1))
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let mut x_segments: HashMap<Scale, Vec<XSegment>> = [(
            Scale::All,
            vec![XSegment {
                scale: Scale::All,
                start_datetime,
                end_datetime,
            }],
        )]
        .into();

        // week
        let mut x_segments_day = Vec::new();
        let mut x_segments_week = Vec::new();
        let mut x_segments_month = Vec::new();
        let mut x_segments_year = Vec::new();

        start_date
            .iter_days()
            .take_while(|current_date| current_date <= end_date)
            .for_each(|current_date| {
                let curr_x_segments_day = XSegment {
                    scale: Scale::Day,
                    start_datetime: current_date.and_hms_opt(0, 0, 0).unwrap(),
                    end_datetime: (current_date + Duration::days(1))
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                };
                if x_segments_day.is_empty()
                    || x_segments_day[x_segments_day.len() - 1] != curr_x_segments_day
                {
                    x_segments_day.push(curr_x_segments_day);
                }

                let curr_x_segments_week = XSegment {
                    scale: Scale::Week,
                    start_datetime: NaiveDate::from_isoywd_opt(
                        current_date.iso_week().year(),
                        current_date.iso_week().week(),
                        Weekday::Mon,
                    )
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                    end_datetime: (NaiveDate::from_isoywd_opt(
                        current_date.iso_week().year(),
                        current_date.iso_week().week(),
                        Weekday::Sun,
                    )
                    .unwrap()
                        + Duration::days(1))
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                };
                if x_segments_week.is_empty()
                    || x_segments_week[x_segments_week.len() - 1] != curr_x_segments_week
                {
                    x_segments_week.push(curr_x_segments_week);
                }

                let curr_x_segments_month = XSegment {
                    scale: Scale::Month,
                    start_datetime: NaiveDate::from_ymd_opt(
                        current_date.year(),
                        current_date.month(),
                        1,
                    )
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                    end_datetime: (NaiveDate::from_ymd_opt(
                        current_date.year(),
                        current_date.month(),
                        1,
                    )
                    .unwrap()
                        + Months::new(1))
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                };
                if x_segments_month.is_empty()
                    || x_segments_month[x_segments_month.len() - 1] != curr_x_segments_month
                {
                    x_segments_month.push(curr_x_segments_month);
                }

                let curr_x_segments_year = XSegment {
                    scale: Scale::Year,
                    start_datetime: NaiveDate::from_ymd_opt(current_date.year(), 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(current_date.year() + 1, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                };
                if x_segments_year.is_empty()
                    || x_segments_year[x_segments_year.len() - 1] != curr_x_segments_year
                {
                    x_segments_year.push(curr_x_segments_year);
                }
            });

        x_segments.insert(Scale::Day, x_segments_day.into_iter().collect());
        x_segments.insert(Scale::Week, x_segments_week.into_iter().collect());
        x_segments.insert(Scale::Month, x_segments_month.into_iter().collect());
        x_segments.insert(Scale::Year, x_segments_year.into_iter().collect());

        let index = ScaleXSegments::build_index(&x_segments);
        ScaleXSegments {
            values: x_segments,
            index,
        }
    }

    fn build_index(
        x_segments: &HashMap<Scale, Vec<XSegment>>,
    ) -> HashMap<Scale, HashMap<String, usize>> {
        [
            (
                Scale::Day,
                x_segments
                    .get(&Scale::Day)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(u, x)| (x.start_datetime.date().to_string(), u))
                    .collect(),
            ),
            (
                Scale::Week,
                x_segments
                    .get(&Scale::Week)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(u, x)| {
                        (
                            x.start_datetime.year().to_string()
                                + &x.start_datetime.iso_week().week().to_string(),
                            u,
                        )
                    })
                    .collect(),
            ),
            (
                Scale::Month,
                x_segments
                    .get(&Scale::Month)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(u, x)| {
                        (
                            x.start_datetime.year().to_string()
                                + &x.start_datetime.month().to_string(),
                            u,
                        )
                    })
                    .collect(),
            ),
            (
                Scale::Year,
                x_segments
                    .get(&Scale::Year)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(u, x)| (x.start_datetime.year().to_string(), u))
                    .collect(),
            ),
        ]
        .into()
    }

    pub fn filter_by_date(&self, start_date: &NaiveDate, end_date: &NaiveDate) -> ScaleXSegments {
        let mut x_segments: HashMap<Scale, Vec<XSegment>> = HashMap::new();

        self.values.iter().for_each(|(scale, scale_x_segments)| {
            let mut filtered_x_segments = Vec::new();
            for xsegment in scale_x_segments {
                if xsegment.start_datetime.date() >= *start_date
                    && xsegment.end_datetime.date() < *end_date
                {
                    filtered_x_segments.push(xsegment.clone());
                }
            }
            x_segments.insert(*scale, filtered_x_segments);
        });

        let index = ScaleXSegments::build_index(&x_segments);

        ScaleXSegments {
            values: x_segments,
            index,
        }
    }

    pub fn find_correponding_x_segment_idx(
        &self,
        scale: &Scale,
        datetime: &NaiveDateTime,
    ) -> usize {
        match scale {
            Scale::Day => {
                let index = datetime.date().to_string();
                let x_segment_idx = self.index.get(&Scale::Day).unwrap().get(&index);
                match x_segment_idx {
                    Some(idx) => *idx,
                    None => 0,
                }
            }
            Scale::Week => {
                let index = datetime.year().to_string() + &datetime.iso_week().week().to_string();
                let x_segment_idx = self.index.get(&Scale::Week).unwrap().get(&index);
                match x_segment_idx {
                    Some(idx) => *idx,
                    None => 0,
                }
            }
            Scale::Month => {
                let index = datetime.year().to_string() + &datetime.month().to_string();
                let x_segment_idx = self.index.get(&Scale::Month).unwrap().get(&index);
                match x_segment_idx {
                    Some(idx) => *idx,
                    None => 0,
                }
            }
            Scale::Year => {
                let year = datetime.year();
                let x_segment_idx = self.index.get(&Scale::Year).unwrap().get(&year.to_string());
                match x_segment_idx {
                    Some(idx) => *idx,
                    None => 0,
                }
            }
            Scale::All => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xsegment_display() {
        let x_segments = [
            XSegment {
                start_datetime: NaiveDate::from_ymd_opt(2022, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 3, 15)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                scale: Scale::Day,
            },
            XSegment {
                start_datetime: NaiveDate::from_ymd_opt(2022, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 3, 15)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                scale: Scale::Week,
            },
            XSegment {
                start_datetime: NaiveDate::from_ymd_opt(2022, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 3, 15)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                scale: Scale::Month,
            },
            XSegment {
                start_datetime: NaiveDate::from_ymd_opt(2022, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 3, 15)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                scale: Scale::Year,
            },
            XSegment {
                start_datetime: NaiveDate::from_ymd_opt(2022, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 3, 15)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
                scale: Scale::All,
            },
        ];

        assert_eq!(
            format!("{}", x_segments[0]),
            "2022-03-14",
            "wrong format for scale day"
        );
        assert_eq!(
            format!("{}", x_segments[1]),
            "2022-w11",
            "wrong format for scale week"
        );
        assert_eq!(
            format!("{}", x_segments[2]),
            "2022-03",
            "wrong format for scale month"
        );
        assert_eq!(
            format!("{}", x_segments[3]),
            "2022",
            "wrong format for scale year"
        );
        assert_eq!(
            format!("{}", x_segments[4]),
            "all",
            "wrong format for scale all"
        );
    }

    #[test]
    fn test_xscalesegments_new() {
        let sxs = ScaleXSegments::new(
            &NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
            &NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
        );

        assert_eq!(sxs.values.len(), 5, "wrong number of scales");

        assert_eq!(
            sxs.values.get(&Scale::Day).unwrap(),
            &vec![
                XSegment {
                    scale: Scale::Day,
                    start_datetime: NaiveDate::from_ymd_opt(2022, 12, 31)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
                XSegment {
                    scale: Scale::Day,
                    start_datetime: NaiveDate::from_ymd_opt(2023, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 1, 2)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
                XSegment {
                    scale: Scale::Day,
                    start_datetime: NaiveDate::from_ymd_opt(2023, 1, 2)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 1, 3)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
            ],
            "wrong x_segments for scale day"
        );

        assert_eq!(
            sxs.values.get(&Scale::Week).unwrap(),
            &vec![
                XSegment {
                    scale: Scale::Week,
                    start_datetime: NaiveDate::from_ymd_opt(2022, 12, 26)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 1, 2)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
                XSegment {
                    scale: Scale::Week,
                    start_datetime: NaiveDate::from_ymd_opt(2023, 1, 2)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 1, 9)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
            ],
            "wrong x_segments for scale week"
        );

        assert_eq!(
            sxs.values.get(&Scale::Month).unwrap(),
            &vec![
                XSegment {
                    scale: Scale::Month,
                    start_datetime: NaiveDate::from_ymd_opt(2022, 12, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
                XSegment {
                    scale: Scale::Month,
                    start_datetime: NaiveDate::from_ymd_opt(2023, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 2, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
            ],
            "wrong x_segments for scale month"
        );

        assert_eq!(
            sxs.values.get(&Scale::Year).unwrap(),
            &vec![
                XSegment {
                    scale: Scale::Year,
                    start_datetime: NaiveDate::from_ymd_opt(2022, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2023, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
                XSegment {
                    scale: Scale::Year,
                    start_datetime: NaiveDate::from_ymd_opt(2023, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    end_datetime: NaiveDate::from_ymd_opt(2024, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                },
            ],
            "wrong x_segments for scale year"
        );

        assert_eq!(
            sxs.values.get(&Scale::All).unwrap(),
            &vec![XSegment {
                scale: Scale::All,
                start_datetime: NaiveDate::from_ymd_opt(2022, 12, 31)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2023, 1, 3)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            },],
            "wrong x_segments for scale all"
        );
    }
}
