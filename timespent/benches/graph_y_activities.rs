use chrono::NaiveDate;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use timespent::activity::{Action, Activities, Activity};
use timespent::graph::x_segments::ScaleXSegments;
use timespent::graph::y_activities::YActivities;

pub fn criterion_benchmark(c: &mut Criterion) {
    let activities = Activities(vec![
        Activity {
            start_datetime: NaiveDate::from_ymd(2015, 7, 22).and_hms(12, 0, 0),
            end_datetime: NaiveDate::from_ymd(2015, 7, 22).and_hms(13, 0, 0),
            description: "activity 1".to_string(),
            action: Action::Code,
            projects: ["tag1".to_string(), "tag2".to_string()].into(),
        },
        Activity {
            start_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(12, 0, 0),
            end_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(13, 0, 0),
            description: "activity 2".to_string(),
            action: Action::Review,
            projects: ["tag2".to_string(), "tag3".to_string()].into(),
        },
        Activity {
            start_datetime: NaiveDate::from_ymd(2022, 7, 26).and_hms(12, 0, 0),
            end_datetime: NaiveDate::from_ymd(2022, 7, 26).and_hms(13, 0, 0),
            description: "activity 3".to_string(),
            action: Action::Code,
            projects: ["tag1".to_string(), "tag2".to_string()].into(),
        },
        Activity {
            start_datetime: NaiveDate::from_ymd(2030, 7, 27).and_hms(12, 0, 0),
            end_datetime: NaiveDate::from_ymd(2030, 7, 27).and_hms(13, 0, 0),
            description: "activity 5".to_string(),
            action: Action::Review,
            projects: ["tag2".to_string(), "tag3".to_string()].into(),
        },
    ]);

    let agg_activities = activities.aggregate_all();

    let sxs = ScaleXSegments::new(&agg_activities.0, &agg_activities.1);

    c.bench_function("YActivities.new", |b| {
        b.iter(|| {
            YActivities::new(
                black_box(&activities),
                black_box(&agg_activities.2),
                black_box(&agg_activities.3),
                black_box(&sxs),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
