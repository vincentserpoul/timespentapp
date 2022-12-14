use crate::activity::{Action, Activity};
use chrono::NaiveDateTime;
use nom::bytes::complete::{take, take_until};
use nom::character::complete::{char, multispace0};
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use std::collections::HashSet;

pub fn parse_time<'a>(date: &str, input: &'a str) -> IResult<&'a str, NaiveDateTime> {
    map_res(
        delimited(multispace0, take(5usize), multispace0),
        |s: &str| {
            NaiveDateTime::parse_from_str(
                format!("{} {}", date, s.trim()).as_str(),
                "%Y.%m.%d %Hh%M",
            )
        },
    )(input)
}

pub fn parse_tag(input: &str) -> IResult<&str, String> {
    let (input, tag) = delimited(
        preceded(multispace0, char('[')),
        take_until("]"),
        terminated(char(']'), multispace0),
    )(input)?;

    Ok((input, tag.trim().to_string()))
}

struct Types(HashSet<Action>, HashSet<String>);

fn parse_tags(input: &str) -> IResult<&str, Types> {
    let (input, tags) = many0(parse_tag)(input)?;

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

    Ok((input, Types(actions, projects)))
}

// parse_activity turn 12h00-13h00: [tag1][tag2][tag3] description into an activity
pub fn parse_activity<'a>(date: &str, input: &'a str) -> IResult<&'a str, Activity> {
    let (input, start_datetime) = parse_time(date, input)?;

    let (input, _) = delimited(multispace0, char('-'), multispace0)(input)?;

    let (input, end_datetime) = parse_time(date, input)?;

    let (input, _) = terminated(multispace0, char(':'))(input)?;

    let (input, types) = parse_tags(input)?;

    let description = input.trim();

    Ok((
        input,
        Activity {
            start_datetime,
            end_datetime,
            description: description.to_string(),
            action: types.0.into_iter().next().unwrap_or(Action::Unknown),
            projects: types.1,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::Action;
    use chrono::NaiveDate;

    #[test]
    fn test_parse_time() {
        let (input, time) = parse_time("2022.07.05", "12h00").unwrap();
        assert_eq!(input, "");
        assert_eq!(
            time,
            NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );

        let (input, time) = parse_time("2022.07.05", "12h00   ").unwrap();
        assert_eq!(input, "");
        assert_eq!(
            time,
            NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );

        let (input, time) = parse_time("2022.07.05", "  12h00   ").unwrap();
        assert_eq!(input, "");
        assert_eq!(
            time,
            NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn test_parse_tag() {
        let (input, tag) = parse_tag("[tag1]").unwrap();
        assert_eq!(input, "");
        assert_eq!(tag, "tag1");

        let (input, tag) = parse_tag("[tag-1]").unwrap();
        assert_eq!(input, "");
        assert_eq!(tag, "tag-1");

        let (input, tag) = parse_tag("[  tag2   ]").unwrap();
        assert_eq!(input, "");
        assert_eq!(tag, "tag2");

        let (input, tag) = parse_tag("  [  tag3   ]").unwrap();
        assert_eq!(input, "");
        assert_eq!(tag, "tag3");

        let (input, tag) = parse_tag("  [  tag4   ]  ").unwrap();
        assert_eq!(input, "");
        assert_eq!(tag, "tag4");
    }

    #[test]
    fn test_parse_tags() {
        let (input, tags) = parse_tags("[tag1]").unwrap();
        assert_eq!(input, "");
        assert_eq!(tags.0.len(), 0);
        assert_eq!(tags.1.len(), 1);
        assert!(tags.1.contains("tag1"));

        let (input, tags) = parse_tags("[tag1][tag2][review]").unwrap();
        assert_eq!(input, "");
        assert_eq!(tags.0.len(), 1);
        assert_eq!(tags.1.len(), 2);
        assert!(tags.1.contains("tag1"));
        assert!(tags.1.contains("tag2"));
        assert!(tags.0.contains(&Action::Review));

        let (input, tags) = parse_tags("[tag1][tag2][review]   ").unwrap();
        assert_eq!(input, "");
        assert_eq!(tags.0.len(), 1);
        assert_eq!(tags.1.len(), 2);
        assert!(tags.1.contains("tag1"));
        assert!(tags.1.contains("tag2"));
        assert!(tags.0.contains(&Action::Review));

        let (input, tags) = parse_tags("[tag1][tag2][review]   [tag3]").unwrap();
        assert_eq!(input, "");
        assert_eq!(tags.0.len(), 1);
        assert_eq!(tags.1.len(), 3);
        assert!(tags.1.contains("tag1"));
        assert!(tags.1.contains("tag2"));
        assert!(tags.1.contains("tag3"));
        assert!(tags.0.contains(&Action::Review));
    }

    #[test]
    fn test_parse_activity() {
        let target_act = Activity {
            start_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            description: "description".to_string(),
            action: Action::Review,
            projects: ["tag2".to_string(), "tag3".to_string()].into(),
        };
        let target_act_dash = Activity {
            start_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            description: "description".to_string(),
            action: Action::Code,
            projects: [
                "tag2".to_string(),
                "tag3".to_string(),
                "re-tash-yo".to_string(),
            ]
            .into(),
        };
        let target_act_spaces = Activity {
            start_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            description: "description of my tests".to_string(),
            action: Action::Review,
            projects: ["tag2".to_string(), "tag3".to_string()].into(),
        };
        let target_emptydesc = Activity {
            start_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            end_datetime: NaiveDate::from_ymd_opt(2022, 7, 5)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            description: "".to_string(),
            action: Action::Review,
            projects: ["tag2".to_string(), "tag3".to_string()].into(),
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
                "12h00-13h00: [re-tash-yo][tag2][tag3][code] description",
                &target_act_dash,
            ),
            (
                "spaces in desc",
                "12h00-13h00: [review][tag2][tag3] description of my tests ",
                &target_act_spaces,
            ),
            (
                "empty desc",
                "12h00-13h00: [review][tag2][tag3]",
                &target_emptydesc,
            ),
        ];

        for tc in test_cases {
            let act = parse_activity("2022.07.05", tc.1).unwrap().1;
            assert_eq!(&act, tc.2, "{} could not be parsed", tc.0);
        }
    }
}
