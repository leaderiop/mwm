#![warn(clippy::all)]
#![warn(future_incompatible)]
use penrose::{core::bindings::KeyEventHandler, x11rb::RustConn};

pub mod actions;
pub mod bar;
pub mod bindings;
pub mod layouts;

pub type KeyHandler = Box<dyn KeyEventHandler<RustConn>>;

pub const FONT: &str = "JetBrainsMono Nerd Font Mono";
pub const BLACK: u32 = 0x282828ff;
pub const WHITE: u32 = 0xebdbb2ff;
pub const GREY: u32 = 0x3c3836ff;
pub const BLUE: u32 = 0x458588ff;
pub const ORANGE: u32 = 0xd65d0eff;

pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.6;
pub const RATIO_STEP: f32 = 0.1;
pub const OUTER_PX: u32 = 5;
pub const INNER_PX: u32 = 5;
pub const BAR_HEIGHT_PX: u32 = 42;
pub const MAX_ACTIVE_WINDOW_CHARS: usize = 50;

pub const DEBUG_ENV_VAR: &str = "PENROSE_DEBUG";

pub const MON_1: &str = "eDP-1";
pub const MON_2: &str = "HDMI-2";
