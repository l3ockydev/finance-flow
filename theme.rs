//! Fluent Nusantara design system: tokens, typography, shared widgets, icons.

use eframe::egui::{self, Color32, FontId, Pos2, Rect, Response, RichText, Rounding, Stroke, Vec2};

// ---------------------------------------------------------------------------
// Palette
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct ThemeColors {
    pub bg: Color32,
    pub surface_low: Color32,
    pub surface: Color32,
    pub surface_high: Color32,
    pub surface_highest: Color32,
    pub primary: Color32,
    pub primary_hover: Color32,
    pub primary_light: Color32,
    pub primary_soft: Color32,
    pub income: Color32,
    pub income_dark: Color32,
    pub income_soft: Color32,
    pub expense: Color32,
    pub expense_dark: Color32,
    pub expense_soft: Color32,
    pub warning: Color32,
    pub warning_soft: Color32,
    pub text: Color32,
    pub white: Color32,
    pub muted: Color32,
    pub muted_dim: Color32,
    pub border: Color32,
    pub border_soft: Color32,
}

impl ThemeColors {
    pub fn default() -> Self {
        // Palet default mengikuti logo FinFlow: abu gelap #2a2a2a,
        // aksen teal #0f5965, dan navy #202f3a.
        Self {
            bg: Color32::from_rgb(0x20, 0x21, 0x23),
            surface_low: Color32::from_rgb(0x26, 0x27, 0x29),
            surface: Color32::from_rgb(0x2a, 0x2a, 0x2a),
            surface_high: Color32::from_rgb(0x33, 0x34, 0x36),
            surface_highest: Color32::from_rgb(0x3d, 0x3e, 0x40),
            primary: Color32::from_rgb(0x0f, 0x59, 0x65),
            primary_hover: Color32::from_rgb(0x14, 0x70, 0x7e),
            primary_light: Color32::from_rgb(0x7a, 0xcb, 0xd4),
            primary_soft: Color32::from_rgba_premultiplied(0x0f, 0x59, 0x65, 0x60),
            income: Color32::from_rgb(0x5e, 0xc9, 0x9a),
            income_dark: Color32::from_rgb(0x2f, 0x8f, 0x68),
            income_soft: Color32::from_rgba_premultiplied(0x5e, 0xc9, 0x9a, 0x30),
            expense: Color32::from_rgb(0xe8, 0x83, 0x6b),
            expense_dark: Color32::from_rgb(0xc2, 0x5a, 0x44),
            expense_soft: Color32::from_rgba_premultiplied(0xe8, 0x83, 0x6b, 0x30),
            warning: Color32::from_rgb(0xd9, 0xa4, 0x41),
            warning_soft: Color32::from_rgba_premultiplied(0xd9, 0xa4, 0x41, 0x30),
            text: Color32::from_rgb(0xf2, 0xf4, 0xf5),
            white: Color32::from_rgb(0xff, 0xff, 0xff),
            muted: Color32::from_rgb(0xa8, 0xb0, 0xb2),
            muted_dim: Color32::from_rgb(0x74, 0x7c, 0x7e),
            border: Color32::from_rgba_premultiplied(0xff, 0xff, 0xff, 0x12),
            border_soft: Color32::from_rgba_premultiplied(0xff, 0xff, 0xff, 0x08),
        }
    }

}

// ---------------------------------------------------------------------------
// Setup
// ---------------------------------------------------------------------------

pub fn install(ctx: &egui::Context, colors: &ThemeColors) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "plus_jakarta".to_owned(),
        egui::FontData::from_static(include_bytes!("assets/PlusJakartaSans.ttf")),
    );
    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        family.insert(0, "plus_jakarta".to_owned());
    }
    ctx.set_fonts(fonts);

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = colors.bg;
    visuals.window_fill = colors.surface;
    visuals.extreme_bg_color = colors.surface_low;
    visuals.faint_bg_color = Color32::from_rgba_premultiplied(0x04, 0x04, 0x04, 0x04);
    visuals.override_text_color = Some(colors.text);
    visuals.selection.bg_fill = colors.primary_soft;
    visuals.selection.stroke = Stroke::new(1.0_f32, colors.primary_light);
    visuals.window_rounding = Rounding::same(14.0);
    visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 8.0),
        blur: 24.0,
        spread: 0.0,
        color: Color32::from_black_alpha(140),
    };
    visuals.popup_shadow = visuals.window_shadow;

    let w = &mut visuals.widgets;
    w.noninteractive.bg_fill = colors.surface;
    w.noninteractive.bg_stroke = Stroke::new(1.0_f32, colors.border);
    w.noninteractive.rounding = Rounding::same(8.0);
    w.inactive.bg_fill = colors.surface_high;
    w.inactive.weak_bg_fill = colors.surface_high;
    w.inactive.bg_stroke = Stroke::new(1.0_f32, colors.border);
    w.inactive.rounding = Rounding::same(8.0);
    w.hovered.bg_fill = colors.surface_highest;
    w.hovered.weak_bg_fill = colors.surface_highest;
    w.hovered.bg_stroke = Stroke::new(1.0_f32, colors.border_soft);
    w.hovered.rounding = Rounding::same(8.0);
    w.active.bg_fill = colors.primary;
    w.active.weak_bg_fill = colors.primary;
    w.active.bg_stroke = Stroke::new(1.0_f32, colors.primary_light);
    w.active.rounding = Rounding::same(8.0);
    w.open.bg_fill = colors.surface_highest;
    w.open.weak_bg_fill = colors.surface_highest;
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(24.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, FontId::new(14.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, FontId::new(13.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Button, FontId::new(13.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, FontId::new(11.0, egui::FontFamily::Proportional)),
    ]
    .into();
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    style.spacing.menu_margin = egui::Margin::same(6.0);
    style.visuals.window_rounding = Rounding::same(14.0);
    ctx.set_style(style);
}

// ---------------------------------------------------------------------------
// Brand logo (bundled JPEG — decoded once, used for window icon + UI)
// ---------------------------------------------------------------------------

pub const LOGO_BYTES: &[u8] = include_bytes!("assets/logo.jpg");

/// Decode the bundled logo into raw RGBA + dimensions.
pub fn logo_rgba() -> (Vec<u8>, u32, u32) {
    let img = image::load_from_memory(LOGO_BYTES)
        .expect("bundled FinFlow logo must decode")
        .to_rgba8();
    let (w, h) = img.dimensions();
    (img.into_raw(), w, h)
}

/// Native window/taskbar icon.
pub fn window_icon() -> std::sync::Arc<eframe::egui::IconData> {
    let (rgba, w, h) = logo_rgba();
    std::sync::Arc::new(eframe::egui::IconData {
        rgba,
        width: w,
        height: h,
    })
}

/// GPU texture for drawing the logo inside egui.
pub fn logo_texture(ctx: &egui::Context) -> egui::TextureHandle {
    let (rgba, w, h) = logo_rgba();
    let image = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &rgba);
    ctx.load_texture("finflow_logo", image, egui::TextureOptions::LINEAR)
}

// ---------------------------------------------------------------------------
// Containers
// ---------------------------------------------------------------------------

pub fn card<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R, tc: &ThemeColors) -> egui::InnerResponse<R> {
    card_pad(ui, 20.0, add, tc)
}

pub fn card_pad<R>(ui: &mut egui::Ui, pad: f32, add: impl FnOnce(&mut egui::Ui) -> R, tc: &ThemeColors) -> egui::InnerResponse<R> {
    egui::Frame::none()
        .fill(tc.surface)
        .rounding(Rounding::same(16.0))
        .stroke(Stroke::new(1.0_f32, tc.border))
        .inner_margin(egui::Margin::same(pad))
        .show(ui, add)
}

pub fn soft_panel<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R, tc: &ThemeColors) -> egui::InnerResponse<R> {
    egui::Frame::none()
        .fill(tc.surface_low)
        .rounding(Rounding::same(12.0))
        .stroke(Stroke::new(1.0_f32, tc.border_soft))
        .inner_margin(egui::Margin::same(14.0))
        .show(ui, add)
}

pub fn divider(ui: &mut egui::Ui, tc: &ThemeColors) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 1.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, Rounding::ZERO, tc.border);
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

pub fn title(ui: &mut egui::Ui, text: &str, tc: &ThemeColors) {
    ui.label(RichText::new(text).size(26.0).color(tc.white).strong());
}

pub fn section_title(ui: &mut egui::Ui, text: &str, subtitle: &str, tc: &ThemeColors) {
    ui.label(RichText::new(text).size(16.0).color(tc.white).strong());
    if !subtitle.is_empty() {
        ui.add_space(1.0);
        ui.label(RichText::new(subtitle).size(11.0).color(tc.muted));
    }
}

pub fn label(ui: &mut egui::Ui, text: &str, size: f32, color: Color32) {
    ui.label(RichText::new(text).size(size).color(color));
}

pub fn overline(ui: &mut egui::Ui, text: &str, tc: &ThemeColors) {
    ui.label(RichText::new(text).size(10.0).color(tc.muted).strong());
}

// ---------------------------------------------------------------------------
// Controls
// ---------------------------------------------------------------------------

pub fn primary_button(ui: &mut egui::Ui, text: &str, tc: &ThemeColors) -> Response {
    ui.add(
        egui::Button::new(RichText::new(text).size(13.0).color(tc.white).strong())
            .fill(tc.primary)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0x40, 0x40, 0x40, 0x40)))
            .min_size(Vec2::new(0.0, 34.0)),
    )
}

pub fn secondary_button(ui: &mut egui::Ui, text: &str, tc: &ThemeColors) -> Response {
    ui.add(
        egui::Button::new(RichText::new(text).size(13.0).color(tc.text))
            .fill(Color32::from_rgba_premultiplied(0x0f, 0x0f, 0x0f, 0x0f))
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0_f32, tc.border))
            .min_size(Vec2::new(0.0, 34.0)),
    )
}

pub fn ghost_button(ui: &mut egui::Ui, text: &str, tc: &ThemeColors) -> Response {
    ui.add(
        egui::Button::new(RichText::new(text).size(12.0).color(tc.muted))
            .fill(Color32::TRANSPARENT)
            .rounding(Rounding::same(6.0))
            .stroke(Stroke::NONE)
            .min_size(Vec2::new(0.0, 26.0)),
    )
}

pub fn chip(ui: &mut egui::Ui, text: &str, selected: bool, tc: &ThemeColors) -> Response {
    let (fg, bg, stroke) = if selected {
        (tc.primary_light, tc.primary_soft, Stroke::new(1.0_f32, tc.primary))
    } else {
        (tc.muted, tc.surface_low, Stroke::new(1.0_f32, tc.border))
    };
    ui.add(
        egui::Button::new(RichText::new(text).size(12.0).color(fg))
            .fill(bg)
            .rounding(Rounding::same(999.0))
            .stroke(stroke)
            .min_size(Vec2::new(0.0, 28.0)),
    )
}

pub fn pill(ui: &mut egui::Ui, text: &str, fg: Color32, bg: Color32) {
    egui::Frame::none()
        .fill(bg)
        .rounding(Rounding::same(999.0))
        .inner_margin(egui::Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(10.5).color(fg));
        });
}

pub fn progress(ui: &mut egui::Ui, ratio: f32, color: Color32, tc: &ThemeColors) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 6.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, Rounding::same(3.0), tc.surface_high);
    let filled = Rect::from_min_size(rect.min, Vec2::new(width * ratio.clamp(0.0, 1.0), rect.height()));
    if filled.width() > 1.0 {
        painter.rect_filled(filled, Rounding::same(3.0), color);
    }
}

pub fn field(ui: &mut egui::Ui, label_text: &str, value: &mut String, hint: &str, tc: &ThemeColors) {
    ui.label(RichText::new(label_text).size(11.0).color(tc.muted).strong());
    ui.add_space(2.0);
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(RichText::new(hint).color(tc.muted_dim))
            .desired_width(f32::INFINITY)
            .margin(Vec2::new(10.0, 8.0)),
    );
    ui.add_space(4.0);
}

pub fn icon_badge(ui: &mut egui::Ui, icon: Icon, fg: Color32, bg: Color32, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, Rounding::same(10.0), bg);
    paint_icon(painter, rect.shrink(size * 0.28), fg, icon, 1.7);
}

pub fn icon_button(ui: &mut egui::Ui, icon: Icon, tooltip: &str, tc: &ThemeColors) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::click());
    let color = if response.hovered() { tc.white } else { tc.muted };
    if response.hovered() {
        ui.painter().rect_filled(rect, Rounding::same(6.0), Color32::from_rgba_premultiplied(0x0f, 0x0f, 0x0f, 0x0f));
    }
    paint_icon(ui.painter(), rect.shrink(7.0), color, icon, 1.5);
    response.on_hover_text(tooltip)
}

pub fn sub_nav_item(ui: &mut egui::Ui, text: &str, selected: bool, tc: &ThemeColors) -> Response {
    let height = 30.0;
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, Rounding::same(7.0), tc.primary_soft);
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(rect.left() + 12.0, rect.center().y - 7.0), Vec2::new(2.0, 14.0)),
            Rounding::same(1.0),
            tc.primary_light,
        );
    } else if response.hovered() {
        painter.rect_filled(rect, Rounding::same(7.0), Color32::from_rgba_premultiplied(0x0d, 0x0d, 0x0d, 0x0d));
    }
    painter.text(
        Pos2::new(rect.left() + 26.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        FontId::proportional(12.0),
        if selected { tc.primary_light } else { tc.muted },
    );
    response
}

pub fn nav_item(ui: &mut egui::Ui, icon: Icon, text: &str, selected: bool, tc: &ThemeColors) -> Response {
    let height = 40.0;
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, Rounding::same(8.0), tc.primary);
    } else if response.hovered() {
        painter.rect_filled(rect, Rounding::same(8.0), Color32::from_rgba_premultiplied(0x0d, 0x0d, 0x0d, 0x0d));
    }
    let color = if selected { tc.white } else { tc.text };
    paint_icon(painter, Rect::from_center_size(Pos2::new(rect.left() + 20.0, rect.center().y), Vec2::splat(18.0)), color, icon, 1.6);
    painter.text(
        Pos2::new(rect.left() + 38.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        FontId::proportional(13.5),
        color,
    );
    response
}

// ---------------------------------------------------------------------------
// Metric card
// ---------------------------------------------------------------------------

pub struct Metric<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub accent: Color32,
    pub icon: Icon,
    pub icon_fg: Color32,
    pub icon_bg: Color32,
    pub delta: &'a str,
    pub delta_color: Color32,
    pub footnote: &'a str,
    pub progress: Option<(f32, Color32)>,
}

pub fn metric_card(ui: &mut egui::Ui, metric: Metric<'_>, tc: &ThemeColors) {
    card_pad(ui, 18.0, |ui| {
        ui.set_min_height(112.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_max_width(ui.available_width() - 52.0);
                ui.label(RichText::new(metric.label).size(10.5).color(tc.muted).strong());
                ui.add_space(6.0);
                let avail_w = ui.available_width();
                let text = metric.value;
                let galley = ui.painter().layout_no_wrap(text.to_owned(), FontId::proportional(24.0), metric.accent);
                let size = if galley.size().x > avail_w {
                    24.0 * (avail_w / galley.size().x).max(0.55)
                } else {
                    24.0
                };
                ui.label(RichText::new(text).size(size).color(metric.accent).strong());
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                icon_badge(ui, metric.icon, metric.icon_fg, metric.icon_bg, 40.0);
            });
        });
        ui.add_space(10.0);
        if let Some((ratio, color)) = metric.progress {
            progress(ui, ratio, color, tc);
            ui.add_space(6.0);
        }
        ui.horizontal(|ui| {
            if !metric.delta.is_empty() {
                ui.label(RichText::new(metric.delta).size(11.0).color(metric.delta_color).strong());
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(metric.footnote).size(11.0).color(tc.muted));
            });
        });
    }, tc);
}

// ---------------------------------------------------------------------------
// Icons (painted, so no icon font dependency)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum Icon {
    Grid,
    Receipt,
    Calendar,
    Pie,
    Gear,
    Wallet,
    ArrowDown,
    ArrowUp,
    Bell,
    Shield,
    Plus,
    Download,
    Search,
    Check,
    Clock,
    Home,
    Bolt,
    Wifi,
    Filter,
    Doc,
    Trash,
    Edit,
    ChevronRight,
    ChevronLeft,
    Sync,
    Minus,
    Square,
    Close,
    User,
}

pub fn paint_icon(painter: &egui::Painter, rect: Rect, color: Color32, icon: Icon, width: f32) {
    let s = rect.width().min(rect.height());
    let c = rect.center();
    let h = s / 2.0;
    let stroke = Stroke::new(width, color);
    let line = |a: Pos2, b: Pos2| painter.line_segment([a, b], stroke);
    let dot = |p: Pos2, r: f32| painter.circle_filled(p, r, color);
    let ring = |p: Pos2, r: f32| painter.circle_stroke(p, r, stroke);

    match icon {
        Icon::Grid => {
            let g = s * 0.06;
            let cell = (s - g) / 2.0;
            for (dx, dy) in [(0.0, 0.0), (cell + g, 0.0), (0.0, cell + g), (cell + g, cell + g)] {
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(c.x - h + dx, c.y - h + dy), Vec2::splat(cell)),
                    Rounding::same(s * 0.12),
                    color,
                );
            }
        }
        Icon::Receipt => {
            let r = Rect::from_center_size(c, Vec2::new(s * 0.68, s * 0.84));
            painter.rect_stroke(r, Rounding::same(s * 0.1), stroke);
            for i in 0..3 {
                let y = r.top() + s * 0.22 + i as f32 * s * 0.2;
                line(Pos2::new(r.left() + s * 0.14, y), Pos2::new(r.right() - s * 0.14, y));
            }
        }
        Icon::Calendar => {
            let r = Rect::from_center_size(c, Vec2::new(s * 0.82, s * 0.74));
            painter.rect_stroke(r, Rounding::same(s * 0.1), stroke);
            line(Pos2::new(r.left(), r.top() + s * 0.2), Pos2::new(r.right(), r.top() + s * 0.2));
            line(Pos2::new(c.x - s * 0.22, r.top() - s * 0.1), Pos2::new(c.x - s * 0.22, r.top() + s * 0.06));
            line(Pos2::new(c.x + s * 0.22, r.top() - s * 0.1), Pos2::new(c.x + s * 0.22, r.top() + s * 0.06));
            dot(Pos2::new(c.x - s * 0.18, c.y + s * 0.06), s * 0.05);
            dot(Pos2::new(c.x + s * 0.02, c.y + s * 0.06), s * 0.05);
        }
        Icon::Pie => {
            ring(c, h * 0.82);
            line(c, Pos2::new(c.x + h * 0.82, c.y));
            line(c, Pos2::new(c.x, c.y - h * 0.82));
        }
        Icon::Gear => {
            ring(c, h * 0.42);
            dot(c, h * 0.16);
            for i in 0..8 {
                let a = std::f32::consts::TAU * i as f32 / 8.0;
                let p1 = Pos2::new(c.x + a.cos() * h * 0.62, c.y + a.sin() * h * 0.62);
                let p2 = Pos2::new(c.x + a.cos() * h * 0.86, c.y + a.sin() * h * 0.86);
                line(p1, p2);
            }
        }
        Icon::Wallet => {
            let r = Rect::from_center_size(c, Vec2::new(s * 0.86, s * 0.62));
            painter.rect_stroke(r, Rounding::same(s * 0.12), stroke);
            line(Pos2::new(r.left() + s * 0.1, r.top() + s * 0.18), Pos2::new(r.left() + s * 0.72, r.top() + s * 0.18));
            painter.rect_filled(
                Rect::from_center_size(Pos2::new(r.right() - s * 0.18, c.y + s * 0.08), Vec2::new(s * 0.22, s * 0.18)),
                Rounding::same(s * 0.05),
                color,
            );
        }
        Icon::ArrowDown | Icon::ArrowUp => {
            let up = matches!(icon, Icon::ArrowUp);
            let (top, bottom) = if up { (c.y + h * 0.7, c.y - h * 0.7) } else { (c.y - h * 0.7, c.y + h * 0.7) };
            line(Pos2::new(c.x, top), Pos2::new(c.x, bottom));
            let dir = if up { -1.0 } else { 1.0 };
            line(Pos2::new(c.x, bottom), Pos2::new(c.x - h * 0.4, bottom - dir * h * 0.4));
            line(Pos2::new(c.x, bottom), Pos2::new(c.x + h * 0.4, bottom - dir * h * 0.4));
        }
        Icon::Bell => {
            let r = Rect::from_center_size(Pos2::new(c.x, c.y - s * 0.06), Vec2::new(s * 0.58, s * 0.56));
            painter.rect_stroke(r, Rounding::same(s * 0.18), stroke);
            line(Pos2::new(r.left(), r.bottom()), Pos2::new(r.right(), r.bottom()));
            dot(Pos2::new(c.x, r.bottom() + s * 0.14), s * 0.06);
            line(Pos2::new(c.x, r.top() - s * 0.08), Pos2::new(c.x, r.top()));
        }
        Icon::Shield => {
            let pts = vec![
                Pos2::new(c.x, c.y - h * 0.82),
                Pos2::new(c.x + h * 0.66, c.y - h * 0.5),
                Pos2::new(c.x + h * 0.5, c.y + h * 0.36),
                Pos2::new(c.x, c.y + h * 0.82),
                Pos2::new(c.x - h * 0.5, c.y + h * 0.36),
                Pos2::new(c.x - h * 0.66, c.y - h * 0.5),
            ];
            painter.add(egui::Shape::convex_polygon(pts, Color32::TRANSPARENT, stroke));
            line(Pos2::new(c.x - h * 0.24, c.y), Pos2::new(c.x - h * 0.04, c.y + h * 0.2));
            line(Pos2::new(c.x - h * 0.04, c.y + h * 0.2), Pos2::new(c.x + h * 0.28, c.y - h * 0.22));
        }
        Icon::Plus => {
            line(Pos2::new(c.x - h * 0.6, c.y), Pos2::new(c.x + h * 0.6, c.y));
            line(Pos2::new(c.x, c.y - h * 0.6), Pos2::new(c.x, c.y + h * 0.6));
        }
        Icon::Download => {
            line(Pos2::new(c.x, c.y - h * 0.7), Pos2::new(c.x, c.y + h * 0.25));
            line(Pos2::new(c.x, c.y + h * 0.25), Pos2::new(c.x - h * 0.32, c.y - h * 0.1));
            line(Pos2::new(c.x, c.y + h * 0.25), Pos2::new(c.x + h * 0.32, c.y - h * 0.1));
            line(Pos2::new(c.x - h * 0.7, c.y + h * 0.65), Pos2::new(c.x + h * 0.7, c.y + h * 0.65));
        }
        Icon::Search => {
            ring(Pos2::new(c.x - h * 0.14, c.y - h * 0.14), h * 0.48);
            line(Pos2::new(c.x + h * 0.22, c.y + h * 0.22), Pos2::new(c.x + h * 0.68, c.y + h * 0.68));
        }
        Icon::Check => {
            line(Pos2::new(c.x - h * 0.62, c.y), Pos2::new(c.x - h * 0.16, c.y + h * 0.45));
            line(Pos2::new(c.x - h * 0.16, c.y + h * 0.45), Pos2::new(c.x + h * 0.66, c.y - h * 0.42));
        }
        Icon::Clock => {
            ring(c, h * 0.78);
            line(c, Pos2::new(c.x, c.y - h * 0.4));
            line(c, Pos2::new(c.x + h * 0.32, c.y + h * 0.18));
        }
        Icon::Home => {
            line(Pos2::new(c.x - h * 0.72, c.y), Pos2::new(c.x, c.y - h * 0.62));
            line(Pos2::new(c.x, c.y - h * 0.62), Pos2::new(c.x + h * 0.72, c.y));
            line(Pos2::new(c.x - h * 0.5, c.y - h * 0.06), Pos2::new(c.x - h * 0.5, c.y + h * 0.66));
            line(Pos2::new(c.x + h * 0.5, c.y - h * 0.06), Pos2::new(c.x + h * 0.5, c.y + h * 0.66));
            line(Pos2::new(c.x - h * 0.5, c.y + h * 0.66), Pos2::new(c.x + h * 0.5, c.y + h * 0.66));
        }
        Icon::Bolt => {
            let pts = vec![
                Pos2::new(c.x + h * 0.12, c.y - h * 0.82),
                Pos2::new(c.x - h * 0.5, c.y + h * 0.08),
                Pos2::new(c.x - h * 0.02, c.y + h * 0.08),
                Pos2::new(c.x - h * 0.16, c.y + h * 0.82),
                Pos2::new(c.x + h * 0.52, c.y - h * 0.12),
                Pos2::new(c.x + h * 0.04, c.y - h * 0.12),
            ];
            painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
        }
        Icon::Wifi => {
            for (i, r) in [0.72_f32, 0.46, 0.2].iter().enumerate() {
                let radius = h * r;
                let mut pts = Vec::new();
                for k in 0..=12 {
                    let a = std::f32::consts::PI * (0.25 + 0.5 * k as f32 / 12.0);
                    pts.push(Pos2::new(c.x - a.cos() * radius, c.y + h * 0.42 - a.sin() * radius));
                }
                let _ = i;
                painter.add(egui::Shape::line(pts, stroke));
            }
            dot(Pos2::new(c.x, c.y + h * 0.42), width * 1.4);
        }
        Icon::Filter => {
            line(Pos2::new(c.x - h * 0.7, c.y - h * 0.5), Pos2::new(c.x + h * 0.7, c.y - h * 0.5));
            line(Pos2::new(c.x - h * 0.42, c.y), Pos2::new(c.x + h * 0.42, c.y));
            line(Pos2::new(c.x - h * 0.14, c.y + h * 0.5), Pos2::new(c.x + h * 0.14, c.y + h * 0.5));
        }
        Icon::Doc => {
            let r = Rect::from_center_size(c, Vec2::new(s * 0.62, s * 0.8));
            painter.rect_stroke(r, Rounding::same(s * 0.08), stroke);
            for i in 0..3 {
                let y = r.top() + s * 0.22 + i as f32 * s * 0.18;
                line(Pos2::new(r.left() + s * 0.1, y), Pos2::new(r.right() - s * 0.1, y));
            }
        }
        Icon::Trash => {
            line(Pos2::new(c.x - h * 0.5, c.y - h * 0.45), Pos2::new(c.x + h * 0.5, c.y - h * 0.45));
            line(Pos2::new(c.x - h * 0.16, c.y - h * 0.45), Pos2::new(c.x - h * 0.16, c.y - h * 0.66));
            line(Pos2::new(c.x + h * 0.16, c.y - h * 0.45), Pos2::new(c.x + h * 0.16, c.y - h * 0.66));
            painter.rect_stroke(
                Rect::from_min_max(Pos2::new(c.x - h * 0.36, c.y - h * 0.34), Pos2::new(c.x + h * 0.36, c.y + h * 0.66)),
                Rounding::same(s * 0.06),
                stroke,
            );
        }
        Icon::Edit => {
            line(Pos2::new(c.x - h * 0.55, c.y + h * 0.55), Pos2::new(c.x + h * 0.35, c.y - h * 0.55));
            line(Pos2::new(c.x + h * 0.35, c.y - h * 0.55), Pos2::new(c.x + h * 0.62, c.y - h * 0.28));
            line(Pos2::new(c.x + h * 0.62, c.y - h * 0.28), Pos2::new(c.x - h * 0.28, c.y + h * 0.62));
            line(Pos2::new(c.x - h * 0.55, c.y + h * 0.55), Pos2::new(c.x - h * 0.62, c.y + h * 0.78));
            line(Pos2::new(c.x - h * 0.62, c.y + h * 0.78), Pos2::new(c.x - h * 0.28, c.y + h * 0.62));
        }
        Icon::ChevronRight | Icon::ChevronLeft => {
            let dir = if matches!(icon, Icon::ChevronRight) { 1.0 } else { -1.0 };
            line(Pos2::new(c.x - dir * h * 0.28, c.y - h * 0.55), Pos2::new(c.x + dir * h * 0.28, c.y));
            line(Pos2::new(c.x + dir * h * 0.28, c.y), Pos2::new(c.x - dir * h * 0.28, c.y + h * 0.55));
        }
        Icon::Sync => {
            // Open arc (gap at upper-right) so it works on any themed background.
            let r = h * 0.62;
            let steps = 28;
            let start = -0.1_f32;
            let sweep = std::f32::consts::TAU - 1.1;
            for i in 0..steps {
                let a0 = start + sweep * i as f32 / steps as f32;
                let a1 = start + sweep * (i + 1) as f32 / steps as f32;
                line(
                    Pos2::new(c.x + a0.cos() * r, c.y + a0.sin() * r),
                    Pos2::new(c.x + a1.cos() * r, c.y + a1.sin() * r),
                );
            }
            line(Pos2::new(c.x + h * 0.36, c.y - h * 0.62), Pos2::new(c.x + h * 0.66, c.y - h * 0.36));
            line(Pos2::new(c.x + h * 0.66, c.y - h * 0.36), Pos2::new(c.x + h * 0.32, c.y - h * 0.22));
        }
        Icon::Minus => {
            line(Pos2::new(c.x - h * 0.6, c.y), Pos2::new(c.x + h * 0.6, c.y));
        }
        Icon::Square => {
            painter.rect_stroke(Rect::from_center_size(c, Vec2::splat(s * 0.66)), Rounding::same(2.0), stroke);
        }
        Icon::Close => {
            line(Pos2::new(c.x - h * 0.55, c.y - h * 0.55), Pos2::new(c.x + h * 0.55, c.y + h * 0.55));
            line(Pos2::new(c.x + h * 0.55, c.y - h * 0.55), Pos2::new(c.x - h * 0.55, c.y + h * 0.55));
        }
        Icon::User => {
            ring(Pos2::new(c.x, c.y - h * 0.34), h * 0.32);
            let mut pts = Vec::new();
            for k in 0..=14 {
                let a = std::f32::consts::PI * (1.0 + k as f32 / 14.0);
                pts.push(Pos2::new(c.x + a.cos() * h * 0.68, c.y + h * 0.66 + a.sin() * h * 0.44));
            }
            painter.add(egui::Shape::line(pts, stroke));
        }
    }
}

pub fn category_color(category: &str, tc: &ThemeColors) -> Color32 {
    match category {
        "Gaji & Bonus" | "Pendapatan Sampingan" => tc.income,
        "Investasi" => Color32::from_rgb(0xc4, 0xb5, 0xfd),
        "Kebutuhan Pokok" => tc.primary_light,
        "Transportasi" => Color32::from_rgb(0x67, 0xe8, 0xf9),
        "Makanan & Minuman" => Color32::from_rgb(0xf9, 0xa8, 0xd4),
        "Hiburan" => Color32::from_rgb(0xd8, 0xb4, 0xfe),
        "Pajak & Tagihan" | "Utilitas" => tc.warning,
        _ => tc.muted,
    }
}

pub fn category_bg(category: &str, tc: &ThemeColors) -> Color32 {
    let c = category_color(category, &tc);
    Color32::from_rgba_premultiplied(c.r() / 6, c.g() / 6, c.b() / 6, 0x50)
}
