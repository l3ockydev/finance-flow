//! Modal dialogs: income, expense, bills, export, detail, evidence, confirmations.

use crate::model::*;
use crate::theme::{ThemeColors, *};
use crate::{App, ExportFormat, Modal};
use base64::Engine;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};

pub fn dim(ctx: &egui::Context) {
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Middle, egui::Id::new("finflow_dim")));
    painter.rect_filled(ctx.screen_rect(), Rounding::ZERO, Color32::from_black_alpha(30));
}

fn modal_frame(tc: &ThemeColors) -> egui::Frame {
    egui::Frame::none()
        .fill(tc.surface)
        .rounding(Rounding::same(12.0))
        .stroke(Stroke::new(1.0_f32, tc.border))
        .inner_margin(egui::Margin::same(20.0))
}

fn modal_header(ui: &mut egui::Ui, icon: Icon, fg: Color32, bg: Color32, heading: &str, subtitle: &str, tc: &ThemeColors) -> bool {
    let mut close = false;
    ui.horizontal(|ui| {
        icon_badge(ui, icon, fg, bg, 34.0);
        ui.add_space(2.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(heading).size(16.0).color(tc.white).strong());
            ui.label(egui::RichText::new(subtitle).size(10.5).color(tc.muted));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            if icon_button(ui, Icon::Close, "Tutup", tc).clicked() {
                close = true;
            }
        });
    });
    ui.add_space(12.0);
    close
}

fn amount_field(ui: &mut egui::Ui, amount: &mut String, tc: &ThemeColors) {
    ui.label(egui::RichText::new("NOMINAL").size(10.5).color(tc.muted_dim).strong());
    ui.add_space(3.0);
    ui.horizontal(|ui| {
        egui::Frame::none()
            .fill(tc.surface_high)
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::symmetric(12.0, 9.0))
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Rp").size(13.0).color(tc.primary_light).strong());
            });
        ui.add(
            egui::TextEdit::singleline(amount)
                .hint_text(RichText::new("0").color(tc.muted_dim))
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Heading)
                .margin(Vec2::new(12.0, 8.0)),
        );
    });
    ui.add_space(6.0);
}

fn category_picker(ui: &mut egui::Ui, value: &mut String, options: &[&str], tc: &ThemeColors) {
    ui.label(egui::RichText::new("KATEGORI").size(10.5).color(tc.muted_dim).strong());
    ui.add_space(3.0);
    egui::ComboBox::from_id_source(format!("cat_{}", value))
        .selected_text(value.clone())
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            for option in options {
                ui.selectable_value(value, option.to_string(), *option);
            }
        });
}

fn account_picker(ui: &mut egui::Ui, value: &mut String, options: &[&str], tc: &ThemeColors) {
    ui.label(egui::RichText::new("AKUN BANK").size(10.5).color(tc.muted_dim).strong());
    ui.add_space(3.0);
    egui::ComboBox::from_id_source(format!("acc_{}", value))
        .selected_text(value.clone())
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            for option in options {
                ui.selectable_value(value, option.to_string(), *option);
            }
        });
}

fn date_field(ui: &mut egui::Ui, value: &mut String, tc: &ThemeColors) {
    ui.label(egui::RichText::new("TANGGAL").size(10.5).color(tc.muted_dim).strong());
    ui.add_space(3.0);
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(RichText::new("2026-11-22").color(tc.muted_dim))
            .desired_width(f32::INFINITY)
            .margin(Vec2::new(10.0, 8.0)),
    );
}

fn evidence_row(ui: &mut egui::Ui, app: &mut App) {
    let tc = app.theme_colors;
    ui.label(egui::RichText::new("BUKTI TRANSAKSI").size(10.5).color(tc.muted_dim).strong());
    ui.add_space(3.0);
    ui.horizontal(|ui| {
        if secondary_button(ui, "Lampirkan Gambar", &tc).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Gambar", &["png", "jpg", "jpeg", "webp"])
                .pick_file()
            {
                if let Ok(bytes) = std::fs::read(&path) {
                    if bytes.len() > 5 * 1024 * 1024 {
                        app.toast("Ukuran gambar melebihi 5 MB. Pilih file yang lebih kecil.");
                    } else {
                        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
                        app.form.image = Some(encoded);
                        app.form.image_name = path.file_name().map(|n| n.to_string_lossy().to_string());
                        app.form.image_mime = path
                            .extension()
                            .map(|e| format!("image/{}", e.to_string_lossy().to_lowercase()))
                            .or_else(|| Some("image/png".into()));
                    }
                }
            }
        }
        if app.form.image.is_some() {
            if ghost_button(ui, "Hapus bukti", &tc).clicked() {
                app.form.image = None;
                app.form.image_name = None;
                app.form.image_mime = None;
            }
            label(ui, app.form.image_name.as_deref().unwrap_or("Gambar terlampir"), 10.0, tc.income);
        } else {
            label(ui, "Opsional â€” struk, invoice, atau screenshot", 10.0, tc.muted_dim);
        }
    });
}

impl App {
    /// Category options for 0 = income, 1 = expense, 2 = bill.
    fn category_options(&self, kind: u8) -> Vec<String> {
        let list = self.current_project().map(|p| match kind {
            0 => p.income_categories.clone(),
            1 => p.expense_categories.clone(),
            _ => p.bill_categories.clone(),
        });
        match list {
            Some(items) if !items.is_empty() => items,
            _ => match kind {
                0 => default_income_categories(),
                1 => default_expense_categories(),
                _ => default_bill_categories(),
            },
        }
    }

    fn account_options(&self) -> Vec<String> {
        match self.current_project() {
            Some(p) if !p.bank_accounts.is_empty() => p.bank_accounts.clone(),
            _ => default_bank_accounts(),
        }
    }

    pub fn show_modal(&mut self, ctx: &egui::Context) {
        match self.modal {
            Modal::None => {}
            Modal::AddTransaction => self.add_transaction_modal(ctx),
            Modal::Income => self.entry_modal(ctx, TxKind::Income),
            Modal::Expense => self.entry_modal(ctx, TxKind::Expense),
            Modal::Bill => self.bill_modal(ctx),
            Modal::Export => self.export_modal(ctx),
            Modal::Detail => self.detail_modal(ctx),
            Modal::Evidence => self.evidence_modal(ctx),
            Modal::ConfirmDelete => self.confirm_delete_tx(ctx),
            Modal::NewProject => self.new_project_modal(ctx),
            Modal::ConfirmDeleteProject => self.confirm_delete_project(ctx),
        }
    }

    fn entry_modal(&mut self, ctx: &egui::Context, kind: TxKind) {
        dim(ctx);
        let tc = self.theme_colors;
        let (heading, subtitle, accent, accent_soft, icon) = match kind {
            TxKind::Income => (
                if self.editing_tx.is_some() { "Ubah Pemasukan" } else { "Catat Pemasukan Baru" },
                "Catat gaji, dividen, pelunasan proyek, atau pendapatan lain.",
                tc.income,
                tc.income_soft,
                Icon::ArrowDown,
            ),
            TxKind::Expense => (
                if self.editing_tx.is_some() { "Ubah Pengeluaran" } else { "Catat Pengeluaran Baru" },
                "Catat belanja, tagihan rutin, transportasi, dan operasional harian.",
                tc.expense,
                tc.expense_soft,
                Icon::ArrowUp,
            ),
        };

        let mut save = false;
        let mut close = false;
        egui::Window::new("entry_modal")
            .title_bar(false)
            .default_width(540.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                if modal_header(ui, icon, accent, accent_soft, heading, subtitle, &tc) {
                    close = true;
                }
                amount_field(ui, &mut self.form.amount, &tc);
                let cats = self.category_options(if kind == TxKind::Income { 0 } else { 1 });
                let cat_opts: Vec<&str> = cats.iter().map(|s| s.as_str()).collect();
                let accounts = self.account_options();
                let account_opts: Vec<&str> = accounts.iter().map(|s| s.as_str()).collect();
                ui.columns(2, |cols| {
                    category_picker(&mut cols[0], &mut self.form.category, &cat_opts, &tc);
                    cols[0].add_space(8.0);
                    date_field(&mut cols[0], &mut self.form.date, &tc);
                    account_picker(
                        &mut cols[1],
                        &mut self.form.account,
                        &account_opts,
                        &tc,
                    );
                    cols[1].add_space(8.0);
                    cols[1].label(egui::RichText::new("WAKTU").size(10.5).color(tc.muted_dim).strong());
                    cols[1].add_space(3.0);
                    cols[1].add(
                        egui::TextEdit::singleline(&mut self.form.time)
                            .hint_text("08:30")
                            .desired_width(f32::INFINITY)
                            .margin(Vec2::new(10.0, 8.0)),
                    );
                });
                ui.add_space(10.0);
                field(ui, "KETERANGAN", &mut self.form.note, "Contoh: Gaji bulanan November", &tc);
                evidence_row(ui, self);
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Batal", &tc).clicked() {
                        close = true;
                    }
                    let label_text = if self.editing_tx.is_some() { "Simpan Perubahan" } else { "Simpan Transaksi" };
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(label_text).size(13.0).color(tc.white).strong())
                                .fill(accent)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(0.0, 34.0)),
                        )
                        .clicked()
                    {
                        save = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        label(ui, "Tersimpan otomatis ke file .ff", 9.5, tc.muted_dim);
                    });
                });
            });

        if save {
            let amount = parse_amount(&self.form.amount);
            if amount > 0.0 {
                if self.editing_tx.is_some() {
                    self.apply_edit_transaction(kind);
                } else {
                    self.add_transaction(
                        kind,
                        amount,
                        self.form.category.clone(),
                        self.form.account.clone(),
                        self.form.note.clone(),
                        self.form.date.clone(),
                        self.form.time.clone(),
                        self.form.image.clone(),
                        self.form.image_mime.clone(),
                        self.form.image_name.clone(),
                    );
                }
                self.close_modal();
            } else {
                self.toast("Nominal harus lebih besar dari nol.");
            }
        } else if close {
            self.close_modal();
        }
    }

    fn bill_modal(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        let mut save = false;
        let mut close = false;
        egui::Window::new("bill_modal")
            .title_bar(false)
            .default_width(500.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                if modal_header(ui, Icon::Calendar, tc.warning, tc.warning_soft, "Tambah Tagihan", "Catat kewajiban rutin dan tanggal jatuh temponya.", &tc) {
                    close = true;
                }
                field(ui, "NAMA TAGIHAN", &mut self.form.name, "Contoh: IndiHome Fiber 100M", &tc);
                amount_field(ui, &mut self.form.amount, &tc);
                let cats = self.category_options(2);
                let cat_opts: Vec<&str> = cats.iter().map(|s| s.as_str()).collect();
                ui.columns(2, |cols| {
                    date_field(&mut cols[0], &mut self.form.due_date, &tc);
                    category_picker(&mut cols[1], &mut self.form.category, &cat_opts, &tc);
                });
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Batal", &tc).clicked() {
                        close = true;
                    }
                    if primary_button(ui, "Simpan Tagihan", &tc).clicked() {
                        save = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        label(ui, "Status awal: Menunggu", 9.5, tc.muted_dim);
                    });
                });
            });

        if save {
            let amount = parse_amount(&self.form.amount);
            if amount > 0.0 && !self.form.name.trim().is_empty() {
                self.add_bill(
                    self.form.name.clone(),
                    amount,
                    self.form.due_date.clone(),
                    self.form.category.clone(),
                    self.form.account.clone(),
                );
                self.close_modal();
            } else {
                self.toast("Nama dan nominal tagihan wajib diisi.");
            }
        } else if close {
            self.close_modal();
        }
    }

    fn export_modal(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        let Some(p) = self.current_project().cloned() else {
            self.close_modal();
            return;
        };
        let mut close = false;
        let mut export = false;
        let mut print = false;
        egui::Window::new("export_modal")
            .title_bar(false)
            .default_width(520.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                if modal_header(ui, Icon::Download, tc.primary_light, tc.primary_soft, "Ekspor & Cetak Laporan", "Pilih format berkas untuk kebutuhan arsip atau pencetakan.", &tc) {
                    close = true;
                }

                for (format, title, desc) in [
                    (ExportFormat::Csv, "CSV", "Tabel transaksi untuk Excel, Google Sheets, atau sistem lain."),
                    (ExportFormat::Excel, "Excel (.xls)", "Lembar kerja dengan header, tipe data, dan ringkasan."),
                    (ExportFormat::Pdf, "PDF", "Laporan siap cetak dengan ringkasan dan tabel transaksi."),
                ] {
                    let selected = self.form.format == format;
                    egui::Frame::none()
                        .fill(if selected { tc.primary_soft } else { tc.surface_low })
                        .rounding(Rounding::same(10.0))
                        .stroke(Stroke::new(1.0_f32, if selected { tc.primary } else { tc.border }))
                        .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                if ui.radio(selected, "").clicked() {
                                    self.form.format = format;
                                }
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(title).size(12.5).color(tc.text).strong());
                                    ui.label(egui::RichText::new(desc).size(10.0).color(tc.muted));
                                });
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    label(ui, &format!("{} transaksi", p.transactions.len()), 9.5, tc.muted_dim);
                                });
                            });
                        });
                    ui.add_space(6.0);
                }

                ui.add_space(6.0);
                soft_panel(ui, |ui| {
                    ui.horizontal(|ui| {
                        label(ui, "Total pemasukan", 10.5, tc.muted);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| label(ui, &format_rp(p.total_income()), 11.0, tc.income));
                    });
                    ui.horizontal(|ui| {
                        label(ui, "Total pengeluaran", 10.5, tc.muted);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| label(ui, &format_rp(p.total_expense()), 11.0, tc.expense));
                    });
                    ui.horizontal(|ui| {
                        label(ui, "Saldo bersih", 10.5, tc.muted);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| label(ui, &format_rp(p.balance()), 11.0, tc.text));
                    });
                }, &tc);
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Batal", &tc).clicked() {
                        close = true;
                    }
                    if secondary_button(ui, "Cetak Langsung", &tc).clicked() {
                        print = true;
                    }
                    if primary_button(ui, "Ekspor Sekarang", &tc).clicked() {
                        export = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        label(ui, "Tanpa internet", 9.5, tc.muted_dim);
                    });
                });
            });

        if print {
            match print_pdf(&p) {
                Ok(()) => self.toast("Laporan PDF dikirim ke dialog cetak Windows."),
                Err(err) => self.toast(&format!("Gagal mencetak: {}", err)),
            }
            self.close_modal();
        } else if export {
            self.run_export(&p);
        } else if close {
            self.close_modal();
        }
    }

    fn detail_modal(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        let Some(id) = self.detail_tx.clone() else {
            self.close_modal();
            return;
        };
        let Some(tx) = self
            .current_project()
            .and_then(|p| p.transactions.iter().find(|t| t.id == id).cloned())
        else {
            self.close_modal();
            return;
        };
        let mut close = false;
        let mut edit = false;
        let mut delete = false;
        let mut show_evidence = false;
        egui::Window::new("detail_modal")
            .title_bar(false)
            .default_width(520.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                let (accent, soft, icon) = if tx.kind == TxKind::Income {
                    (tc.income, tc.income_soft, Icon::ArrowDown)
                } else {
                    (tc.expense, tc.expense_soft, Icon::ArrowUp)
                };
                if modal_header(ui, icon, accent, soft, "Detail Transaksi", &format!("{} â€¢ {}", tx.kind.label(), format_date_long(&tx.date)), &tc) {
                    close = true;
                }
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("NOMINAL").size(9.5).color(tc.muted).strong());
                        ui.label(egui::RichText::new(format_signed(tx.kind, tx.amount)).size(30.0).color(accent).strong());
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        pill(ui, &tx.category, category_color(&tx.category, &tc), category_bg(&tx.category, &tc));
                    });
                });
                ui.add_space(12.0);
                divider(ui, &tc);
                ui.add_space(12.0);
                egui::Grid::new("detail_grid").num_columns(2).spacing([24.0, 10.0]).show(ui, |ui| {
                    for (key, value) in [
                        ("Keterangan", tx.note.clone()),
                        ("Rekening", tx.account.clone()),
                        ("Waktu", format!("{} WIB", tx.time)),
                        ("Status", "Selesai".to_string()),
                    ] {
                        ui.label(egui::RichText::new(key).size(10.0).color(tc.muted));
                        ui.label(egui::RichText::new(value).size(11.5).color(tc.text));
                        ui.end_row();
                    }
                });
                ui.add_space(12.0);
                soft_panel(ui, |ui| {
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, Rounding::same(7.0), tc.surface_high);
                        paint_icon(ui.painter(), rect.shrink(7.0), tc.primary_light, Icon::Doc, 1.4);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(if tx.image.is_some() { "Bukti transaksi terlampir" } else { "Belum ada bukti transaksi" }).size(11.0).color(tc.text));
                            ui.label(egui::RichText::new(tx.image_name.as_deref().unwrap_or("Lampirkan struk atau invoice saat mengubah transaksi")).size(9.5).color(tc.muted));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if tx.image.is_some() && secondary_button(ui, "Lihat Bukti", &tc).clicked() {
                                show_evidence = true;
                            }
                        });
                    });
                }, &tc);
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Tutup", &tc).clicked() {
                        close = true;
                    }
                    if secondary_button(ui, "Hapus", &tc).clicked() {
                        delete = true;
                    }
                    if primary_button(ui, "Ubah Transaksi", &tc).clicked() {
                        edit = true;
                    }
                });
            });

        if show_evidence {
            self.modal = Modal::Evidence;
        } else if edit {
            self.open_edit_transaction(&tx.id);
        } else if delete {
            self.pending_delete = Some(tx.id.clone());
            self.modal = Modal::ConfirmDelete;
        } else if close {
            self.close_modal();
        }
    }

    fn evidence_modal(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        let Some(id) = self.detail_tx.clone() else {
            self.close_modal();
            return;
        };
        let Some(tx) = self
            .current_project()
            .and_then(|p| p.transactions.iter().find(|t| t.id == id).cloned())
        else {
            self.close_modal();
            return;
        };
        let mut close = false;
        egui::Window::new("evidence_modal")
            .title_bar(false)
            .default_width(640.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                if modal_header(ui, Icon::Doc, tc.primary_light, tc.primary_soft, "Bukti Transaksi", tx.image_name.as_deref().unwrap_or("Gambar terlampir"), &tc) {
                    close = true;
                }
                if let Some(encoded) = &tx.image {
                    self.ensure_evidence_texture(ctx, &tx.id, encoded);
                    if let Some((_, texture)) = &self.evidence_tex {
                        let size = texture.size_vec2();
                        let scale = (600.0 / size.x).min(420.0 / size.y).min(1.0);
                        ui.vertical_centered(|ui| {
                            ui.add(egui::Image::new((texture.id(), size * scale)).rounding(Rounding::same(10.0)));
                        });
                    }
                } else {
                    label(ui, "Tidak ada gambar pada transaksi ini.", 11.0, tc.muted);
                }
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Tutup", &tc).clicked() {
                        close = true;
                    }
                    if secondary_button(ui, "Kembali ke Detail", &tc).clicked() {
                        self.modal = Modal::Detail;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        label(ui, "Disimpan di dalam file .ff", 9.5, tc.muted_dim);
                    });
                });
            });
        if close {
            self.modal = Modal::Detail;
        }
    }

    fn confirm_delete_tx(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        let mut close = false;
        let mut confirm = false;
        egui::Window::new("confirm_delete")
            .title_bar(false)
            .default_width(420.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                if modal_header(ui, Icon::Trash, tc.expense, tc.expense_soft, "Hapus Transaksi?", "Tindakan ini tidak dapat dibatalkan.", &tc) {
                    close = true;
                }
                label(ui, "Transaksi akan dihapus dari proyek dan file .ff akan diperbarui segera.", 11.0, tc.muted);
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Batal", &tc).clicked() {
                        close = true;
                    }
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("Ya, Hapus").size(13.0).color(tc.white).strong())
                                .fill(tc.expense)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(0.0, 34.0)),
                        )
                        .clicked()
                    {
                        confirm = true;
                    }
                });
            });
        if confirm {
            if let Some(id) = self.pending_delete.take() {
                self.delete_transaction(&id);
            }
            self.close_modal();
        } else if close {
            self.pending_delete = None;
            self.close_modal();
        }
    }

    fn new_project_modal(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        let mut close = false;
        let mut create = false;
        egui::Window::new("new_project")
            .title_bar(false)
            .default_width(480.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                if modal_header(ui, Icon::Doc, tc.primary_light, tc.primary_soft, "Proyek Keuangan Baru", "Laporan baru disimpan sebagai file .ff terpisah.", &tc) {
                    close = true;
                }
                field(ui, "NAMA PROYEK", &mut self.form.name, "Contoh: Keuangan Keluarga 2026", &tc);
                field(ui, "PEMILIK / PEMBUAT", &mut self.form.owner, "Nama lengkap", &tc);
                ui.add_space(4.0);
                soft_panel(ui, |ui| {
                    ui.label(egui::RichText::new("Proyek baru dimulai tanpa transaksi, tagihan, atau gambar dari proyek lain.").size(10.0).color(tc.muted));
                }, &tc);
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Batal", &tc).clicked() {
                        close = true;
                    }
                    if primary_button(ui, "Buat Proyek", &tc).clicked() {
                        create = true;
                    }
                });
            });
        if create {
            let name = if self.form.name.trim().is_empty() { "Proyek Keuangan Baru".to_string() } else { self.form.name.clone() };
            let owner = self.form.owner.clone();
            self.create_project(name, owner);
            self.close_modal();
        } else if close {
            self.close_modal();
        }
    }

    fn confirm_delete_project(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        let mut close = false;
        let mut confirm = false;
        egui::Window::new("confirm_delete_project")
            .title_bar(false)
            .default_width(430.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                if modal_header(ui, Icon::Trash, tc.expense, tc.expense_soft, "Hapus Proyek Ini?", "File .ff akan dihapus permanen dari disk.", &tc) {
                    close = true;
                }
                label(ui, "Seluruh transaksi, tagihan, dan bukti gambar pada proyek ini akan hilang. Ekspor cadangan terlebih dahulu bila diperlukan.", 11.0, tc.muted);
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Batal", &tc).clicked() {
                        close = true;
                    }
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("Hapus Proyek").size(13.0).color(tc.white).strong())
                                .fill(tc.expense)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(0.0, 34.0)),
                        )
                        .clicked()
                    {
                        confirm = true;
                    }
                });
            });
        if confirm {
            self.delete_current_project();
            self.close_modal();
        } else if close {
            self.close_modal();
        }
    }

    fn add_transaction_modal(&mut self, ctx: &egui::Context) {
        dim(ctx);
        let tc = self.theme_colors;
        #[derive(Clone, Copy, PartialEq)]
        enum Tab { Income, Expense, Bill }
        let mut tab = match self.add_tx_tab {
            1 => Tab::Expense,
            2 => Tab::Bill,
            _ => Tab::Income,
        };
        let mut save = false;
        let mut close = false;

        egui::Window::new("add_transaction_modal")
            .title_bar(false)
            .default_width(580.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .frame(modal_frame(&tc))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let tabs = [
                        (Tab::Income, "Pemasukan", tc.income, tc.income_soft, Icon::ArrowDown),
                        (Tab::Expense, "Pengeluaran", tc.expense, tc.expense_soft, Icon::ArrowUp),
                        (Tab::Bill, "Tagihan", tc.warning, tc.warning_soft, Icon::Bell),
                    ];
                    for (t, label, fg, bg, icon) in tabs {
                        let selected = tab == t;
                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), egui::Sense::hover());
                        let resp = ui
                            .add(egui::Button::new({
                                paint_icon(ui.painter(), rect.shrink(4.0), if selected { tc.white } else { fg }, icon, 1.2);
                                egui::RichText::new(label).size(11.5).color(if selected { tc.white } else { fg }).strong()
                            })
                            .fill(if selected { fg } else { bg })
                            .rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(110.0, 34.0)));
                        if resp.clicked() {
                            tab = t;
                            self.add_tx_tab = match t {
                                Tab::Income => 0,
                                Tab::Expense => 1,
                                Tab::Bill => 2,
                            };
                            match t {
                                Tab::Income => {
                                    self.form.category = self.category_options(0).first().cloned().unwrap_or_else(|| "Lainnya".into());
                                    self.form.name.clear();
                                }
                                Tab::Expense => {
                                    self.form.category = self.category_options(1).first().cloned().unwrap_or_else(|| "Lainnya".into());
                                    self.form.name.clear();
                                }
                                Tab::Bill => {
                                    self.form.category = self.category_options(2).first().cloned().unwrap_or_else(|| "Lainnya".into());
                                    self.form.name.clear();
                                    self.form.due_date = shift_iso(&today_iso(), 7);
                                }
                            }
                        }
                    }
                });
                ui.add_space(16.0);

                match tab {
                    Tab::Income => {
                        let (heading, subtitle, accent, accent_soft, icon) = (
                            if self.editing_tx.is_some() { "Ubah Pemasukan" } else { "Catat Pemasukan Baru" },
                            "Catat gaji, dividen, pelunasan proyek, atau pendapatan lain.",
                            tc.income, tc.income_soft, Icon::ArrowDown,
                        );
                        if modal_header(ui, icon, accent, accent_soft, heading, subtitle, &tc) {
                            close = true;
                        }
                        amount_field(ui, &mut self.form.amount, &tc);
                        let cats = self.category_options(0);
                        let cat_opts: Vec<&str> = cats.iter().map(|s| s.as_str()).collect();
                        let accounts = self.account_options();
                        let account_opts: Vec<&str> = accounts.iter().map(|s| s.as_str()).collect();
                        ui.columns(2, |cols| {
                            category_picker(
                                &mut cols[0],
                                &mut self.form.category,
                                &cat_opts,
                                &tc
                            );
                            cols[0].add_space(8.0);
                            date_field(&mut cols[0], &mut self.form.date, &tc);
                            account_picker(
                                &mut cols[1],
                                &mut self.form.account,
                                &account_opts,
                                &tc
                            );
                            cols[1].add_space(8.0);
                            cols[1].label(egui::RichText::new("WAKTU").size(10.5).color(tc.muted_dim).strong());
                            cols[1].add_space(3.0);
                            cols[1].add(
                                egui::TextEdit::singleline(&mut self.form.time)
                                    .hint_text("08:30")
                                    .desired_width(f32::INFINITY)
                                    .margin(Vec2::new(10.0, 8.0)),
                            );
                        });
                        ui.add_space(10.0);
                        field(ui, "KETERANGAN", &mut self.form.note, "Contoh: Gaji bulanan November", &tc);
                        evidence_row(ui, self);
                    }
                    Tab::Expense => {
                        let (heading, subtitle, accent, accent_soft, icon) = (
                            if self.editing_tx.is_some() { "Ubah Pengeluaran" } else { "Catat Pengeluaran Baru" },
                            "Catat belanja, tagihan rutin, transportasi, dan operasional harian.",
                            tc.expense, tc.expense_soft, Icon::ArrowUp,
                        );
                        if modal_header(ui, icon, accent, accent_soft, heading, subtitle, &tc) {
                            close = true;
                        }
                        amount_field(ui, &mut self.form.amount, &tc);
                        let cats = self.category_options(1);
                        let cat_opts: Vec<&str> = cats.iter().map(|s| s.as_str()).collect();
                        let accounts = self.account_options();
                        let account_opts: Vec<&str> = accounts.iter().map(|s| s.as_str()).collect();
                        ui.columns(2, |cols| {
                            category_picker(
                                &mut cols[0],
                                &mut self.form.category,
                                &cat_opts,
                                &tc
                            );
                            cols[0].add_space(8.0);
                            date_field(&mut cols[0], &mut self.form.date, &tc);
                            account_picker(
                                &mut cols[1],
                                &mut self.form.account,
                                &account_opts,
                                &tc
                            );
                            cols[1].add_space(8.0);
                            cols[1].label(egui::RichText::new("WAKTU").size(10.5).color(tc.muted_dim).strong());
                            cols[1].add_space(3.0);
                            cols[1].add(
                                egui::TextEdit::singleline(&mut self.form.time)
                                    .hint_text("08:30")
                                    .desired_width(f32::INFINITY)
                                    .margin(Vec2::new(10.0, 8.0)),
                            );
                        });
                        ui.add_space(10.0);
                        field(ui, "KETERANGAN", &mut self.form.note, "Contoh: Belanja mingguan", &tc);
                        evidence_row(ui, self);
                    }
                    Tab::Bill => {
                        let (heading, subtitle, accent, accent_soft, icon) = (
                            "Tambah Tagihan",
                            "Catat kewajiban rutin dan tanggal jatuh temponya.",
                            tc.warning, tc.warning_soft, Icon::Bell,
                        );
                        if modal_header(ui, icon, accent, accent_soft, heading, subtitle, &tc) {
                            close = true;
                        }
                        field(ui, "NAMA TAGIHAN", &mut self.form.name, "Contoh: IndiHome Fiber 100M", &tc);
                        amount_field(ui, &mut self.form.amount, &tc);
                        let cats = self.category_options(2);
                        let cat_opts: Vec<&str> = cats.iter().map(|s| s.as_str()).collect();
                        ui.columns(2, |cols| {
                            date_field(&mut cols[0], &mut self.form.due_date, &tc);
                            category_picker(&mut cols[1], &mut self.form.category, &cat_opts, &tc);
                        });
                        ui.add_space(10.0);
                        let accounts = self.account_options();
                        let account_opts: Vec<&str> = accounts.iter().map(|s| s.as_str()).collect();
                        account_picker(
                            ui,
                            &mut self.form.account,
                            &account_opts,
                            &tc
                        );
                        ui.add_space(10.0);
                        field(ui, "KETERANGAN", &mut self.form.note, "Contoh: Tagihan bulanan", &tc);
                        evidence_row(ui, self);
                    }
                }

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Batal", &tc).clicked() {
                        close = true;
                    }
                    let label_text = match tab {
                        Tab::Income | Tab::Expense => {
                            if self.editing_tx.is_some() { "Simpan Perubahan" } else { "Simpan Transaksi" }
                        }
                        Tab::Bill => "Simpan Tagihan",
                    };
                    let accent = match tab {
                        Tab::Income => tc.income,
                        Tab::Expense => tc.expense,
                        Tab::Bill => tc.warning,
                    };
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(label_text).size(13.0).color(tc.white).strong())
                                .fill(accent)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(0.0, 34.0)),
                        )
                        .clicked()
                    {
                        save = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        label(ui, "Tersimpan otomatis ke file .ff", 9.5, tc.muted_dim);
                    });
                });
            });

        if save {
            match tab {
                Tab::Income | Tab::Expense => {
                    let kind = if tab == Tab::Income { TxKind::Income } else { TxKind::Expense };
                    let amount = parse_amount(&self.form.amount);
                    if amount > 0.0 {
                        if self.editing_tx.is_some() {
                            self.apply_edit_transaction(kind);
                        } else {
                            self.add_transaction(
                                kind,
                                amount,
                                self.form.category.clone(),
                                self.form.account.clone(),
                                self.form.note.clone(),
                                self.form.date.clone(),
                                self.form.time.clone(),
                                self.form.image.clone(),
                                self.form.image_mime.clone(),
                                self.form.image_name.clone(),
                            );
                        }
                        self.close_modal();
                    } else {
                        self.toast("Nominal harus lebih besar dari nol.");
                    }
                }
                Tab::Bill => {
                    let amount = parse_amount(&self.form.amount);
                    if amount > 0.0 && !self.form.name.trim().is_empty() {
                        self.add_bill(
                            self.form.name.clone(),
                            amount,
                            self.form.due_date.clone(),
                            self.form.category.clone(),
                            self.form.account.clone(),
                        );
                        self.close_modal();
                    } else {
                        self.toast("Nama dan nominal tagihan wajib diisi.");
                    }
                }
            }
        } else if close {
            self.close_modal();
        }
    }
}
