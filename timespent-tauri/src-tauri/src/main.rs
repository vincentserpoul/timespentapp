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
    graph::ui::{Filter, Graph, ScaleXSegments, YActivities},
    loader, ActivitiesAggregate,
};

use tauri::State;

struct StateContainer(Arc<Mutex<Graph>>);

fn main() {
    let directory = "../../timespent/tests/days";
    let activities = loader::load_from_filepath(directory).unwrap();
    let graph = Graph::new(&activities);

    let state = Arc::new(Mutex::new(graph));

    tauri::Builder::default()
        .manage(StateContainer(state))
        .invoke_handler(tauri::generate_handler![apply_filter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_graph_at_scale(
    state: tauri::State<StateContainer>,
    scale: &Scale,
) -> (ScaleXSegments, YActivities) {
    let graph = state.0.lock().unwrap();

    (
        graph.filtered_per_scale_x_segments[scale],
        graph.filtered_per_scale_y_activities[scale],
    )
}

#[tauri::command]
fn get_filter(state: tauri::State<StateContainer>) -> (ActivitiesAggregate, Filter) {
    let graph = state.0.lock().unwrap();
    (graph.activities_aggregate, graph.applied_filter)
}

#[tauri::command]
fn apply_filter(graph: State<'_, StateContainer>, filter: Filter) {
    let mut graph = graph
        .0
        .lock()
        .expect("connection not initialize; use the `connect` command first");

    graph.apply_filter(&filter);
}
