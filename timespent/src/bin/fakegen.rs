use rand::Rng;
use std::io::Write;
use std::{collections::HashSet, fs::File};
use timespent::activity::{Action, Activities, Activity};

fn main() {
    let gen_path = "../fake";
    let projects = vec![
        "create-manes",
        "boring-admin",
        "rust",
        "manager",
        "management",
        "christmas-party",
        "create-manes2",
        "boring-admin2",
        "rust2",
        "manager2",
        "management2",
        "christmas-party2",
        "create-manes3",
        "boring-admin3",
        "rust3",
        "manager3",
        "management3",
        "christmas-party3",
        "create-manes4",
        "boring-admin4",
        "rust4",
        "manager4",
        "management4",
        "christmas-party4",
    ];

    let actions = vec![
        &Action::Code,
        &Action::Meeting,
        &Action::Review,
        &Action::Research,
        &Action::Unknown,
    ];

    let start_date = chrono::NaiveDate::from_ymd_opt(2018, 1, 1).unwrap();
    let end_date = chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

    start_date
        .iter_days()
        .take_while(|current_date| current_date <= &end_date)
        .for_each(|current_date| {
            let acts = gen_day(&current_date, &actions, &projects);
            if let Ok(mut fakef) = File::create(format!(
                "{}/{}.txt",
                gen_path,
                &current_date.format("%Y.%m.%d")
            )) {
                fakef.write_all(acts.to_string().as_bytes()).unwrap();
            } else {
                println!(
                    "Failed to create file for {}",
                    current_date.format("%Y.%m.%d")
                );
            };
        });
}

fn gen_day(day: &chrono::NaiveDate, actions: &[&Action], projects: &[&str]) -> Activities {
    let mut rng = rand::thread_rng();
    let mut activities = Vec::new();

    let start_time = chrono::NaiveTime::from_hms_opt(8, 30, 0).unwrap();
    let end_time = chrono::NaiveTime::from_hms_opt(19, 30, 0).unwrap();

    let mut curr_time = start_time;
    while curr_time <= end_time {
        let duration = rand::distributions::Uniform::new(0, 120);

        let rand_project = rand::distributions::Uniform::new(0, projects.len());
        let selected_project = [projects[rng.sample(rand_project)].to_string()];

        let rand_action = rand::distributions::Uniform::new(0, actions.len());
        let selected_action = actions[rng.sample(rand_action)];

        let duration = rng.sample(duration);
        let duration = chrono::Duration::minutes(duration);
        let activity = Activity {
            start_datetime: day.and_time(curr_time),
            end_datetime: day.and_time(curr_time + duration),
            description: "description".to_string(),
            action: *selected_action,
            projects: HashSet::from(selected_project),
        };

        activities.push(activity);

        curr_time += duration;
    }

    Activities(activities)
}
