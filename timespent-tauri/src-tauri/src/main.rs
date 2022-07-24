#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::cargo
)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use timespent::{
    activity::Activities,
    aggregates::{Aggregatable, Aggregates},
    filter::{Filter, Filterable},
    loader,
};

use tauri::State;

struct StateContainer(Arc<Mutex<TimeSpentState>>);

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
struct TimeSpentState {
    activities: Activities,
    filter: Filter,
    filtered_activities: Activities,
    aggregates: Aggregates,
}

fn main() {
    let directory = "../../timespent/tests/days";
    let activities = loader::load(directory).unwrap();
    let filter = Filter::new(&activities);
    let filtered_activities = activities.clone();
    let activities_agg = activities.aggregate();
    let aggregates = activities_agg.over_specific_days(&filter.min_date, &filter.max_date);

    let state = Arc::new(Mutex::new(TimeSpentState {
        activities,
        filter,
        filtered_activities,
        aggregates,
    }));

    tauri::Builder::default()
        .manage(StateContainer(state))
        .invoke_handler(tauri::generate_handler![get_curr_state, apply_filter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn apply_filter(
    timespent_state: State<'_, StateContainer>,
    filter: Filter,
) -> (Filter, Aggregates) {
    let mut state = timespent_state
        .0
        .lock()
        .expect("connection not initialize; use the `connect` command first");

    state.filter = filter;
    state.filtered_activities = state.activities.filter(&state.filter);
    let activities_agg = state.filtered_activities.aggregate();
    state.aggregates =
        activities_agg.over_specific_days(&state.filter.min_date, &state.filter.max_date);

    (state.filter.clone(), state.aggregates.clone())
}

#[tauri::command]
fn get_curr_state(timespent_state: State<'_, StateContainer>) -> (Filter, Aggregates) {
    let state = timespent_state
        .0
        .lock()
        .expect("connection not initialize; use the `connect` command first");

    (state.filter.clone(), state.aggregates.clone())
}
