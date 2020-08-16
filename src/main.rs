use pathsearch::find_executable_in_path;
use std::fs;
use eventual::{Timer};
use windows_win::raw::window;
use windows_win::winapi::um::winuser::{SW_HIDE};


#[derive(serde::Deserialize)]
pub struct Config {
    window_class_name: String
}

fn hide_window(window_class_name: &String) {
    let search_result = window::get_by_class(window_class_name, None).unwrap();

    for window_hwnd in search_result {
        println!("hiding handle {:?}", window_hwnd);
        window::show(window_hwnd, SW_HIDE);
    }
}

fn main() {
    let config_path = find_executable_in_path("window-auto-hide.toml").expect("Config file not found");
    let config_bytes = fs::read(config_path).expect("Error reading config file");
    let config: Config = toml::from_slice(&config_bytes).expect("Error parsing config file");

    let interval = 1000;

    let timer = Timer::new();
    let ticks = timer.interval_ms(interval).iter();

    hide_window(&config.window_class_name);
    for _ in ticks {
        hide_window(&config.window_class_name);
    }
}
