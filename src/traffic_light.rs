#[derive(Debug)]
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

impl TrafficLightState {
    pub fn next(&self) -> TrafficLightState {
        match self {
            TrafficLightState::Red => TrafficLightState::Green,
            TrafficLightState::Yellow => TrafficLightState::Red,
            TrafficLightState::Green => TrafficLightState::Yellow,
        }
    }

    pub fn duration(&self) -> u64 {
        match self {
            TrafficLightState::Red => 6,
            TrafficLightState::Yellow => 1,
            TrafficLightState::Green => 5,
        }
    }
}
