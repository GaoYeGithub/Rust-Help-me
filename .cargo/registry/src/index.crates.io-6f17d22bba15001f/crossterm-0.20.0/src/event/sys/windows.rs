//! This is a WINDOWS specific implementation for input related action.

use std::convert::TryFrom;
use std::sync::atomic::{AtomicU64, Ordering};

use crossterm_winapi::{ConsoleMode, Handle};

use crate::Result;

#[cfg(feature = "event-stream")]
pub(crate) mod waker;

pub(crate) mod parse;
pub(crate) mod poll;

const ENABLE_MOUSE_MODE: u32 = 0x0010 | 0x0080 | 0x0008;

/// This is a either `u64::MAX` if it's uninitialized or a valid `u32` that stores the original
/// console mode if it's initialized.
static ORIGINAL_CONSOLE_MODE: AtomicU64 = AtomicU64::new(u64::MAX);

/// Initializes the default console color. It will will be skipped if it has already been initialized.
fn init_original_console_mode(original_mode: u32) {
    let _ = ORIGINAL_CONSOLE_MODE.compare_exchange(
        u64::MAX,
        u64::from(original_mode),
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
}

/// Returns the original console color, make sure to call `init_console_color` before calling this function. Otherwise this function will panic.
fn original_console_mode() -> u32 {
    u32::try_from(ORIGINAL_CONSOLE_MODE.load(Ordering::Relaxed))
        // safe unwrap, initial console color was set with `init_console_color` in `WinApiColor::new()`
        .expect("Original console mode not set")
}

pub(crate) fn enable_mouse_capture() -> Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    init_original_console_mode(mode.mode()?);
    mode.set_mode(ENABLE_MOUSE_MODE)?;

    Ok(())
}

pub(crate) fn disable_mouse_capture() -> Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    mode.set_mode(original_console_mode())?;
    Ok(())
}
