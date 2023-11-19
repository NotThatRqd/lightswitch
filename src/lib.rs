use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Mutex;
use subprocess::{Exec, Popen};
use btnify::ShutdownConfig;
use btnify::button::ButtonResponse;
use btnify::{button::Button, bind_server};

pub struct LightSwitchState {
    server_process: Popen,
    process_info: ProcessInfo,
}

impl LightSwitchState {
    fn new(process_info: ProcessInfo) -> LightSwitchState {
        let server_process = process_info.start_process();
        LightSwitchState {
            server_process,
            process_info, 
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    addr: SocketAddr,
    process_info: ProcessInfo,
}

#[derive(Deserialize)]
struct ProcessInfo {
    cmd: String,
    args: Vec<String>,
    cwd: String,
}

impl ProcessInfo {
    fn start_process(&self) -> Popen {
        println!("[LIGHTSWITCH] The process is being started...");

        Exec::cmd(&self.cmd)
            .args(&self.args)
            .cwd(&self.cwd)
            .popen()
            .unwrap()
    }
}

pub async fn start_lightswitch(config: Config) {
    let start = Button::create_button_with_state("start server", Box::new(start_click));
    let check = Button::create_button_with_state("check server", Box::new(check_click));

    let shutdown_config = ShutdownConfig::new(None, Some(Box::new(on_lightswitch_end)));

    let state = Mutex::new(LightSwitchState::new(config.process_info));

    bind_server(&config.addr, vec![start, check], state, Some(shutdown_config))
        .await
        .unwrap();
}

fn start_click(state: &Mutex<LightSwitchState>) -> ButtonResponse {
    let mut state = state.lock().unwrap(); 

    if is_running(&mut state.server_process) {
        "The server is already running.".into()
    } else {
        state.server_process = state.process_info.start_process();
        "The server has been started.".into()
    }
}

fn check_click(state: &Mutex<LightSwitchState>) -> ButtonResponse {
    let mut state = state.lock().unwrap(); 

    if is_running(&mut state.server_process) {
        "The server is running.".into()
    } else {
        "The server is not running".into()
    }
}

fn is_running(process: &mut Popen) -> bool {
    match process.poll() {
        None => true,
        Some(_) => false
    }
}

fn on_lightswitch_end(state: &Mutex<LightSwitchState>) {
    state.lock().unwrap().server_process.kill().unwrap();
}
