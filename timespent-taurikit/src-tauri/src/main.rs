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

#[derive(Serialize, Deserialize)]
struct MyConfig {
    base_path: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            base_path: "../../timespent/tests/days".into(),
        }
    }
}

// use serde::{Deserialize, Serialize};
use std::sync::RwLock;

use timespent::{
    graph::ui::{Filter, Graph},
    graph::x_segments::ScaleXSegments,
    graph::y_activities::YActivities,
    loader,
};

pub struct StateContainer(pub RwLock<Graph>);

fn main() {
    let cfg: MyConfig = confy::load("timespent", Some("config")).unwrap();
    let directory = &cfg.base_path;
    println!("Loading data from {}", directory);

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
fn get_filter(state: tauri::State<StateContainer>) -> (ScaleXSegments, Filter, Filter) {
    let graph = state.0.read().unwrap();

    (
        graph.all_per_scale_x_segments.clone(),
        graph.all_filter.clone(),
        graph.applied_filter.clone(),
    )
}

#[tauri::command]
fn apply_filter(state: tauri::State<StateContainer>, filter: Filter) {
    let mut graph = state.0.write().unwrap();
    graph.apply_filter(&filter);
}
