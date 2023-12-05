#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_state_transition() {
        let state = TrafficLightState::Red;
        assert_eq!(state.next(), TrafficLightState::Green);
        assert_eq!(state.next().next(), TrafficLightState::Yellow);
        assert_eq!(state.next().next().next(), TrafficLightState::Red);
    }

    #[test]
    fn test_traffic_light_state_duration() {
        assert_eq!(TrafficLightState::Red.duration(), 6);
        assert_eq!(TrafficLightState::Yellow.duration(), 1);
        assert_eq!(TrafficLightState::Green.duration(), 5);
    }
}
