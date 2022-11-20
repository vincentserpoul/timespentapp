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
use serde_with::serde_as;
use serde_with::DurationSeconds;

#[serde_as]
#[derive(Serialize, Deserialize)]
struct MyConfig {
    base_path: String,
    #[serde_as(as = "DurationSeconds<i64>")]
    start_ago: chrono::Duration,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            base_path: "../../timespent/tests/days".into(),
            start_ago: chrono::Duration::days(14),
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

const CONFIG_NAME: &str = "config";
const APP_NAME: &str = "timespent";

fn main() {
    println!(
        "using config path {:?}",
        confy::get_configuration_file_path(APP_NAME, CONFIG_NAME).unwrap(),
    );
    let cfg: MyConfig = confy::load(APP_NAME, Some(CONFIG_NAME)).unwrap();
    let directory = &cfg.base_path;
    println!("Loading data from {}", directory);

    let activities = loader::load_from_filepath(directory).expect("Failed to load data");
    let graph = Graph::new(&activities);

    let state = StateContainer(RwLock::new(graph));

    tauri::Builder::default()
        .manage(state)
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
