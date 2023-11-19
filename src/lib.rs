use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Mutex;
use subprocess::{Exec, Popen};
use btnify::ShutdownConfig;
use btnify::button::ButtonResponse;
use btnify::{button::Button, bind_server};

pub struct LightSwitchState {
    password: Option<String>,
    lightswitch_process: Mutex<LightSwitchProcess>,
}

impl LightSwitchState {
    fn new(process_info: ProcessInfo, password: Option<String>) -> LightSwitchState {
        LightSwitchState {
            password,
            lightswitch_process: Mutex::new(LightSwitchProcess::new(process_info)),
        }
    }
}

struct LightSwitchProcess {
    process: Popen,
    process_info: ProcessInfo,
}

impl LightSwitchProcess {
    fn new(process_info: ProcessInfo) -> LightSwitchProcess {
        let process = process_info.start_process();
        LightSwitchProcess {
            process,
            process_info
        }
    }

    fn is_running(&mut self) -> bool {
        match self.process.poll() {
            None => true,
            Some(_) => false
        }
    }

    fn start(&mut self) {
        self.process = self.process_info.start_process()
    }

    fn kill(&mut self) {
        self.process.kill().unwrap();
    }
}

#[derive(Deserialize)]
pub struct Config {
    addr: SocketAddr,
    password: Option<String>,
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
    let start = if let Some(_) = &config.password {
        Button::create_button_with_state_and_prompts("start server", Box::new(start_click_with_pass), vec!["Enter password".to_string()])
    } else {
        Button::create_button_with_state("start server", Box::new(start_click_no_pass))
    };
    let check = Button::create_button_with_state("check server", Box::new(check_click));

    let shutdown_config = ShutdownConfig::new(None, Some(Box::new(on_lightswitch_end)));

    let state = LightSwitchState::new(config.process_info, config.password);

    bind_server(&config.addr, vec![start, check], state, Some(shutdown_config))
        .await
        .unwrap();
}

fn start_click_with_pass(state: &LightSwitchState, responses: Vec<Option<String>>) -> ButtonResponse {
    if let Some(guess) = &responses[0] {
        if guess != state.password.as_ref().unwrap() {
            return "Incorrect password.".into();
        }
    } else {
        return "You did not provide a password.".into();
    }

    start_click_no_pass(&state)
}

fn start_click_no_pass(state: &LightSwitchState) -> ButtonResponse {
    let mut lightswitch_process = state.lightswitch_process.lock().unwrap(); 

    if lightswitch_process.is_running() {
        "The server is already running.".into()
    } else {
        lightswitch_process.start();
        "The server has been started.".into()
    }
}

fn check_click(state: &LightSwitchState) -> ButtonResponse {
    let mut lightswitch_process = state.lightswitch_process.lock().unwrap(); 

    if lightswitch_process.is_running() {
        "The server is running.".into()
    } else {
        "The server is not running".into()
    }
}

fn on_lightswitch_end(state: &LightSwitchState) {
    state.lightswitch_process.lock().unwrap().kill();
}
