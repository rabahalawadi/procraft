//! macOS Top Bar: changes the window top bar to macOS style with traffic light window controls.
//! When disabled or turned off, ProCraft uses the platform's default window decorations.

#![no_std]

/// Controls: an enabled switch that defaults to on (1.0).
static CONTROLS: kit::Controls<1> = kit::Controls::new([1.0]);
kit::controls!(CONTROLS);
const ENABLED: usize = 0;

kit::yes!(procraft_macos_top_bar, || CONTROLS.on(ENABLED));

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
