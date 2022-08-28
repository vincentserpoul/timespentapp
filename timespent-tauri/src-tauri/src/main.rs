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

// use serde::{Deserialize, Serialize};
use std::sync::RwLock;

use timespent::{
    activity::ActivitiesAggregate,
    graph::ui::{Filter, Graph},
    graph::x_segments::ScaleXSegments,
    graph::y_activities::YActivities,
    loader,
};

pub struct StateContainer(pub RwLock<Graph>);

fn main() {
    let directory = "../../timespent/tests/days";
    let activities = loader::load_from_filepath(directory).unwrap();
    let graph = Graph::new(&activities);

    tauri::Builder::default()
        .manage(StateContainer(RwLock::new(graph)))
        .invoke_handler(tauri::generate_handler![
            get_graph,
            get_filter,
            apply_filter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_graph(state: tauri::State<StateContainer>) -> (ScaleXSegments, YActivities) {
    let graph = state.0.read().unwrap();

    (
        graph.filtered_per_scale_x_segments.clone(),
        graph.filtered_per_scale_y_activities.clone(),
    )
}

#[tauri::command]
fn get_filter(state: tauri::State<StateContainer>) -> (ActivitiesAggregate, Filter) {
    let graph = state.0.read().unwrap();

    (
        graph.activities_aggregate.clone(),
        graph.applied_filter.clone(),
    )
}

#[tauri::command]
fn apply_filter(
    state: tauri::State<StateContainer>,
    filter: Filter,
) -> (ActivitiesAggregate, Filter) {
    let mut graph = state.0.write().unwrap();
    graph.apply_filter(&filter);

    (
        graph.activities_aggregate.clone(),
        graph.applied_filter.clone(),
    )
}
