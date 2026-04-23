/// Represents an 'easing style' for a TweenInfo. These correspond to values of the
/// [`EasingStyle`][EasingStyle-rbx] enum on Roblox.
///
/// Due to the nature of Roblox enums, this is marked as non-exhaustive as
/// Roblox may add more variants to it at any time.
///
/// [EasingStyle-rbx]: https://create.roblox.com/docs/reference/engine/enums/EasingStyle
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum EasingStyle {
    Linear,
    Sine,
    Back,
    #[default]
    Quad,
    Quart,
    Quint,
    Bounce,
    Elastic,
    Exponential,
    Circular,
    Cubic,
}

impl EasingStyle {
    /// Creates a `EasingStyle` from the provided value, if possible. Accepted
    /// values correspond to entries in the [`EasingStyle`][EasingStyle-rbx] enum
    /// on Roblox.
    ///
    /// [EasingStyle-rbx]: https://create.roblox.com/docs/reference/engine/enums/EasingStyle
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(match value {
            0 => EasingStyle::Linear,
            1 => EasingStyle::Sine,
            2 => EasingStyle::Back,
            3 => EasingStyle::Quad,
            4 => EasingStyle::Quart,
            5 => EasingStyle::Quint,
            6 => EasingStyle::Bounce,
            7 => EasingStyle::Elastic,
            8 => EasingStyle::Exponential,
            9 => EasingStyle::Circular,
            10 => EasingStyle::Cubic,
            _ => return None,
        })
    }

    /// Converts this vaue into a number identifier. Values are converted into
    /// their respective value of the Roblox [`EasingStyle`][EasingStyle-rbx]
    /// enum. As an example, `EasingStyle::Linear` is converted to `0`.
    ///
    /// [EasingStyle-rbx]: https://create.roblox.com/docs/reference/engine/enums/EasingStyle
    pub fn as_u8(self) -> u8 {
        match self {
            EasingStyle::Linear => 0,
            EasingStyle::Sine => 1,
            EasingStyle::Back => 2,
            EasingStyle::Quad => 3,
            EasingStyle::Quart => 4,
            EasingStyle::Quint => 5,
            EasingStyle::Bounce => 6,
            EasingStyle::Elastic => 7,
            EasingStyle::Exponential => 8,
            EasingStyle::Circular => 9,
            EasingStyle::Cubic => 10,
        }
    }
}

/// Represents an 'easing direction' for a TweenInfo. These correspond to values of the
/// [`EasingDirection`][EasingDirection-rbx] enum on Roblox.
///
/// Due to the nature of Roblox enums, this is marked as non-exhaustive as
/// Roblox may add more variants to it at any time.
///
/// [EasingDirection-rbx]: https://create.roblox.com/docs/reference/engine/enums/EasingDirection
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum EasingDirection {
    In,
    #[default]
    Out,
    InOut,
}

impl EasingDirection {
    /// Creates a `EasingDirection` from the provided value, if possible. Accepted
    /// values correspond to entries in the [`EasingDirection`][EasingDirection-rbx] enum
    /// on Roblox.
    ///
    /// [EasingDirection-rbx]: https://create.roblox.com/docs/reference/engine/enums/EasingDirection
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(match value {
            0 => EasingDirection::In,
            1 => EasingDirection::Out,
            2 => EasingDirection::InOut,
            _ => return None,
        })
    }

    /// Converts this vaue into a number identifier. Values are converted into
    /// their respective value of the Roblox [`EasingDirection`][EasingDirection-rbx]
    /// enum. As an example, `EasingDirection::In` is converted to `0`.
    ///
    /// [EasingDirection-rbx]: https://create.roblox.com/docs/reference/engine/enums/EasingDirection
    pub fn as_u8(self) -> u8 {
        match self {
            EasingDirection::In => 0,
            EasingDirection::Out => 1,
            EasingDirection::InOut => 2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct TweenInfo {
    pub time: f32,
    pub easing_style: EasingStyle,
    pub easing_direction: EasingDirection,
    pub repeat_count: i32,
    pub reverses: bool,
    pub delay_time: f32,
}

impl Default for TweenInfo {
    fn default() -> Self {
        Self {
            time: 1.,
            easing_style: Default::default(),
            easing_direction: Default::default(),
            repeat_count: Default::default(),
            reverses: Default::default(),
            delay_time: Default::default(),
        }
    }
}

impl TweenInfo {
    /// Constructs a new tween info.
    pub fn new(
        time: f32,
        easing_style: EasingStyle,
        easing_direction: EasingDirection,
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

    /// Constructs a new tween info from a time value.
    pub fn from_time(time: f32) -> Self {
        Self {
            time,
            ..Default::default()
        }
    }
}
