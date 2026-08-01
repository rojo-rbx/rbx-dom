use crate::basic_types::Enum;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TweenInfo {
    time: f32,
    easing_style: Enum,
    easing_direction: Enum,
    repeat_count: i32,
    reverses: bool,
    delay_time: f32,
}

impl TweenInfo {
    pub const fn new(
        time: f32,
        easing_style: Enum,
        easing_direction: Enum,
        repeat_count: i32,
        reverses: bool,
        delay_time: f32,
    ) -> Self {
        Self {
            time,
            easing_style,
            easing_direction,
            repeat_count,
            reverses,
            delay_time,
        }
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn set_time(&mut self, value: f32) {
        self.time = value;
    }

    pub fn easing_style(&self) -> Enum {
        self.easing_style
    }

    pub fn set_easing_style(&mut self, value: Enum) {
        self.easing_style = value;
    }

    pub fn easing_direction(&self) -> Enum {
        self.easing_direction
    }

    pub fn set_easing_direction(&mut self, value: Enum) {
        self.easing_direction = value;
    }

    pub fn repeat_count(&self) -> i32 {
        self.repeat_count
    }

    pub fn set_repeat_count(&mut self, value: i32) {
        self.repeat_count = value;
    }

    pub fn reverses(&self) -> bool {
        self.reverses
    }

    pub fn set_reverses(&mut self, value: bool) {
        self.reverses = value;
    }

    pub fn delay_time(&self) -> f32 {
        self.delay_time
    }

    pub fn set_delay_time(&mut self, value: f32) {
        self.delay_time = value;
    }
}
