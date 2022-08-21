use timespent::loader;

#[test]
fn test_loader() {
    let activities = loader::load_from_filepath("tests/days").unwrap();
    assert_eq!(activities.0.len(), 16);
}

#[test]
fn test_aggregates() {
    let activities = loader::load_from_filepath("tests/days").unwrap();
    let agg = activities.aggregate_all();

    assert_eq!(agg.2.len(), 4);
}
