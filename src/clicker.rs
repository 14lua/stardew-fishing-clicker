use enigo::{self, Enigo, MouseControllable};
use tokio::time::{sleep, Duration};
use device_query::{self, DeviceQuery, DeviceState, Keycode};

pub async fn listen_key() {
    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::F10) {
            trigger().await;
        }
    }
}

async fn trigger() {
    let mut enigo = Enigo::new();
    enigo.mouse_down(enigo::MouseButton::Left);
    sleep(Duration::from_millis(1030)).await;
    enigo.mouse_up(enigo::MouseButton::Left);
}