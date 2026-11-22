//! Transactions ledger, bills, analysis, settings, and empty state.

use crate::model::{*, Theme};
use crate::theme::*;
use crate::views::{bill_icon, category_icon, table_header, tx_row, widths, RowAction};
use crate::{App, Modal, SettingsSection};
use eframe::egui::{self, Color32, FontId, Pos2, Rect, RichText, Rounding, Stroke, Vec2};

impl App {
    pub fn transactions(&mut self, ui: &mut egui::Ui) {
        let Some(p) = self.current_project().cloned() else {
            self.empty_state(ui);
            return;
        };

        let tc = self.theme_colors;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                title(ui, "Buku Transaksi", &tc);
                ui.add_space(2.0);
                label(ui, "Seluruh catatan pemasukan dan pengeluaran proyek ini.", 11.5, tc.muted);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_button(ui, "+ Tambah Transaksi", &tc).clicked() {
                    self.form.reset_entry(TxKind::Income);
                    self.open_modal(Modal::AddTransaction);
                }
                if secondary_button(ui, "Ekspor CSV / Excel / PDF", &tc).clicked() {
                    self.open_modal(Modal::Export);
                }
            });
        });

        ui.add_space(16.0);

        let search = self.search.to_lowercase();
        let filtered: Vec<Transaction> = p
            .transactions
            .iter()
            .filter(|t| self.filter_kind.map_or(true, |k| t.kind == k))
            .filter(|t| self.filter_category.as_ref().map_or(true, |c| &t.category == c))
            .filter(|t| {
                search.is_empty()
                    || t.note.to_lowercase().contains(&search)
                    || t.category.to_lowercase().contains(&search)
                    || t.account.to_lowercase().contains(&search)
            })
            .cloned()
            .collect();

        let income: f64 = filtered.iter().filter(|t| t.kind == TxKind::Income).map(|t| t.amount).sum();
        let expense: f64 = filtered.iter().filter(|t| t.kind == TxKind::Expense).map(|t| t.amount).sum();
        let net = income - expense;

        let gap = 14.0;
        let width = ((ui.available_width() - gap * 2.0) / 3.0).floor().max(200.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(Vec2::new(width, 148.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                metric_card(ui, Metric {
                    label: "PEMASUKAN TERFILTER",
                    value: &format_rp(income),
                    accent: tc.income,
                    icon: Icon::ArrowDown,
                    icon_fg: tc.income,
                    icon_bg: tc.income_soft,
                    delta: "+12.4%",
                    delta_color: tc.income,
                    footnote: "vs bulan lalu",
                    progress: None,
                }, &tc);
            });
            ui.add_space(gap);
            ui.allocate_ui_with_layout(Vec2::new(width, 148.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                metric_card(ui, Metric {
                    label: "PENGELUARAN TERFILTER",
                    value: &format_rp(expense),
                    accent: tc.expense,
                    icon: Icon::ArrowUp,
                    icon_fg: tc.expense,
                    icon_bg: tc.expense_soft,
                    delta: "-4.1%",
                    delta_color: tc.income,
                    footnote: "anggaran terjaga",
                    progress: None,
                }, &tc);
            });
            ui.add_space(gap);
            ui.allocate_ui_with_layout(Vec2::new(width, 148.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                metric_card(ui, Metric {
                    label: "ARUS KAS BERSIH",
                    value: &format_rp(net),
                    accent: if net >= 0.0 { tc.income } else { tc.expense },
                    icon: Icon::Wallet,
                    icon_fg: tc.primary_light,
                    icon_bg: tc.primary_soft,
                    delta: if net >= 0.0 { "Surplus" } else { "Defisit" },
                    delta_color: if net >= 0.0 { tc.income } else { tc.expense },
                    footnote: "dari data terfilter",
                    progress: None,
                }, &tc);
            });
        });

        ui.add_space(16.0);

        card_pad(ui, 14.0, |ui| {
            ui.horizontal(|ui| {
                if chip(ui, "Semua Transaksi", self.filter_kind.is_none(), &tc).clicked() {
                    self.filter_kind = None;
                    self.page = 0;
                }
                if chip(ui, "Pemasukan", self.filter_kind == Some(TxKind::Income), &tc).clicked() {
                    self.filter_kind = Some(TxKind::Income);
                    self.page = 0;
                }
                if chip(ui, "Pengeluaran", self.filter_kind == Some(TxKind::Expense), &tc).clicked() {
                    self.filter_kind = Some(TxKind::Expense);
                    self.page = 0;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    label(ui, &format!("{} transaksi", filtered.len()), 10.5, tc.muted);
                });
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                let search_w = (ui.available_width() - 190.0).max(160.0);
                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text(RichText::new("Cari keterangan, kategori, rekening...").color(tc.muted_dim))
                        .desired_width(search_w)
                        .margin(Vec2::new(10.0, 6.0)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut categories: Vec<String> = p.transactions.iter().map(|t| t.category.clone()).collect();
                    categories.sort();
                    categories.dedup();
                    egui::ComboBox::from_id_source("filter_category")
                        .selected_text(self.filter_category.clone().unwrap_or_else(|| "Semua Kategori".into()))
                        .width(170.0)
                        .show_ui(ui, |ui| {
                            if ui.selectable_label(self.filter_category.is_none(), "Semua Kategori").clicked() {
                                self.filter_category = None;
                                self.page = 0;
                            }
                            for cat in &categories {
                                if ui.selectable_label(self.filter_category.as_deref() == Some(cat.as_str()), cat).clicked() {
                                    self.filter_category = Some(cat.clone());
                                    self.page = 0;
                                }
                            }
                        });
                });
            });
        }, &tc);

        ui.add_space(14.0);

        let per_page = 10usize;
        let pages = filtered.len().div_ceil(per_page).max(1);
        if self.page >= pages {
            self.page = pages - 1;
        }
        let start = self.page * per_page;

        card(ui, |ui| {
            let cols = widths(ui.available_width(), true);
            table_header(ui, &cols, true, &tc);
            ui.add_space(2.0);
            if filtered.is_empty() {
                ui.add_space(30.0);
                ui.vertical_centered(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(40.0), egui::Sense::hover());
                    paint_icon(ui.painter(), r, tc.muted_dim, Icon::Search, 1.6);
                    ui.add_space(6.0);
                    label(ui, "Tidak ada transaksi yang cocok dengan filter.", 12.0, tc.muted);
                });
                ui.add_space(30.0);
            }
            for tx in filtered.iter().skip(start).take(per_page) {
                match tx_row(ui, tx, &cols, true, &tc) {
                    RowAction::Open(id) => {
                        self.detail_tx = Some(id);
                        self.open_modal(Modal::Detail);
                    }
                    RowAction::Edit(id) => self.open_edit_transaction(&id),
                    RowAction::Delete(id) => {
                        self.pending_delete = Some(id);
                        self.open_modal(Modal::ConfirmDelete);
                    }
                    RowAction::None => {}
                }
                divider(ui, &tc);
            }
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                label(ui, &format!("Menampilkan {} - {} dari {} transaksi", if filtered.is_empty() { 0 } else { start + 1 }, (start + per_page).min(filtered.len()), filtered.len()), 10.5, tc.muted);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_enabled(self.page + 1 < pages, egui::Button::new(RichText::new("Berikutnya  ›").size(11.0).color(tc.text))).clicked() {
                        self.page += 1;
                    }
                    label(ui, &format!("Halaman {} dari {}", self.page + 1, pages), 10.5, tc.muted);
                    if ui.add_enabled(self.page > 0, egui::Button::new(RichText::new("‹  Sebelumnya").size(11.0).color(tc.text))).clicked() {
                        self.page = self.page.saturating_sub(1);
                    }
                });
            });
        }, &tc);
    }

    pub fn bills(&mut self, ui: &mut egui::Ui) {
        let Some(p) = self.current_project().cloned() else {
            self.empty_state(ui);
            return;
        };

        let tc = self.theme_colors;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                title(ui, "Kelola Tagihan", &tc);
                ui.add_space(2.0);
                label(ui, "Pantau tagihan rutin, jatuh tempo, dan status pembayaran.", 11.5, tc.muted);
            });
            
        });

        ui.add_space(16.0);

        let pending = p.pending_bills_total();
        let paid = p.paid_bills_total();
        let gap = 14.0;
        let width = ((ui.available_width() - gap * 2.0) / 3.0).floor().max(200.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(Vec2::new(width, 148.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                metric_card(ui, Metric {
                    label: "TOTAL KEWAJIBAN",
                    value: &format_rp(pending + paid),
                    accent: tc.white,
                    icon: Icon::Wallet,
                    icon_fg: tc.primary_light,
                    icon_bg: tc.primary_soft,
                    delta: &format!("{} tagihan", p.bills.len()),
                    delta_color: tc.muted,
                    footnote: "bulan ini",
                    progress: None,
                }, &tc);
            });
            ui.add_space(gap);
            ui.allocate_ui_with_layout(Vec2::new(width, 148.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                metric_card(ui, Metric {
                    label: "SUDAH TERBAYAR",
                    value: &format_rp(paid),
                    accent: tc.income,
                    icon: Icon::Check,
                    icon_fg: tc.income,
                    icon_bg: tc.income_soft,
                    delta: "Lunas",
                    delta_color: tc.income,
                    footnote: "terverifikasi",
                    progress: None,
                }, &tc);
            });
            ui.add_space(gap);
            ui.allocate_ui_with_layout(Vec2::new(width, 148.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                metric_card(ui, Metric {
                    label: "BELUM DIBAYAR",
                    value: &format_rp(pending),
                    accent: tc.warning,
                    icon: Icon::Clock,
                    icon_fg: tc.warning,
                    icon_bg: tc.warning_soft,
                    delta: "Menunggu",
                    delta_color: tc.warning,
                    footnote: "jatuh tempo dekat",
                    progress: None,
                }, &tc);
            });
        });

        ui.add_space(18.0);

        card(ui, |ui| {
            ui.horizontal(|ui| {
                section_title(ui, "Daftar Tagihan", "Kelola pembayaran rutin dan cicilan", &tc);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    pill(ui, &format!("{} tagihan", p.bills.len()), tc.muted, Color32::from_rgba_premultiplied(0x0c, 0x0c, 0x0c, 0x0c));
                });
            });
            ui.add_space(12.0);

            if p.bills.is_empty() {
                ui.add_space(20.0);
                ui.vertical_centered(|ui| label(ui, "Belum ada tagihan. Tambahkan tagihan pertama Anda.", 12.0, tc.muted));
                ui.add_space(20.0);
            }

            for bill in &p.bills {
                let is_paid = bill.status == BillStatus::Paid;
                soft_panel(ui, |ui| {
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(40.0), egui::Sense::hover());
                        let tint = if is_paid { tc.income } else { tc.warning };
                        ui.painter().rect_filled(rect, Rounding::same(10.0), if is_paid { tc.income_soft } else { tc.warning_soft });
                        paint_icon(ui.painter(), rect.shrink(10.0), tint, bill_icon(bill), 1.6);

                        ui.add_space(4.0);
                        ui.vertical(|ui| {
                            ui.set_width((ui.available_width() - 300.0).max(160.0));
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&bill.name).size(12.5).color(tc.white).strong());
                                let (fg, bg) = match bill.status {
                                    BillStatus::Paid => (tc.income, tc.income_soft),
                                    BillStatus::Pending => (tc.warning, tc.warning_soft),
                                    BillStatus::Scheduled => (tc.primary_light, tc.primary_soft),
                                    BillStatus::Overdue => (tc.expense, tc.expense_soft),
                                };
                                pill(ui, bill.status.label(), fg, bg);
                            });
                            let days = days_until(&bill.due_date);
                            let due = if is_paid {
                                format!("Lunas  •  jatuh tempo {}", format_date_short(&bill.due_date))
                            } else if days < 0 {
                                format!("Jatuh tempo {}  •  terlambat {} hari", format_date_short(&bill.due_date), -days)
                            } else {
                                format!("Jatuh tempo {}  •  {} hari lagi", format_date_short(&bill.due_date), days)
                            };
                            ui.label(egui::RichText::new(due).size(10.0).color(tc.muted));
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if !is_paid {
                                if ui
                                    .add(
                                        egui::Button::new(egui::RichText::new("Tandai Lunas").size(10.5).color(tc.white))
                                            .fill(tc.primary)
                                            .rounding(Rounding::same(6.0))
                                            .min_size(Vec2::new(0.0, 26.0)),
                                    )
                                    .clicked()
                                {
                                    self.mark_bill_paid(&bill.id);
                                }
                            }
                            if icon_button(ui, Icon::Trash, "Hapus tagihan", &tc).clicked() {
                                self.delete_bill(&bill.id);
                            }
                            ui.label(egui::RichText::new(format_rp(bill.amount)).size(13.0).color(if is_paid { tc.muted } else { tc.text }).strong());
                            ui.label(egui::RichText::new(&bill.category).size(10.0).color(tc.muted));
                        });
                    });
                }, &tc);
                ui.add_space(8.0);
            }
        }, &tc);
    }

    pub fn analysis(&mut self, ui: &mut egui::Ui) {
        let Some(p) = self.current_project().cloned() else {
            self.empty_state(ui);
            return;
        };

        let tc = self.theme_colors;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                title(ui, "Analisis & Anggaran", &tc);
                ui.add_space(2.0);
                label(ui, "Kendali belanja, rasio tabungan, dan komposisi pengeluaran.", 11.5, tc.muted);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if secondary_button(ui, "Ekspor Laporan", &tc).clicked() {
                    self.open_modal(Modal::Export);
                }
            });
        });

        ui.add_space(16.0);

        let total_w = ui.available_width();
        let left_w = (total_w - 16.0) * 0.58;
        let right_w = total_w - left_w - 16.0;
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(Vec2::new(left_w, 360.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                card(ui, |ui| {
                    section_title(ui, "Realisasi Anggaran", "Batas pengeluaran per kategori", &tc);
                    ui.add_space(14.0);
                    if p.budgets.is_empty() {
                        label(ui, "Belum ada anggaran yang ditetapkan.", 11.0, tc.muted);
                    }
                    for budget in &p.budgets {
                        let spent = p.spent_for(&budget.category);
                        let ratio = if budget.limit > 0.0 { (spent / budget.limit) as f32 } else { 0.0 };
                        let color = if ratio >= 0.9 { tc.expense } else if ratio >= 0.7 { tc.warning } else { tc.income };
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&budget.category).size(12.0).color(tc.text).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new(format!("{} / {}", format_rp(spent), format_rp(budget.limit))).size(10.0).color(tc.muted));
                                ui.label(egui::RichText::new(format!("{:.0}%", ratio * 100.0)).size(10.5).color(color).strong());
                            });
                        });
                        ui.add_space(4.0);
                        progress(ui, ratio, color, &tc);
                        ui.add_space(12.0);
                    }
                    ui.add_space(2.0);
                    divider(ui, &tc);
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        let total_budget: f64 = p.budgets.iter().map(|b| b.limit).sum();
                        label(ui, "Total anggaran", 11.0, tc.muted);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            label(ui, &format_rp(total_budget), 12.0, tc.text);
                        });
                    });
                }, &tc);
            });
            ui.add_space(16.0);
            ui.allocate_ui_with_layout(Vec2::new(right_w, 360.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                self.health_card(ui, &p);
            });
        });

        ui.add_space(18.0);

        card(ui, |ui| {
            ui.horizontal(|ui| {
                section_title(ui, "Pengeluaran per Kategori", "Komposisi pengeluaran pada proyek ini", &tc);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    pill(ui, &format!("Total {}", format_rp(p.total_expense())), tc.expense, tc.expense_soft);
                });
            });
            ui.add_space(12.0);

            let mut cats: Vec<(String, f64)> = Vec::new();
            for t in p.transactions.iter().filter(|t| t.kind == TxKind::Expense) {
                if let Some(entry) = cats.iter_mut().find(|(c, _)| c == &t.category) {
                    entry.1 += t.amount;
                } else {
                    cats.push((t.category.clone(), t.amount));
                }
            }
            cats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let max = cats.first().map(|c| c.1).unwrap_or(1.0).max(1.0);
            let total = p.total_expense().max(1.0);

            for (category, amount) in &cats {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, Rounding::same(7.0), category_bg(category, &tc));
                    paint_icon(ui.painter(), rect.shrink(7.0), category_color(category, &tc), category_icon(category), 1.4);
                    ui.add_space(4.0);
                    ui.allocate_ui_with_layout(Vec2::new(160.0, 22.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(category).size(11.5).color(tc.text));
                    });
                    let bar_w = (ui.available_width() - 220.0).max(60.0);
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(bar_w, 8.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, Rounding::same(4.0), tc.surface_high);
                    let filled = Rect::from_min_size(rect.min, Vec2::new(bar_w * (*amount / max) as f32, 8.0));
                    ui.painter().rect_filled(filled, Rounding::same(4.0), category_color(category, &tc));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(format!("{:.1}%", amount / total * 100.0)).size(10.0).color(tc.muted));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(format_rp(*amount)).size(11.0).color(tc.text).strong());
                    });
                });
                ui.add_space(9.0);
            }
            if cats.is_empty() {
                label(ui, "Belum ada pengeluaran tercatat.", 11.0, tc.muted);
            }
        }, &tc);
    }

    fn health_card(&mut self, ui: &mut egui::Ui, p: &Project) {
        let tc = self.theme_colors;
        card(ui, |ui| {
            section_title(ui, "Kesehatan Kas", "Rasio tabungan dari total pemasukan", &tc);
            ui.add_space(10.0);

            let ratio = p.savings_ratio();
            let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 168.0), egui::Sense::hover());
            let center = Pos2::new(rect.center().x, rect.center().y + 4.0);
            let radius = 58.0;
            let painter = ui.painter();
            painter.circle_stroke(center, radius, Stroke::new(14.0, Color32::from_rgba_premultiplied(0x0c, 0x0c, 0x0c, 0x0c)));

            let mut cats: Vec<(String, f64)> = Vec::new();
            for t in p.transactions.iter().filter(|t| t.kind == TxKind::Expense) {
                if let Some(entry) = cats.iter_mut().find(|(c, _)| c == &t.category) {
                    entry.1 += t.amount;
                } else {
                    cats.push((t.category.clone(), t.amount));
                }
            }
            let total: f64 = cats.iter().map(|c| c.1).sum();
            if total > 0.0 {
                let mut start = -std::f32::consts::FRAC_PI_2;
                for (category, amount) in &cats {
                    let sweep = (*amount / total) as f32 * std::f32::consts::TAU;
                    draw_arc(painter, center, radius, start + 0.02, start + sweep - 0.02, category_color(category, &tc));
                    start += sweep;
                }
            }
            painter.text(center + Vec2::new(0.0, -8.0), egui::Align2::CENTER_CENTER, format!("{:.0}%", ratio * 100.0), egui::FontId::proportional(26.0), tc.white);
            painter.text(center + Vec2::new(0.0, 16.0), egui::Align2::CENTER_CENTER, "Rasio Hemat", egui::FontId::proportional(9.5), tc.muted);

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    label(ui, "Saldo bersih", 10.0, tc.muted);
                    label(ui, &format_rp(p.balance()), 14.0, if p.balance() >= 0.0 { tc.income } else { tc.expense });
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let status = if ratio >= 0.3 { ("Optimal", tc.income) } else if ratio >= 0.15 { ("Cukup", tc.warning) } else { ("Perlu Perhatian", tc.expense) };
                    pill(ui, status.0, status.1, if ratio >= 0.3 { tc.income_soft } else if ratio >= 0.15 { tc.warning_soft } else { tc.expense_soft });
                });
            });
        }, &tc);
    }

    pub fn settings(&mut self, ui: &mut egui::Ui) {
        let tc = self.theme_colors;
        let Some(p) = self.current_project().cloned() else {
            self.empty_state(ui);
            return;
        };
        if self.form.name.is_empty() {
            self.form.name = p.title.clone();
        }
        if self.form.owner.is_empty() {
            self.form.owner = p.owner.clone();
        }

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                title(ui, "Pengaturan", &tc);
                ui.add_space(2.0);
                label(ui, &format!("Bagian: {}", self.settings_section.label()), 11.5, tc.muted);
            });
        });

        ui.add_space(16.0);

        match self.settings_section {
            SettingsSection::Default => self.settings_default(ui, &p),
            SettingsSection::Payments => self.settings_payments(ui, &p),
            SettingsSection::Income => self.settings_categories(ui, &p, SettingsSection::Income),
            SettingsSection::Expenses => self.settings_categories(ui, &p, SettingsSection::Expenses),
            SettingsSection::Bills => self.settings_categories(ui, &p, SettingsSection::Bills),
            SettingsSection::Theme => self.settings_theme(ui, &p),
        }
    }

    fn settings_default(&mut self, ui: &mut egui::Ui, p: &Project) {
        let tc = self.theme_colors;

        ui.columns(2, |cols| {
            // ── KOLOM KIRI ─────────────────────────────────────────────────
            cols[0].vertical(|ui| {
                card(ui, |ui| {
                    section_title(ui, "Profil Proyek", "Nama laporan dan pemilik", &tc);
                    ui.add_space(14.0);
                    field(ui, "NAMA PROYEK", &mut self.form.name, "Contoh: Keuangan Pribadi 2026", &tc);
                    field(ui, "PEMILIK / PEMBUAT LAPORAN", &mut self.form.owner, "Nama lengkap", &tc);
                    ui.add_space(10.0);
                    if primary_button(ui, "Simpan Perubahan", &tc).clicked() {
                        self.save_profile(self.form.name.clone(), self.form.owner.clone());
                    }
                    ui.add_space(6.0);
                    label(ui, &format!("Terakhir diperbarui {}", format_date_short(&p.updated_at)), 10.0, tc.muted_dim);
                }, &tc);

                ui.add_space(14.0);

                card(ui, |ui| {
                    section_title(ui, "Penyimpanan Lokal", "Satu proyek, satu file .ff", &tc);
                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(36.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, Rounding::same(9.0), tc.primary_soft);
                        paint_icon(ui.painter(), rect.shrink(9.0), tc.primary_light, Icon::Doc, 1.5);
                        ui.add_space(12.0);
                        ui.vertical(|ui| {
                            label(ui, &format!("{}.ff", p.id), 12.5, tc.text);
                            label(ui, &self.dir.display().to_string(), 9.5, tc.muted);
                        });
                    });
                    ui.add_space(16.0);
                    for (key, value) in [
                        ("Format", "JSON • .ff".to_string()),
                        ("Transaksi", format!("{} catatan", p.transactions.len())),
                        ("Proyek tersimpan", format!("{} proyek", self.projects.len())),
                    ] {
                        ui.horizontal(|ui| {
                            label(ui, key, 10.5, tc.muted);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                label(ui, &value, 10.5, tc.text);
                            });
                        });
                        ui.add_space(6.0);
                    }
                    ui.add_space(10.0);
                    ui.columns(2, |btn| {
                        if secondary_button(&mut btn[0], "Buka Folder", &tc).clicked() {
                            self.open_projects_folder();
                        }
                        if secondary_button(&mut btn[1], "Ekspor / Cetak", &tc).clicked() {
                            self.open_modal(Modal::Export);
                        }
                    });
                }, &tc);
            });

            // ── KOLOM KANAN ────────────────────────────────────────────────
            cols[1].vertical(|ui| {
                card(ui, |ui| {
                    section_title(ui, "Tindakan Proyek", "Kelola laporan keuangan terpisah", &tc);
                    ui.add_space(14.0);
                    ui.columns(2, |btn| {
                        if secondary_button(&mut btn[0], "+ Proyek Baru", &tc).clicked() {
                            self.open_modal(Modal::NewProject);
                        }
                        if secondary_button(&mut btn[1], "Buka File .ff", &tc).clicked() {
                            self.open_project_dialog();
                        }
                    });
                    ui.add_space(8.0);
                    ui.columns(2, |btn| {
                        if secondary_button(&mut btn[0], "Simpan Salinan", &tc).clicked() {
                            self.save_copy_dialog();
                        }
                        if secondary_button(&mut btn[1], "Hapus Proyek", &tc).clicked() {
                            self.open_modal(Modal::ConfirmDeleteProject);
                        }
                    });
                    ui.add_space(16.0);
                    divider(ui, &tc);
                    ui.add_space(14.0);
                    section_title(ui, "Aplikasi", "Versi dan lisensi", &tc);
                    ui.add_space(12.0);
                    soft_panel(ui, |ui| {
                        ui.horizontal(|ui| {
                            if let Some(tex) = &self.logo_tex {
                                ui.add(
                                    egui::Image::new(tex)
                                        .fit_to_exact_size(Vec2::splat(36.0))
                                        .rounding(Rounding::same(9.0)),
                                );
                                ui.add_space(10.0);
                            }
                            ui.vertical(|ui| {
                                label(ui, "FinFlow v1.0", 13.5, tc.text);
                                ui.add_space(4.0);
                                label(ui, "Rust + egui 0.27", 10.5, tc.muted);
                                label(ui, "Windows 10/11", 10.5, tc.muted);
                                ui.add_space(8.0);
                                label(ui, "Offline-only • Per-project .ff", 9.5, tc.muted_dim);
                                ui.add_space(4.0);
                                label(ui, "build by Cakra Bisnis", 9.5, tc.muted_dim);
                                label(ui, "License for Personal use", 9.5, tc.muted_dim);
                            });
                        });
                    }, &tc);
                }, &tc);
            });
        });
    }

    fn settings_theme(&mut self, ui: &mut egui::Ui, _p: &Project) {
        let tc = self.theme_colors;
        let total_w = ui.available_width().max(600.0);
        card(ui, |ui| {
            ui.set_min_width(total_w);
            section_title(ui, "Pilih Tema", "Ganti warna tampilan aplikasi", &tc);
            ui.add_space(16.0);
            
            let themes = [
                Theme::Default,
                Theme::Banking,
                Theme::DarkFinance,
                Theme::Minimal,
                Theme::Emerald,
                Theme::Amber,
                Theme::Violet,
            ];
            
            // Grid 2 kolom untuk tema
            let cols = 2;
            let gap = 12.0;
            let card_w = (total_w - 40.0 - gap) / cols as f32;
            
            for (idx, theme) in themes.iter().enumerate() {
                let col = idx % cols;

                if col == 0 {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = gap;
                        self.render_theme_card(ui, *theme, card_w);
                        if idx + 1 < themes.len() {
                            self.render_theme_card(ui, themes[idx + 1], card_w);
                        } else {
                            // Empty spacer for odd count
                            ui.allocate_exact_size(Vec2::new(card_w, 1.0), egui::Sense::hover());
                        }
                    });
                    ui.add_space(12.0);
                }
            }
            
            ui.add_space(8.0);
            divider(ui, &tc);
            ui.add_space(12.0);
            
            // Preview section
            if let Some(proj) = self.current_project() {
                let tc = ThemeColors::from_theme(proj.theme);
                ui.horizontal(|ui| {
                    label(ui, "Pratinjau", 14.0, tc.text);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        label(ui, ThemeColors::label(proj.theme), 12.0, tc.muted);
                    });
                });
                ui.add_space(12.0);
                
                // Preview card
                let preview_rect = ui.allocate_exact_size(Vec2::new(total_w - 40.0, 140.0), egui::Sense::hover()).0;
                ui.painter().rect_filled(preview_rect, Rounding::same(12.0), tc.surface);
                ui.painter().rect_stroke(preview_rect, Rounding::same(12.0), Stroke::new(1.0, tc.border));
                
                // Preview content
                ui.put(preview_rect.shrink(20.0).translate(Vec2::new(0.0, -10.0)), 
                    egui::Label::new(RichText::new("FinFlow").size(24.0).color(tc.text).strong()));
                ui.put(preview_rect.shrink(20.0).translate(Vec2::new(0.0, 22.0)), 
                    egui::Label::new(RichText::new("Keuangan Pribadi").size(13.0).color(tc.muted)));
                
                // Color swatches
                let swatch_y = preview_rect.top() + 90.0;
                let swatches = [
                    ("Background", tc.bg),
                    ("Surface", tc.surface),
                    ("Primary", tc.primary),
                    ("Income", tc.income),
                    ("Expense", tc.expense),
                    ("Warning", tc.warning),
                    ("Text", tc.text),
                    ("Muted", tc.muted),
                    ("Border", tc.border),
                ];
                let swatch_w = (total_w - 40.0 - 16.0) / 9.0;
                for (i, (name, color)) in swatches.iter().enumerate() {
                    let x = preview_rect.left() + 20.0 + i as f32 * (swatch_w + 2.0);
                    let swatch_rect = Rect::from_min_size(Pos2::new(x, swatch_y), Vec2::new(swatch_w, 28.0));
                    ui.painter().rect_filled(swatch_rect, Rounding::same(8.0), *color);
                    ui.painter().text(
                        swatch_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        name,
                        FontId::proportional(8.0),
                        if *color == tc.bg || *color == tc.surface { tc.text } else { tc.white },
                    );
                }
                
                ui.add_space(160.0);
                label(ui, "Perubahan tema berlaku instan di seluruh aplikasi.", 10.0, tc.muted);
            }
        }, &tc);
    }
    
    fn render_theme_card(&mut self, ui: &mut egui::Ui, theme: Theme, card_w: f32) {
        let Some(proj) = self.current_project().cloned() else { return };
        let is_active = proj.theme == theme;
        let tc = ThemeColors::from_theme(theme);
        let response = ui.allocate_response(
            Vec2::new(card_w, 64.0),
            egui::Sense::click(),
        );
        let rect = response.rect;
        let bg = if is_active { tc.primary_soft } else { tc.surface };
        let stroke_color = if is_active { tc.primary } else { tc.border };
        let stroke_width = if is_active { 2.5_f32 } else { 1.0_f32 };
        
        ui.painter().rect_stroke(rect, Rounding::same(12.0), Stroke::new(stroke_width, stroke_color));
        ui.painter().rect_filled(rect.shrink(2.0), Rounding::same(10.0), bg);
        
        // Draw label text centered
        let label_text = ThemeColors::label(theme);
        let galley = ui.painter().layout_no_wrap(label_text.to_string(), FontId::proportional(14.0), tc.text);
        ui.painter().galley(
            rect.center() - galley.size() / 2.0,
            galley,
            tc.text,
        );
        
        // Active indicator
        if is_active {
            let dot_rect = Rect::from_min_size(
                Pos2::new(rect.right() - 28.0, rect.top() + 8.0),
                Vec2::splat(18.0)
            );
            ui.painter().circle_filled(dot_rect.center(), 9.0, tc.primary);
            paint_icon(ui.painter(), dot_rect.shrink(4.0), tc.white, Icon::Check, 1.4);
        }
        
        if response.clicked() {
            if let Some(proj_mut) = self.current_project_mut() {
                proj_mut.theme = theme;
                proj_mut.updated_at = crate::model::today_iso();
                self.save_current();
                self.refresh_theme();
                crate::theme::install(ui.ctx(), &self.theme_colors);
                self.toast("Tema diperbarui");
            }
        }
    }

    fn settings_payments(&mut self, ui: &mut egui::Ui, p: &Project) {
        let tc = self.theme_colors;
        let total_w = ui.available_width();
        let left_w = ((total_w - 16.0) * 0.55).floor();
        let right_w = total_w - left_w - 16.0;
        let mut to_remove: Option<usize> = None;
        let mut add = false;
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(Vec2::new(left_w, 420.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                card(ui, |ui| {
                    section_title(ui, "Metode Pembayaran", "Akun bank, kartu, dan dompet digital", &tc);
                    ui.add_space(12.0);
                    if p.bank_accounts.is_empty() {
                        label(ui, "Belum ada akun. Tambahkan akun pertama Anda.", 11.0, tc.muted);
                        ui.add_space(6.0);
                    }
                    for (idx, acc) in p.bank_accounts.iter().enumerate() {
                        soft_panel(ui, |ui| {
                            ui.horizontal(|ui| {
                                let (r, _) = ui.allocate_exact_size(Vec2::splat(16.0), egui::Sense::hover());
                                paint_icon(ui.painter(), r, tc.primary_light, Icon::Wallet, 1.3);
                                ui.add_space(8.0);
                                ui.label(RichText::new(acc).size(11.5).color(tc.text));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if icon_button(ui, Icon::Trash, "Hapus akun", &tc).clicked() {
                                        to_remove = Some(idx);
                                    }
                                });
                            });
                        }, &tc);
                        ui.add_space(6.0);
                    }
                    ui.add_space(6.0);
                    if add_item_row(ui, "AKUN BARU", &mut self.form.bank_name, "Contoh: BRI Simpedes", &tc) {
                        add = true;
                    }
                }, &tc);
            });
            ui.add_space(16.0);
            ui.allocate_ui_with_layout(Vec2::new(right_w, 420.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                card(ui, |ui| {
                    section_title(ui, "Cara Kerja", "Akun dipakai sebagai pilihan", &tc);
                    ui.add_space(12.0);
                    label(ui, "Daftar akun ini muncul pada formulir pemasukan, pengeluaran, dan tagihan sebagai pilihan akun bank.", 11.0, tc.muted);
                    ui.add_space(10.0);
                    divider(ui, &tc);
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        label(ui, "Jumlah akun", 10.5, tc.muted);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| label(ui, &format!("{} akun", p.bank_accounts.len()), 10.5, tc.text));
                    });
                }, &tc);
            });
        });

        if let Some(idx) = to_remove {
            if let Some(proj) = self.current_project_mut() {
                if idx < proj.bank_accounts.len() {
                    proj.bank_accounts.remove(idx);
                }
                proj.updated_at = today_iso();
            }
        }
        if add {
            let new_acc = self.form.bank_name.trim().to_string();
            if !new_acc.is_empty() {
                if let Some(proj) = self.current_project_mut() {
                    if !proj.bank_accounts.contains(&new_acc) {
                        proj.bank_accounts.push(new_acc);
                    }
                    proj.updated_at = today_iso();
                }
                self.form.bank_name.clear();
            }
        }
    }

    fn settings_categories(&mut self, ui: &mut egui::Ui, p: &Project, section: SettingsSection) {
        let (heading, subtitle, items, help) = match section {
            SettingsSection::Income => (
                "Kategori Pemasukan",
                "Pilihan kategori saat mencatat pemasukan.",
                p.income_categories.clone(),
                "Kategori ini muncul pada tab Pemasukan di formulir transaksi.",
            ),
            SettingsSection::Expenses => (
                "Kategori Pengeluaran",
                "Pilihan kategori saat mencatat pengeluaran.",
                p.expense_categories.clone(),
                "Kategori ini muncul pada tab Pengeluaran di formulir transaksi.",
            ),
            SettingsSection::Bills => (
                "Kategori Tagihan",
                "Pilihan kategori saat mencatat tagihan.",
                p.bill_categories.clone(),
                "Kategori ini muncul pada tab Tagihan di formulir transaksi.",
            ),
            _ => return,
        };

        let tc = self.theme_colors;
        let total_w = ui.available_width();
        let left_w = ((total_w - 16.0) * 0.55).floor();
        let right_w = total_w - left_w - 16.0;
        let mut remove: Option<usize> = None;
        let mut add = false;

        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(Vec2::new(left_w, 420.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                card(ui, |ui| {
                    section_title(ui, heading, subtitle, &tc);
                    ui.add_space(12.0);
                    for (idx, item) in items.iter().enumerate() {
                        soft_panel(ui, |ui| {
                            ui.horizontal(|ui| {
                                let (r, _) = ui.allocate_exact_size(Vec2::splat(16.0), egui::Sense::hover());
                                paint_icon(ui.painter(), r, tc.primary_light, Icon::Doc, 1.2);
                                ui.add_space(8.0);
                                ui.label(RichText::new(item).size(11.5).color(tc.text));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if icon_button(ui, Icon::Trash, "Hapus kategori", &tc).clicked() {
                                        remove = Some(idx);
                                    }
                                });
                            });
                        }, &tc);
                        ui.add_space(6.0);
                    }
                    ui.add_space(6.0);
                    if add_item_row(ui, "KATEGORI BARU", &mut self.form.settings_item, "Contoh: Donasi", &tc) {
                        add = true;
                    }
                }, &tc);
            });
            ui.add_space(16.0);
            ui.allocate_ui_with_layout(Vec2::new(right_w, 420.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                card(ui, |ui| {
                    section_title(ui, "Cara Kerja", "", &tc);
                    ui.add_space(12.0);
                    label(ui, help, 11.0, tc.muted);
                    ui.add_space(10.0);
                    divider(ui, &tc);
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        label(ui, "Jumlah kategori", 10.5, tc.muted);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| label(ui, &format!("{} kategori", items.len()), 10.5, tc.text));
                    });
                }, &tc);
            });
        });

        if let Some(idx) = remove {
            if let Some(list) = self.category_list_mut(section) {
                if idx < list.len() {
                    list.remove(idx);
                }
            }
            if let Some(proj) = self.current_project_mut() {
                proj.updated_at = today_iso();
            }
        }
        if add {
            let new_item = self.form.settings_item.trim().to_string();
            if !new_item.is_empty() {
                if let Some(list) = self.category_list_mut(section) {
                    if !list.contains(&new_item) {
                        list.push(new_item);
                    }
                }
                if let Some(proj) = self.current_project_mut() {
                    proj.updated_at = today_iso();
                }
                self.form.settings_item.clear();
            }
        }
    }

    fn category_list_mut(&mut self, section: SettingsSection) -> Option<&mut Vec<String>> {
        let proj = self.current_project_mut()?;
        Some(match section {
            SettingsSection::Income => &mut proj.income_categories,
            SettingsSection::Expenses => &mut proj.expense_categories,
            SettingsSection::Bills => &mut proj.bill_categories,
            _ => return None,
        })
    }

    pub fn empty_state(&mut self, ui: &mut egui::Ui) {
        let tc = self.theme_colors;
        ui.add_space(90.0);
        ui.horizontal(|ui| {
            let width = 440.0;
            ui.add_space(((ui.available_width() - width) / 2.0).max(0.0));
            card_pad(ui, 28.0, |ui| {
                ui.set_width(width - 56.0);
                ui.vertical_centered(|ui| {
                    if let Some(tex) = &self.logo_tex {
                        ui.add(
                            egui::Image::new(tex)
                                .fit_to_exact_size(Vec2::splat(64.0))
                                .rounding(Rounding::same(14.0)),
                        );
                    } else {
                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(56.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, Rounding::same(16.0), tc.primary_soft);
                        paint_icon(ui.painter(), rect.shrink(15.0), tc.primary_light, Icon::Doc, 1.6);
                    }
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("Belum Ada Proyek").size(20.0).color(tc.white).strong());
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Setiap laporan keuangan disimpan sebagai file .ff terpisah, sehingga data antar proyek tidak pernah tercampur.").size(11.0).color(tc.muted));
                    ui.add_space(16.0);
                    if primary_button(ui, "+ Buat Proyek Baru", &tc).clicked() {
                        self.open_modal(Modal::NewProject);
                    }
                    if ghost_button(ui, "atau buka file .ff yang sudah ada", &tc).clicked() {
                        self.open_project_dialog();
                    }
                });
            }, &tc);
        });
    }
}

fn add_item_row(ui: &mut egui::Ui, label_text: &str, value: &mut String, hint: &str, tc: &ThemeColors) -> bool {
    let mut clicked = false;
    ui.label(RichText::new(label_text).size(10.0).color(tc.muted).strong());
    ui.add_space(3.0);
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(value)
                .hint_text(RichText::new(hint).color(tc.muted_dim))
                .desired_width(240.0)
                .margin(Vec2::new(10.0, 7.0)),
        );
        if primary_button(ui, "Tambah", tc).clicked() {
            clicked = true;
        }
    });
    clicked
}

fn draw_arc(painter: &egui::Painter, center: Pos2, radius: f32, start: f32, end: f32, color: Color32) {
    if end <= start {
        return;
    }
    let steps = (((end - start) / 0.08) as usize).max(2);
    let mut points = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let a = start + (end - start) * i as f32 / steps as f32;
        points.push(Pos2::new(center.x + a.cos() * radius, center.y + a.sin() * radius));
    }
    painter.add(egui::Shape::line(points, Stroke::new(14.0, color)));
}
