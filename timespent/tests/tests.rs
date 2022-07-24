use crate::aggregates::Aggregatable;
use chrono::NaiveDate;
use timespent::aggregates;
use timespent::loader;

#[test]
fn test_loader() {
    let activities = loader::load("tests/days").unwrap();
    assert_eq!(activities.0.len(), 16);
}

#[test]
fn test_aggregates() {
    let activities = loader::load("tests/days").unwrap();
    let agg = activities.aggregate();
    let agg_s = agg.over_specific_days(
        &NaiveDate::parse_from_str("2022-05-27", "%Y-%m-%d").unwrap(),
        &NaiveDate::parse_from_str("2022-05-31", "%Y-%m-%d").unwrap(),
    );

    assert_eq!(agg.daily.len(), 2);
    assert_eq!(agg_s.daily.len(), 5);

    assert_eq!(agg.daily[0].total_minutes, 450);
    assert_eq!(agg_s.daily[0].total_minutes, 450);

    assert_eq!(agg.daily[1].total_minutes, 450);
    assert_eq!(agg_s.daily[1].total_minutes, 0);

    assert_eq!(agg.all.total_minutes, 900);
    assert_eq!(agg_s.all.total_minutes, 900);
}
