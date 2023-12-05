use std::{thread, time};
mod traffic_light;
use traffic_light::TrafficLightState;

fn main() {
    let mut state = TrafficLightState::Red;

    loop {
        println!("Traffic Light is now {:?}", state);
        let duration = time::Duration::from_secs(state.duration());

        thread::sleep(duration);

        state = state.next();
    }
}
