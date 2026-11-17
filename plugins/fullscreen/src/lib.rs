//! Fullscreen plugin for ProCraft: controls window fullscreen modes,
//! exclusive fullscreen (maximum monitor refresh rate), and F11 / Alt+Enter shortcut toggles.

#![no_std]

/// Controls:
/// 0: enabled (switch, default on)
/// 1: exclusive (switch, default on)
/// 2: shortcuts (switch, default on)
static CONTROLS: kit::Controls<3> = kit::Controls::new([1.0, 1.0, 1.0]);
kit::controls!(CONTROLS);

const ENABLED: usize = 0;
const EXCLUSIVE: usize = 1;
const SHORTCUTS: usize = 2;

kit::yes!(procraft_fullscreen, || CONTROLS.on(ENABLED));
kit::yes!(procraft_exclusive_fullscreen, || CONTROLS.on(ENABLED) && CONTROLS.on(EXCLUSIVE));
kit::yes!(procraft_fullscreen_shortcuts, || CONTROLS.on(ENABLED) && CONTROLS.on(SHORTCUTS));

#[unsafe(no_mangle)]
pub extern "C" fn procraft_action(_index: i32) -> i32 {
    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
