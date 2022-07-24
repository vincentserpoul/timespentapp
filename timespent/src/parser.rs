use crate::activity::Action;
use crate::activity::Activity;
use chrono::NaiveDateTime;
use nom::bytes::streaming::{take, take_until};
use nom::character::complete::multispace0;
use nom::character::streaming::char;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, terminated};
use nom::IResult;
use std::collections::HashSet;

// parse_activity turn 12h00-13h00: [tag1][tag2][tag3] description into an activity
pub fn parse_activity<'a>(date: &str, input: &'a str) -> IResult<&'a str, Activity> {
    let (input, start_datetime) = map_res(
        delimited(multispace0, take(5usize), multispace0),
        |s: &str| {
            NaiveDateTime::parse_from_str(
                format!("{} {}", date, s.trim()).as_str(),
                "%Y.%m.%d %Hh%M",
            )
        },
    )(input)?;

    let (input, _) = delimited(multispace0, char('-'), multispace0)(input)?;

    let (input, end_datetime) = map_res(
        delimited(multispace0, take(5usize), multispace0),
        |s: &str| {
            NaiveDateTime::parse_from_str(
                format!("{} {}", date, s.trim()).as_str(),
                "%Y.%m.%d %Hh%M",
            )
        },
    )(input)?;
    let (input, _) = delimited(multispace0, char(':'), multispace0)(input)?;

    let (input, tags) = many0(delimited(
        delimited(multispace0, char('['), multispace0),
        take_until("]"),
        terminated(char(']'), multispace0),
    ))(input)?;

    let description = input.trim();

    let mut projects = HashSet::new();
    let mut actions = HashSet::new();
    for tag in tags {
        let tag = tag.trim();
        if let Ok(action) = tag.parse::<Action>() {
            actions.insert(action);
        } else {
            projects.insert(tag.to_string());
        }
    }

    Ok((
        input,
        Activity {
            start_datetime,
            end_datetime,
            description: description.to_string(),
            actions,
            projects,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::Action;

    #[test]
    fn test_parse_activity() {
        let target_act = Activity {
            start_datetime: NaiveDateTime::parse_from_str("2022-02-05 12:00", "%Y-%m-%d %H:%M")
                .unwrap(),
            end_datetime: NaiveDateTime::parse_from_str("2022-02-05 13:00", "%Y-%m-%d %H:%M")
                .unwrap(),
            description: "description".to_string(),
            projects: vec!["tag2".to_string(), "tag3".to_string()]
                .into_iter()
                .collect(),
            actions: vec![Action::Review].into_iter().collect(),
        };
        let target_act_dash = Activity {
            start_datetime: NaiveDateTime::parse_from_str("2022-02-05 12:00", "%Y-%m-%d %H:%M")
                .unwrap(),
            end_datetime: NaiveDateTime::parse_from_str("2022-02-05 13:00", "%Y-%m-%d %H:%M")
                .unwrap(),
            description: "description".to_string(),
            projects: vec![
                "re-tash-yo".to_string(),
                "tag2".to_string(),
                "tag3".to_string(),
            ]
            .into_iter()
            .collect(),
            actions: HashSet::new(),
        };
        let target_act_spaces = Activity {
            start_datetime: NaiveDateTime::parse_from_str("2022-02-05 12:00", "%Y-%m-%d %H:%M")
                .unwrap(),
            end_datetime: NaiveDateTime::parse_from_str("2022-02-05 13:00", "%Y-%m-%d %H:%M")
                .unwrap(),
            description: "description of my tests".to_string(),
            projects: vec!["tag2".to_string(), "tag3".to_string()]
                .into_iter()
                .collect(),
            actions: vec![Action::Review].into_iter().collect(),
        };

        let test_cases = vec![
            (
                "usual case",
                "12h00-13h00: [review][tag2][tag3] description",
                &target_act,
            ),
            (
                "space after time",
                "12h00-13h00 : [review][tag2][tag3] description",
                &target_act,
            ),
            (
                "two spaces after :",
                "12h00-13h00:  [review][tag2][tag3] description",
                &target_act,
            ),
            (
                "spaces between tags",
                "12h00-13h00: [review] [tag2]  [tag3] description",
                &target_act,
            ),
            (
                "spaces before description",
                "12h00-13h00: [review] [tag2]  [tag3]  description",
                &target_act,
            ),
            (
                "space in tag",
                "12h00-13h00: [review ][tag2][tag3] description",
                &target_act,
            ),
            (
                "dash in tag",
                "12h00-13h00: [re-tash-yo][tag2][tag3] description",
                &target_act_dash,
            ),
            (
                "spaces in desc",
                "12h00-13h00: [review][tag2][tag3] description of my tests ",
                &target_act_spaces,
            ),
        ];

        for tc in test_cases {
            let act = parse_activity("2022.02.05", tc.1).unwrap().1;
            assert_eq!(&act, tc.2, "{} could not be parsed", tc.0);
        }
    }
}
