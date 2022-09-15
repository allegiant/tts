use std::{thread, time::Duration};

use sapi_lite::tts::SyncSynthesizer;

use super::api_client as client;

//线程运行状态
static mut THREAD_STATUS: bool = false;

pub fn speak(text: &str) {
    sapi_lite::initialize().unwrap();
    let synth = SyncSynthesizer::new().unwrap();
    let _res = synth.set_volume(100);
    synth.speak(text, None).unwrap();
    sapi_lite::finalize();
}

pub fn loop_speak(url: String) {
    unsafe {
        let new_url = url.clone();
        THREAD_STATUS = true;
        thread::spawn(move || {
            while THREAD_STATUS {
                println!("task  run...");
                let content = client::get_data(&new_url);
                speak(&content);
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }
}

pub fn stop_loop_speak() {
    unsafe {
        THREAD_STATUS = false;
    }
}
