use bevy::prelude::Timer;

pub(crate) struct OncePerSecond {
    pub(crate) timer: Timer,
}

impl Default for OncePerSecond {
    fn default() -> Self {
        OncePerSecond {
            timer: Timer::from_seconds(1., true),
        }
    }
}
