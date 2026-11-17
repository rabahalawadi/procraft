//! macOS-style traffic lights (3 window control dots: Close, Minimize, Maximize)
//! and window action dispatching for custom top bars.

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use egui::{pos2, vec2, Color32, Rect, Sense, Stroke, Ui};

static MACOS_TOPBAR: AtomicBool = AtomicBool::new(false);

/// Sets whether the macOS style top bar is active.
pub fn set_macos_topbar(enabled: bool) {
    MACOS_TOPBAR.store(enabled, Ordering::Relaxed);
}

/// Returns whether the macOS style top bar is active.
pub fn is_macos_topbar() -> bool {
    MACOS_TOPBAR.load(Ordering::Relaxed)
}

/// The pending window action requested by the user interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowControlAction {
    Close,
    Minimize,
    Maximize,
    Drag,
    ToggleFullscreen,
}

static PENDING_ACTION: AtomicU8 = AtomicU8::new(0);
const ACT_NONE: u8 = 0;
const ACT_CLOSE: u8 = 1;
const ACT_MINIMIZE: u8 = 2;
const ACT_MAXIMIZE: u8 = 3;
const ACT_DRAG: u8 = 4;
const ACT_TOGGLE_FULLSCREEN: u8 = 5;

/// Retrieves and clears any pending window action.
pub fn take_pending_action() -> Option<WindowControlAction> {
    match PENDING_ACTION.swap(ACT_NONE, Ordering::Relaxed) {
        ACT_CLOSE => Some(WindowControlAction::Close),
        ACT_MINIMIZE => Some(WindowControlAction::Minimize),
        ACT_MAXIMIZE => Some(WindowControlAction::Maximize),
        ACT_DRAG => Some(WindowControlAction::Drag),
        ACT_TOGGLE_FULLSCREEN => Some(WindowControlAction::ToggleFullscreen),
        _ => None,
    }
}

pub fn request_close() {
    PENDING_ACTION.store(ACT_CLOSE, Ordering::Relaxed);
}

pub fn request_minimize() {
    PENDING_ACTION.store(ACT_MINIMIZE, Ordering::Relaxed);
}

pub fn request_maximize() {
    PENDING_ACTION.store(ACT_MAXIMIZE, Ordering::Relaxed);
}

pub fn request_drag() {
    PENDING_ACTION.store(ACT_DRAG, Ordering::Relaxed);
}

pub fn request_toggle_fullscreen() {
    PENDING_ACTION.store(ACT_TOGGLE_FULLSCREEN, Ordering::Relaxed);
}

pub const DOT_RADIUS: f32 = 6.0;
pub const DOT_DIAMETER: f32 = 12.0;
pub const DOT_SPACING: f32 = 8.0;
pub const TRAFFIC_LIGHTS_WIDTH: f32 = DOT_DIAMETER * 3.0 + DOT_SPACING * 2.0; // 52.0

/// Draws the macOS 3 traffic light dots centered at `center_y` starting at `left`.
/// Returns true if the traffic light cluster or any dot was hovered or interacted with.
pub fn traffic_lights(ui: &Ui, left: f32, center_y: f32) -> bool {
    let cluster_rect = Rect::from_min_size(pos2(left, center_y - 8.0), vec2(TRAFFIC_LIGHTS_WIDTH, 16.0));
    let cluster_hovered = ui.input(|i| {
        i.pointer.hover_pos().is_some_and(|p| cluster_rect.expand(3.0).contains(p))
    });

    let red_center = pos2(left + 6.0, center_y);
    let yellow_center = pos2(left + 26.0, center_y);
    let green_center = pos2(left + 46.0, center_y);

    let red_rect = Rect::from_center_size(red_center, vec2(14.0, 14.0));
    let yellow_rect = Rect::from_center_size(yellow_center, vec2(14.0, 14.0));
    let green_rect = Rect::from_center_size(green_center, vec2(14.0, 14.0));

    let red_res = ui.interact(red_rect, ui.scope_id().with("mac_dot_red"), Sense::click());
    let yellow_res = ui.interact(yellow_rect, ui.scope_id().with("mac_dot_yellow"), Sense::click());
    let green_res = ui.interact(green_rect, ui.scope_id().with("mac_dot_green"), Sense::click());

    if red_res.clicked() {
        request_close();
    }
    if yellow_res.clicked() {
        request_minimize();
    }
    if green_res.clicked() {
        request_maximize();
    }

    let painter = ui.painter();

    // Red (Close): #FF5F57
    let red_fill = if red_res.is_pointer_button_down_on() {
        Color32::from_rgb(190, 60, 55)
    } else {
        Color32::from_rgb(255, 95, 87)
    };
    painter.circle_filled(red_center, DOT_RADIUS, red_fill);
    painter.circle_stroke(red_center, DOT_RADIUS - 0.25, Stroke::new(0.5, Color32::from_rgb(224, 68, 62)));

    // Yellow (Minimize): #FEBC2E
    let yellow_fill = if yellow_res.is_pointer_button_down_on() {
        Color32::from_rgb(205, 145, 30)
    } else {
        Color32::from_rgb(254, 188, 46)
    };
    painter.circle_filled(yellow_center, DOT_RADIUS, yellow_fill);
    painter.circle_stroke(yellow_center, DOT_RADIUS - 0.25, Stroke::new(0.5, Color32::from_rgb(222, 161, 35)));

    // Green (Maximize / Zoom): #28C840
    let green_fill = if green_res.is_pointer_button_down_on() {
        Color32::from_rgb(28, 150, 45)
    } else {
        Color32::from_rgb(40, 200, 64)
    };
    painter.circle_filled(green_center, DOT_RADIUS, green_fill);
    painter.circle_stroke(green_center, DOT_RADIUS - 0.25, Stroke::new(0.5, Color32::from_rgb(26, 171, 41)));

    // Draw macOS symbols on hover
    if cluster_hovered {
        // Red symbol: 'x'
        let x_color = Color32::from_rgba_premultiplied(77, 0, 0, 220);
        painter.line_segment([red_center + vec2(-2.5, -2.5), red_center + vec2(2.5, 2.5)], Stroke::new(1.15, x_color));
        painter.line_segment([red_center + vec2(-2.5, 2.5), red_center + vec2(2.5, -2.5)], Stroke::new(1.15, x_color));

        // Yellow symbol: '-'
        let minus_color = Color32::from_rgba_premultiplied(153, 87, 0, 240);
        painter.line_segment([yellow_center + vec2(-3.0, 0.0), yellow_center + vec2(3.0, 0.0)], Stroke::new(1.15, minus_color));

        // Green symbol: two opposing diagonal triangles pointing to corners
        let glyph_green = Color32::from_rgba_premultiplied(0, 100, 0, 220);
        painter.add(egui::Shape::convex_polygon(
            vec![
                green_center + vec2(2.7, -2.7),
                green_center + vec2(0.5, -2.7),
                green_center + vec2(2.7, -0.5),
            ],
            glyph_green,
            Stroke::NONE,
        ));
        painter.add(egui::Shape::convex_polygon(
            vec![
                green_center + vec2(-2.7, 2.7),
                green_center + vec2(-0.5, 2.7),
                green_center + vec2(-2.7, 0.5),
            ],
            glyph_green,
            Stroke::NONE,
        ));
    }

    cluster_hovered || red_res.hovered() || yellow_res.hovered() || green_res.hovered()
}
