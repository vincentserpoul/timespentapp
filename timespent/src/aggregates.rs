use crate::activity::{Action, Activities};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Aggregates {
    pub daily: Vec<DailyAggregate>,
    pub all: AllAggregate,
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct DailyAggregate {
    pub day: NaiveDate,
    pub total_minutes: i64,
    pub project_minutes: HashMap<String, i64>,
    pub action_minutes: HashMap<Action, i64>,
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct AllAggregate {
    pub total_minutes: i64,
    pub project_minutes: HashMap<String, i64>,
    pub action_minutes: HashMap<Action, i64>,
}

impl Aggregates {
    pub fn over_specific_days(&self, start_date: &NaiveDate, end_date: &NaiveDate) -> Aggregates {
        if end_date < start_date {
            return Aggregates {
                daily: vec![],
                all: AllAggregate {
                    total_minutes: 0,
                    project_minutes: HashMap::new(),
                    action_minutes: HashMap::new(),
                },
            };
        }

        let days_count = 1 + (*end_date - *start_date).num_days() as usize;
        let mut daily_res = Vec::with_capacity(days_count);
        let mut current_date = *start_date;
        let mut curr_day_agg_idx = 0;

        while current_date <= *end_date {
            if curr_day_agg_idx < self.daily.len()
                && self.daily[curr_day_agg_idx].day == current_date
            {
                daily_res.push(self.daily[curr_day_agg_idx].clone());
                curr_day_agg_idx += 1;
            } else {
                daily_res.push(DailyAggregate {
                    day: current_date,
                    total_minutes: 0,
                    project_minutes: HashMap::new(),
                    action_minutes: HashMap::new(),
                });
            }

            current_date = current_date.succ();
        }

        Aggregates {
            daily: daily_res,
            all: self.all.clone(),
        }
    }
}

pub trait Aggregatable {
    fn aggregate(&self) -> Aggregates;
}

impl Aggregatable for Activities {
    fn aggregate(&self) -> Aggregates {
        let (daily, period) = self.0.iter().fold(
            (
                HashMap::new(),
                AllAggregate {
                    total_minutes: 0,
                    project_minutes: HashMap::new(),
                    action_minutes: HashMap::new(),
                },
            ),
            |(mut days_hm_agg, mut all_agg), a| {
                let days_agg =
                    days_hm_agg
                        .entry(a.start_datetime.date())
                        .or_insert(DailyAggregate {
                            day: a.start_datetime.date(),
                            total_minutes: 0,
                            project_minutes: HashMap::new(),
                            action_minutes: HashMap::new(),
                        });

                let duration = a
                    .end_datetime
                    .signed_duration_since(a.start_datetime)
                    .num_minutes();

                days_agg.total_minutes += duration;
                all_agg.total_minutes += duration;

                for prj in &a.projects {
                    let proj_min = days_agg.project_minutes.entry(prj.to_string()).or_insert(0);
                    *proj_min += duration;

                    let proj_min = all_agg.project_minutes.entry(prj.to_string()).or_insert(0);
                    *proj_min += duration;
                }

                for act in &a.actions {
                    let act_min = days_agg.action_minutes.entry(*act).or_insert(0);
                    *act_min += duration;

                    let act_min = all_agg.action_minutes.entry(*act).or_insert(0);
                    *act_min += duration;
                }

                (days_hm_agg, all_agg)
            },
        );

        let mut final_agg = Aggregates {
            daily: daily.into_iter().map(|(_, v)| v).collect(),
            all: period,
        };

        final_agg.daily.sort_by(|a, b| a.day.cmp(&b.day));

        final_agg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::{Action, Activity};
    use chrono::NaiveDateTime;

    #[test]
    fn test_new_aggregate() {
        let activities = Activities(vec![
            Activity {
                start_datetime: NaiveDateTime::parse_from_str("2022-07-22 12:00", "%Y-%m-%d %H:%M")
                    .unwrap(),
                end_datetime: NaiveDateTime::parse_from_str("2022-07-22 13:00", "%Y-%m-%d %H:%M")
                    .unwrap(),
                description: "description".to_string(),
                projects: vec!["tag2".to_string(), "tag3".to_string()]
                    .into_iter()
                    .collect(),
                actions: vec![Action::Code].into_iter().collect(),
            },
            Activity {
                start_datetime: NaiveDateTime::parse_from_str("2022-07-25 12:00", "%Y-%m-%d %H:%M")
                    .unwrap(),
                end_datetime: NaiveDateTime::parse_from_str("2022-07-25 13:01", "%Y-%m-%d %H:%M")
                    .unwrap(),
                description: "description".to_string(),
                projects: vec!["tag2".to_string(), "tag3".to_string()]
                    .into_iter()
                    .collect(),
                actions: vec![Action::Review].into_iter().collect(),
            },
        ]);

        let tcs = vec![(
            "total",
            NaiveDate::parse_from_str("2022-07-22", "%Y-%m-%d").unwrap(),
            NaiveDate::parse_from_str("2022-07-25", "%Y-%m-%d").unwrap(),
            Aggregates {
                daily: vec![
                    DailyAggregate {
                        day: NaiveDate::parse_from_str("2022-07-22", "%Y-%m-%d").unwrap(),
                        total_minutes: 60,
                        project_minutes: vec![("tag2".to_string(), 60), ("tag3".to_string(), 60)]
                            .into_iter()
                            .collect(),
                        action_minutes: vec![(Action::Code, 60)].into_iter().collect(),
                    },
                    DailyAggregate {
                        day: NaiveDate::parse_from_str("2022-07-25", "%Y-%m-%d").unwrap(),
                        total_minutes: 61,
                        project_minutes: vec![("tag2".to_string(), 61), ("tag3".to_string(), 61)]
                            .into_iter()
                            .collect(),
                        action_minutes: vec![(Action::Review, 61)].into_iter().collect(),
                    },
                ],
                all: AllAggregate {
                    total_minutes: 121,
                    project_minutes: vec![("tag2".to_string(), 121), ("tag3".to_string(), 121)]
                        .into_iter()
                        .collect(),
                    action_minutes: vec![(Action::Code, 60), (Action::Review, 61)]
                        .into_iter()
                        .collect(),
                },
            },
            Aggregates {
                daily: vec![
                    DailyAggregate {
                        day: NaiveDate::parse_from_str("2022-07-22", "%Y-%m-%d").unwrap(),
                        total_minutes: 60,
                        project_minutes: vec![("tag2".to_string(), 60), ("tag3".to_string(), 60)]
                            .into_iter()
                            .collect(),
                        action_minutes: vec![(Action::Code, 60)].into_iter().collect(),
                    },
                    DailyAggregate {
                        day: NaiveDate::parse_from_str("2022-07-23", "%Y-%m-%d").unwrap(),
                        total_minutes: 0,
                        project_minutes: HashMap::new(),
                        action_minutes: HashMap::new(),
                    },
                    DailyAggregate {
                        day: NaiveDate::parse_from_str("2022-07-24", "%Y-%m-%d").unwrap(),
                        total_minutes: 0,
                        project_minutes: HashMap::new(),
                        action_minutes: HashMap::new(),
                    },
                    DailyAggregate {
                        day: NaiveDate::parse_from_str("2022-07-25", "%Y-%m-%d").unwrap(),
                        total_minutes: 61,
                        project_minutes: vec![("tag2".to_string(), 61), ("tag3".to_string(), 61)]
                            .into_iter()
                            .collect(),
                        action_minutes: vec![(Action::Review, 61)].into_iter().collect(),
                    },
                ],
                all: AllAggregate {
                    total_minutes: 121,
                    project_minutes: vec![("tag2".to_string(), 121), ("tag3".to_string(), 121)]
                        .into_iter()
                        .collect(),
                    action_minutes: vec![(Action::Code, 60), (Action::Review, 61)]
                        .into_iter()
                        .collect(),
                },
            },
        )];

        for tc in tcs {
            let agg = activities.aggregate();
            assert_eq!(agg, tc.3);

            let agg_s = agg.over_specific_days(&tc.1, &tc.2);
            assert_eq!(agg_s, tc.4);
        }
    }
}
