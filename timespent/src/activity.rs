use chrono::NaiveDateTime;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;
use ts_rs::TS;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum Action {
    Review,
    Meeting,
    Research,
    Code,
    Docs,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "review" => Ok(Action::Review),
            "meeting" => Ok(Action::Meeting),
            "research" => Ok(Action::Research),
            "code" => Ok(Action::Code),
            "docs" => Ok(Action::Docs),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, TS)]
pub enum Type {
    Action(Action),
    Project(String),
}

impl FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Type, Self::Err> {
        match Action::from_str(input) {
            Ok(a) => Ok(Type::Action(a)),
            _ => Ok(Type::Project(input.to_string())),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Activity {
    pub start_datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
    pub description: String,
    pub projects: HashSet<String>,
    pub actions: HashSet<Action>,
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Activities(pub Vec<Activity>);

impl From<Vec<Activity>> for Activities {
    fn from(activities: Vec<Activity>) -> Self {
        Activities(activities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::{Action, Type};

    #[test]
    fn test_action_from_str() {
        assert_eq!(Action::from_str("review").unwrap(), Action::Review);
        assert_eq!(Action::from_str("meeting").unwrap(), Action::Meeting);
        assert_eq!(Action::from_str("research").unwrap(), Action::Research);
        assert_eq!(Action::from_str("code").unwrap(), Action::Code);
        assert_eq!(Action::from_str("docs").unwrap(), Action::Docs);
        assert!(Action::from_str("other").is_err());
    }

    #[test]
    fn test_type_from_str() {
        assert_eq!(
            Type::from_str("review").unwrap(),
            Type::Action(Action::Review)
        );
        assert_eq!(
            Type::from_str("meeting").unwrap(),
            Type::Action(Action::Meeting)
        );
        assert_eq!(
            Type::from_str("research").unwrap(),
            Type::Action(Action::Research)
        );
        assert_eq!(Type::from_str("code").unwrap(), Type::Action(Action::Code));
        assert_eq!(Type::from_str("docs").unwrap(), Type::Action(Action::Docs));
        assert_eq!(
            Type::from_str("other").unwrap(),
            Type::Project("other".to_string())
        );
    }
}
