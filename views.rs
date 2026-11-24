//! Dashboard view and shared transaction table row rendering.

use crate::model::*;
use crate::theme::{ThemeColors, *};
use crate::{App, Modal, View};
use eframe::egui::{self, Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2};

#[derive(PartialEq)]
pub enum RowAction {
    None,
    Open(String),
    Edit(String),
    Delete(String),
}

pub fn category_icon(category: &str) -> Icon {
    match category {
        "Kebutuhan Pokok" => Icon::Home,
        "Transportasi" => Icon::Wallet,
        "Makanan & Minuman" => Icon::Receipt,
        "Pajak & Tagihan" | "Utilitas" => Icon::Bolt,
        "Investasi" => Icon::Pie,
        "Gaji & Bonus" | "Pendapatan Sampingan" => Icon::ArrowDown,
        "Internet" => Icon::Wifi,
        _ => Icon::Doc,
    }
}

pub fn bill_icon(bill: &Bill) -> Icon {
    let name = bill.name.to_lowercase();
    if name.contains("internet") || name.contains("indihome") || name.contains("wifi") {
        Icon::Wifi
    } else if name.contains("listrik") || name.contains("pln") || name.contains("token") {
        Icon::Bolt
    } else if name.contains("kpr") || name.contains("cicilan") || name.contains("rumah") {
        Icon::Home
    } else {
        Icon::Calendar
    }
}

fn legend(ui: &mut egui::Ui, text: &str, color: Color32, muted: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, Rounding::same(2.0), color);
    ui.label(egui::RichText::new(text).size(10.5).color(muted));
}

pub struct Cols {
    pub desc: f32,
}

pub fn widths(avail: f32, actions: bool) -> Cols {
    let status_w = if actions { 0.0 } else { 88.0 };
    let fixed = 34.0 + 140.0 + 100.0 + status_w + 132.0 + if actions { 76.0 } else { 0.0 } + 56.0;
    Cols { desc: (avail - fixed).max(120.0) }
}

pub fn table_header(ui: &mut egui::Ui, cols: &Cols, actions: bool, tc: &ThemeColors) {
    ui.add_space(2.0);
    let status_w = if actions { 0.0 } else { 88.0 };
    let row_h = 26.0;
    let total_w = 34.0 + cols.desc + 140.0 + 100.0 + status_w + 132.0 + if actions { 76.0 } else { 0.0 };
    let (row_rect, _) = ui.allocate_exact_size(Vec2::new(total_w, row_h), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(row_rect, Rounding::same(8.0), tc.surface_low);
    let mut x = row_rect.left() + 34.0;
    
    // DESKRIPSI & REKENING
    let rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(cols.desc, row_h));
    painter.text(rect.left_top() + Vec2::new(0.0, 7.0), egui::Align2::LEFT_TOP, "DESKRIPSI & REKENING", FontId::proportional(10.0), tc.muted_dim);
    x += cols.desc;
    
    // KATEGORI
    let rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(140.0, row_h));
    painter.text(rect.left_top() + Vec2::new(0.0, 7.0), egui::Align2::LEFT_TOP, "KATEGORI", FontId::proportional(10.0), tc.muted_dim);
    x += 140.0;
    
    // TANGGAL
    let rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(100.0, row_h));
    painter.text(rect.left_top() + Vec2::new(0.0, 7.0), egui::Align2::LEFT_TOP, "TANGGAL", FontId::proportional(10.0), tc.muted_dim);
    x += 100.0;
    
    // STATUS (only when not actions)
    if !actions {
        let rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(status_w, row_h));
        painter.text(rect.left_top() + Vec2::new(0.0, 7.0), egui::Align2::LEFT_TOP, "STATUS", FontId::proportional(10.0), tc.muted_dim);
        x += status_w;
    }
    
    // NOMINAL (right-aligned)
    let rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(132.0, row_h));
    painter.text(rect.right_top() + Vec2::new(0.0, 7.0), egui::Align2::RIGHT_TOP, "NOMINAL", FontId::proportional(10.0), tc.muted_dim);
    
    ui.add_space(2.0);
    divider(ui, tc);
}

pub fn tx_row(ui: &mut egui::Ui, tx: &Transaction, cols: &Cols, actions: bool, tc: &ThemeColors) -> RowAction {
    let mut action = RowAction::None;
    let row_h = 52.0;
    let status_w = if actions { 0.0 } else { 88.0 };
    let total_w = 34.0 + cols.desc + 140.0 + 100.0 + status_w + 132.0 + if actions { 76.0 } else { 0.0 };
    let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(total_w, row_h), egui::Sense::click());
    let painter = ui.painter();
    if row_resp.hovered() {
        painter.rect_filled(
            row_rect.shrink2(Vec2::new(2.0, 2.0)),
            Rounding::same(10.0),
            tc.surface_high,
        );
    }
    let mut x = row_rect.left() + 34.0;
    
    // ICON column (34px)
    let icon_rect = Rect::from_min_size(Pos2::new(row_rect.left(), row_rect.top()), Vec2::new(34.0, row_h));
    let tint = category_color(&tx.category, tc);
    painter.rect_filled(icon_rect, Rounding::same(10.0), category_bg(&tx.category, tc));
    paint_icon(painter, icon_rect.shrink(9.0), tint, category_icon(&tx.category), 1.5);
    
    // DESKRIPSI & REKENING
    let desc_rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(cols.desc, row_h));
    // Note text
    let note_galley = painter.layout_no_wrap(tx.note.clone(), FontId::proportional(12.5), tc.text);
    let note_x = desc_rect.left() + 2.0;
    let note_y = desc_rect.top() + 6.0;
    let note_rect = Rect::from_min_size(Pos2::new(note_x, note_y), note_galley.size());
    let note_resp = ui.interact(note_rect, ui.id().with(tx.id.as_str()).with("note"), egui::Sense::click());
    if note_resp.clicked() {
        action = RowAction::Open(tx.id.clone());
    }
    painter.galley(Pos2::new(note_x, note_y), note_galley, tc.text);
    // Account/sub text
    let sub = if tx.image.is_some() {
        format!("{}  •  Bukti terlampir", tx.account)
    } else {
        tx.account.clone()
    };
    let sub_galley = painter.layout_no_wrap(sub, FontId::proportional(10.5), tc.muted);
    let sub_x = desc_rect.left() + 2.0;
    let sub_y = desc_rect.top() + 26.0;
    painter.galley(Pos2::new(sub_x, sub_y), sub_galley, tc.muted);
    x += cols.desc;
    
    // KATEGORI (pill)
    let cat_rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(140.0, row_h));
    let cat_center = cat_rect.center();
    let pill_w = 130.0_f32.min(cat_rect.width() - 8.0);
    let pill_h = 22.0;
    let pill_rect = Rect::from_center_size(Pos2::new(cat_center.x, cat_center.y), Vec2::new(pill_w, pill_h));
    painter.rect_filled(pill_rect, Rounding::same(999.0), category_bg(&tx.category, tc));
    let cat_galley = painter.layout_no_wrap(tx.category.clone(), FontId::proportional(11.0), category_color(&tx.category, tc));
    painter.galley(pill_rect.center() - cat_galley.size() / 2.0, cat_galley, category_color(&tx.category, tc));
    x += 140.0;
    
    // TANGGAL
    let date_rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(100.0, row_h));
    let date_galley = painter.layout_no_wrap(format_date_short(&tx.date), FontId::proportional(10.5), tc.text);
    let time_galley = painter.layout_no_wrap(format!("{} WIB", tx.time), FontId::proportional(10.0), tc.muted_dim);
    painter.galley(Pos2::new(date_rect.left() + 2.0, date_rect.top() + 8.0), date_galley, tc.text);
    painter.galley(Pos2::new(date_rect.left() + 2.0, date_rect.top() + 27.0), time_galley, tc.muted);
    x += 100.0;
    
    // STATUS (only when not actions)
    if !actions {
        let status_rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(status_w, row_h));
        let icon_rect = Rect::from_center_size(Pos2::new(status_rect.left() + 10.0, status_rect.center().y), Vec2::splat(12.0));
        painter.circle_filled(icon_rect.center(), 6.0, tc.income);
        paint_icon(painter, icon_rect, tc.white, Icon::Check, 1.5);
        let status_galley = painter.layout_no_wrap("Selesai".to_string(), FontId::proportional(10.5), tc.income);
        painter.galley(Pos2::new(status_rect.left() + 24.0, status_rect.center().y - 7.0), status_galley, tc.income);
        x += status_w;
    }
    
    // NOMINAL (right-aligned)
    let nominal_rect = Rect::from_min_size(Pos2::new(x, row_rect.top()), Vec2::new(132.0, row_h));
    let color = if tx.kind == TxKind::Income { tc.income } else { tc.expense };
    let amt_galley = painter.layout_no_wrap(format_signed(tx.kind, tx.amount), FontId::proportional(13.0), color);
    painter.galley(Pos2::new(nominal_rect.right() - amt_galley.size().x, nominal_rect.center().y - amt_galley.size().y / 2.0), amt_galley, color);
    
    // ACTIONS (Edit/Delete buttons)
    if actions {
        let action_x = nominal_rect.right() + 8.0;
        let row_mid = row_rect.center().y;
        let edit_rect = Rect::from_min_size(Pos2::new(action_x, row_mid - 15.0), Vec2::splat(30.0));
        let edit_resp = ui.interact(edit_rect, ui.id().with(tx.id.as_str()).with("edit"), egui::Sense::click());
        if edit_resp.hovered() {
            painter.rect_filled(edit_rect, Rounding::same(7.0), tc.surface_high);
        }
        paint_icon(painter, edit_rect.shrink(9.0), if edit_resp.hovered() { tc.text } else { tc.muted }, Icon::Edit, 1.5);
        if edit_resp.clicked() {
            action = RowAction::Edit(tx.id.clone());
        }

        let del_rect = Rect::from_min_size(Pos2::new(action_x + 38.0, row_mid - 15.0), Vec2::splat(30.0));
        let del_resp = ui.interact(del_rect, ui.id().with(tx.id.as_str()).with("delete"), egui::Sense::click());
        if del_resp.hovered() {
            painter.rect_filled(del_rect, Rounding::same(7.0), tc.expense_soft);
        }
        paint_icon(painter, del_rect.shrink(9.0), if del_resp.hovered() { tc.expense } else { tc.muted }, Icon::Trash, 1.5);
        if del_resp.clicked() {
            action = RowAction::Delete(tx.id.clone());
        }
    }
    
    // Row click for detail (if no action clicked)
    if action == RowAction::None && row_resp.clicked() && !actions {
        action = RowAction::Open(tx.id.clone());
    }
    
    // Divider
    painter.line_segment(
        [Pos2::new(row_rect.left(), row_rect.bottom()), Pos2::new(row_rect.right(), row_rect.bottom())],
        Stroke::new(1.0_f32, tc.border_soft),
    );
    
    action
}

impl App {
    pub fn dashboard(&mut self, ui: &mut egui::Ui) {
        let Some(p) = self.current_project().cloned() else {
            self.empty_state(ui);
            return;
        };

        let tc = self.theme_colors;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                page_heading(
                    ui,
                    "Ringkasan",
                    "Dashboard",
                    "Kondisi keuangan proyek Anda bulan ini.",
                    &tc,
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_button(ui, "+ Tambah Transaksi", &tc).clicked() {
                    self.open_modal(Modal::AddTransaction);
                }
            });
        });

        ui.add_space(12.0);

        let gap = 12.0;
        let width = ((ui.available_width() - gap * 3.0) / 4.0).floor().max(160.0);
        let (income, expense) = p.this_month_totals();
        let budget_total: f64 = p.budgets.iter().map(|b| b.limit).sum();
        let expense_ratio = if budget_total > 0.0 { (expense / budget_total) as f32 } else { 0.0 };
        let income_ratio = if income + expense > 0.0 { (income / (income + expense)) as f32 } else { 0.0 };
        let pending = p.bills.iter().filter(|b| b.status != BillStatus::Paid).count();

        let balance_txt = format_rp(p.balance());
        let income_txt = format_rp(income);
        let expense_txt = format_rp(expense);
        let pending_txt = format_rp(p.pending_bills_total());
        let expense_delta = format!("{:.1}% dari Anggaran", expense_ratio * 100.0);
        let expense_foot = String::new();
        let pending_delta = format!("● {} Tagihan Aktif", pending);

        let cards = [
            Metric {
                label: "SALDO SAAT INI",
                value: &balance_txt,
                accent: tc.text,
                icon: Icon::Wallet,
                icon_fg: tc.primary_light,
                icon_bg: tc.primary_soft,
                delta: "+8.4%",
                delta_color: tc.income,
                footnote: "vs bulan lalu",
                progress: None,
            },
            Metric {
                label: "PEMASUKAN",
                value: &income_txt,
                accent: tc.income,
                icon: Icon::ArrowDown,
                icon_fg: tc.income,
                icon_bg: tc.income_soft,
                delta: "+12.0%",
                delta_color: tc.income,
                footnote: "Gaji & Dividen",
                progress: Some((income_ratio, tc.income)),
            },
            Metric {
                label: "PENGELUARAN",
                value: &expense_txt,
                accent: tc.expense,
                icon: Icon::ArrowUp,
                icon_fg: tc.expense,
                icon_bg: tc.expense_soft,
                delta: &expense_delta,
                delta_color: tc.expense,
                footnote: &expense_foot,
                progress: Some((expense_ratio, tc.expense)),
            },
            Metric {
                label: "TAGIHAN MENUNGGU BAYAR",
                value: &pending_txt,
                accent: tc.text,
                icon: Icon::Bell,
                icon_fg: tc.warning,
                icon_bg: tc.warning_soft,
                delta: &pending_delta,
                delta_color: tc.muted,
                footnote: "Prioritas: WiFi & Listrik",
                progress: None,
            },
        ];

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for (idx, metric) in cards.into_iter().enumerate() {
                if idx > 0 {
                    ui.add_space(gap);
                }
                ui.allocate_ui_with_layout(
                    Vec2::new(width, 144.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| metric_card(ui, metric, &tc),
                );
            }
        });

        ui.add_space(14.0);

        let total_w = ui.available_width();
        let left_w = ((total_w - 14.0) * 0.635).floor();
        let right_w = total_w - left_w - 14.0;
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(Vec2::new(left_w, 312.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                self.cashflow_card(ui, &p);
            });
            ui.add_space(14.0);
            ui.allocate_ui_with_layout(Vec2::new(right_w, 312.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                self.upcoming_bills_card(ui, &p);
            });
        });

        ui.add_space(14.0);

        let tc = self.theme_colors;
        card(ui, |ui| {
            ui.horizontal(|ui| {
                section_title(ui, "Transaksi Terbaru", "", &tc);
                ui.add_space(4.0);
                pill(ui, "5 Data Terkini", tc.muted, tc.surface_high);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if secondary_button(ui, "Ekspor", &tc).clicked() {
                        self.open_modal(Modal::Export);
                    }
                    if secondary_button(ui, "Filter", &tc).clicked() {
                        self.view = View::Transactions;
                    }
                });
            });
            ui.add_space(10.0);
            let cols = widths(ui.available_width(), false);
            table_header(ui, &cols, false, &tc);
            ui.add_space(2.0);
            for tx in p.transactions.iter().rev().take(5) {
                match tx_row(ui, tx, &cols, false, &tc) {
                    RowAction::Open(id) => {
                        self.detail_tx = Some(id);
                        self.open_modal(Modal::Detail);
                    }
                    _ => {}
                }
                divider(ui, &tc);
            }
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                label(ui, &format!("Menampilkan {} dari {} transaksi", p.transactions.len().min(5), p.transactions.len()), 10.5, tc.muted);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ghost_button(ui, "Buka Buku Transaksi Lengkap  ›", &tc).clicked() {
                        self.view = View::Transactions;
                    }
                });
            });
        }, &tc);
    }

    fn cashflow_card(&mut self, ui: &mut egui::Ui, p: &Project) {
        let tc = self.theme_colors;
        card_pad(ui, 20.0, |ui| {
            ui.set_min_height(300.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    section_title(ui, "Arus Kas & Rasio Tabungan", "Perbandingan Pemasukan vs Pengeluaran 6 Bulan Terakhir", &tc);
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let ratio = p.savings_ratio() * 100.0;
                    egui::Frame::none()
                        .fill(tc.income_soft)
                        .rounding(Rounding::same(8.0))
                        .inner_margin(egui::Margin::symmetric(9.0, 5.0))
                        .show(ui, |ui| {
                            ui.set_width(66.0);
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("Rasio Hemat").size(9.0).color(tc.income));
                                ui.label(egui::RichText::new(format!("{:.1}%", ratio)).size(12.0).color(tc.income).strong());
                            });
                        });
                });
            });
            ui.add_space(12.0);

            let months = p.month_totals(6);
            let max = months.iter().map(|(_, i, e)| i.max(*e)).fold(1.0_f64, f64::max);
            let desired = Vec2::new(ui.available_width(), 176.0);
            let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
            let painter = ui.painter();
            let base_y = rect.bottom() - 24.0;
            let chart_h = rect.height() - 44.0;
            for i in 1..=3 {
                let y = base_y - chart_h * i as f32 / 3.0;
                painter.line_segment(
                    [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                    Stroke::new(1.0_f32, tc.border_soft),
                );
            }
            painter.line_segment(
                [Pos2::new(rect.left(), base_y), Pos2::new(rect.right(), base_y)],
                Stroke::new(1.0_f32, tc.border),
            );

            let group_w = rect.width() / months.len() as f32;
            let bar_w = (group_w * 0.16).clamp(9.0, 18.0);
            for (idx, (name, income, expense)) in months.iter().enumerate() {
                let cx = rect.left() + group_w * (idx as f32 + 0.5);
                let ih = (income / max) as f32 * chart_h;
                let eh = (expense / max) as f32 * chart_h;
                let gap = 5.0;
                let income_rect = Rect::from_min_max(
                    Pos2::new(cx - bar_w - gap * 0.5, base_y - ih.max(2.0)),
                    Pos2::new(cx - gap * 0.5, base_y),
                );
                let expense_rect = Rect::from_min_max(
                    Pos2::new(cx + gap * 0.5, base_y - eh.max(2.0)),
                    Pos2::new(cx + bar_w + gap * 0.5, base_y),
                );
                painter.rect_filled(income_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, tc.income);
                painter.rect_filled(expense_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, tc.expense);
                painter.text(
                    Pos2::new(cx, base_y + 8.0),
                    egui::Align2::CENTER_TOP,
                    name,
                    egui::FontId::proportional(10.0),
                    tc.muted,
                );
                if idx == months.len() - 1 && ih > 4.0 {
                    let tag = Rect::from_center_size(Pos2::new(cx - gap * 0.5 - bar_w * 0.5, base_y - ih - 12.0), Vec2::new(32.0, 16.0));
                    painter.rect_filled(tag, Rounding::same(4.0), tc.primary_soft);
                    painter.text(tag.center(), egui::Align2::CENTER_CENTER, "Aktif", egui::FontId::proportional(9.0), tc.primary_light);
                }
            }

            // Legend sits below-left of the chart.
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                legend(ui, "Pemasukan", tc.income, tc.muted);
                ui.add_space(10.0);
                legend(ui, "Pengeluaran", tc.expense, tc.muted);
            });
        }, &tc);
    }

    fn upcoming_bills_card(&mut self, ui: &mut egui::Ui, p: &Project) {
        let tc = self.theme_colors;
        card_pad(ui, 20.0, |ui| {
            ui.set_min_height(300.0);
            ui.horizontal(|ui| {
                section_title(ui, "Jatuh Tempo Segera", "", &tc);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ghost_button(ui, "Kelola Semua  ›", &tc).clicked() {
                        self.view = View::Bills;
                    }
                });
            });
            ui.add_space(8.0);
            let bills: Vec<&Bill> = p.bills.iter().filter(|b| b.status != BillStatus::Paid).take(3).collect();
            for bill in &bills {
                soft_panel(ui, |ui| {
                    ui.horizontal(|ui| {
                        let icon = bill_icon(bill);
                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(34.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, Rounding::same(9.0), tc.surface_high);
                        paint_icon(ui.painter(), rect.shrink(8.5), tc.primary_light, icon, 1.5);
                        ui.add_space(8.0);
                        let text_w = (ui.available_width() - 104.0).max(70.0);
                        ui.vertical(|ui| {
                            ui.set_width(text_w);
                            marquee_label(ui, &bill.name, 11.5, tc.text, text_w);
                            let days = days_until(&bill.due_date);
                            let when = if days < 0 {
                                format!("{} (terlambat {} hari)", format_date_short(&bill.due_date), -days)
                            } else if days == 0 {
                                format!("{} (hari ini)", format_date_short(&bill.due_date))
                            } else {
                                format!("{} ({} hari lagi)", format_date_short(&bill.due_date), days)
                            };
                            ui.add(
                                egui::Label::new(egui::RichText::new(when).size(9.5).color(tc.muted))
                                    .truncate(true),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(format_rp(bill.amount)).size(11.5).color(tc.text).strong());
                        });
                    });
                }, &tc);
                ui.add_space(6.0);
            }
            if bills.is_empty() {
                ui.label(egui::RichText::new("Tidak ada tagihan tertunda. Semua sudah lunas.").size(11.0).color(tc.muted));
            }
        }, &tc);
    }
}

/// Single-line label that scrolls right-to-left when the text is wider than its box.
fn marquee_label(ui: &mut egui::Ui, text: &str, size: f32, color: Color32, width: f32) {
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), egui::FontId::proportional(size), color);
    let text_w = galley.size().x;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, size + 4.0), egui::Sense::hover());
    let painter = ui.painter().with_clip_rect(rect);
    if text_w <= rect.width() {
        painter.galley(rect.left_top() + Vec2::new(0.0, 1.0), galley, color);
    } else {
        let t = ui.input(|i| i.time) as f32;
        let gap = 48.0;
        let span = text_w + gap;
        let offset = (t * 28.0) % span;
        let x = rect.left() - offset;
        painter.galley(Pos2::new(x, rect.top() + 1.0), galley.clone(), color);
        painter.galley(Pos2::new(x + span, rect.top() + 1.0), galley, color);
        ui.ctx().request_repaint();
    }
}
