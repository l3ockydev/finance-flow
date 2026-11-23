//! FinFlow — desktop personal finance manager (Rust + egui).
//! Offline-first: every project is one JSON `.ff` document stored on disk.

// Sembunyikan console window saat dijalankan dari Explorer / shortcut.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod modals;
mod model;
mod theme;
mod views;
mod views_pages;

use base64::Engine;
use eframe::egui::{self, Color32, FontId, Rounding, Sense, Stroke, Vec2};
use model::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use theme::*;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Transactions,
    Bills,
    Analysis,
    Settings,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Modal {
    None,
    AddTransaction,
    Income,
    Expense,
    Bill,
    Export,
    Detail,
    Evidence,
    ConfirmDelete,
    NewProject,
    ConfirmDeleteProject,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Csv,
    Excel,
    Pdf,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SettingsSection {
    Payments,
    Expenses,
    Income,
    Bills,
    Default,
    Privacy,
}

impl SettingsSection {
    pub fn label(self) -> &'static str {
        match self {
            SettingsSection::Payments => "Pembayaran",
            SettingsSection::Expenses => "Pengeluaran",
            SettingsSection::Income => "Pemasukan",
            SettingsSection::Bills => "Tagihan",
            SettingsSection::Default => "Default",
            SettingsSection::Privacy => "Privasi & Update",
        }
    }
}

/// Preferensi aplikasi yang tidak ikut masuk ke file `.ff` proyek.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub check_updates: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { check_updates: true }
    }
}

impl AppSettings {
    fn path() -> PathBuf {
        let base = std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(".").join(".finflow"));
        base.join("FinFlow").join("settings.json")
    }

    fn load() -> Self {
        fs::read_to_string(Self::path())
            .ok()
            .and_then(|content| serde_json::from_str::<Self>(&content).ok())
            .unwrap_or_default()
    }

    fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let temp = path.with_extension("json.tmp");
            if fs::write(&temp, json).is_ok() {
                let _ = fs::rename(&temp, &path);
            }
        }
    }
}

#[derive(Clone)]
pub struct UndoDelete {
    pub project_id: String,
    pub index: usize,
    pub transaction: Transaction,
}

#[derive(Clone)]
pub struct Toast {
    pub message: String,
    pub created: Instant,
    pub undo: Option<UndoDelete>,
}

#[derive(Clone)]
pub struct Form {
    pub amount: String,
    pub category: String,
    pub account: String,
    pub note: String,
    pub date: String,
    pub time: String,
    pub name: String,
    pub owner: String,
    pub due_date: String,
    pub image: Option<String>,
    pub image_mime: Option<String>,
    pub image_name: Option<String>,
    pub format: ExportFormat,
    pub bank_name: String,
    pub settings_item: String,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            amount: String::new(),
            category: "Gaji & Bonus".to_string(),
            account: "BCA Prioritas".to_string(),
            note: String::new(),
            date: today_iso(),
            time: "09:00".to_string(),
            name: String::new(),
            owner: String::new(),
            due_date: shift_iso(&today_iso(), 7),
            image: None,
            image_mime: None,
            image_name: None,
            format: ExportFormat::Pdf,
            bank_name: String::new(),
            settings_item: String::new(),
        }
    }
}

impl Form {
    fn reset_entry(&mut self, kind: TxKind) {
        self.amount.clear();
        self.note.clear();
        self.date = today_iso();
        self.time = "09:00".to_string();
        self.category = match kind {
            TxKind::Income => "Gaji & Bonus".to_string(),
            TxKind::Expense => "Kebutuhan Pokok".to_string(),
        };
        self.account = "BCA Prioritas".to_string();
        self.image = None;
        self.image_mime = None;
        self.image_name = None;
    }

    fn reset_bill(&mut self) {
        self.name.clear();
        self.amount.clear();
        self.due_date = shift_iso(&today_iso(), 7);
        self.category = "Internet".to_string();
    }
}

pub struct App {
    pub projects: Vec<Project>,
    pub current: Option<String>,
    pub view: View,
    pub dir: PathBuf,
    pub modal: Modal,
    pub form: Form,
    pub filter_kind: Option<TxKind>,
    pub filter_category: Option<String>,
    pub search: String,
    pub page: usize,
    pub pending_delete: Option<String>,
    pub detail_tx: Option<String>,
    pub editing_tx: Option<String>,
    pub evidence_tex: Option<(String, egui::TextureHandle)>,
    pub toast_message: Option<Toast>,
    pub settings: AppSettings,
    pub add_tx_tab: u8,
    pub settings_section: SettingsSection,
    pub settings_expanded: bool,
    pub theme_colors: ThemeColors,
    pub logo_tex: Option<egui::TextureHandle>,
}

impl App {
    pub fn new() -> Self {
        let dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("projects");
        let _ = fs::create_dir_all(&dir);
        let settings = AppSettings::load();
        if settings.check_updates {
            spawn_auto_updater();
        }
        let mut app = Self {
            projects: Vec::new(),
            current: None,
            view: View::Dashboard,
            dir,
            modal: Modal::None,
            form: Form::default(),
            filter_kind: None,
            filter_category: None,
            search: String::new(),
            page: 0,
            pending_delete: None,
            detail_tx: None,
            editing_tx: None,
            evidence_tex: None,
            toast_message: None,
            settings,
            add_tx_tab: 0,
            settings_section: SettingsSection::Default,
            settings_expanded: false,
            theme_colors: ThemeColors::default(),
            logo_tex: None,
        };
        app.load_projects();
        if app.projects.is_empty() {
            // Mulai dengan proyek kosong — tidak ada transaksi atau tagihan contoh.
            let mut project = Project::blank(1);
            project.title = "Keuangan Pribadi".to_string();
            app.save_project(&project);
            app.current = Some(project.id.clone());
            app.projects.push(project);
        } else {
            app.projects.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            app.current = Some(app.projects[0].id.clone());
        }
        if let Some(version) = update_notice() {
            app.toast(&format!("FinFlow diperbarui ke v{version}."));
        }
        app
    }

    // -- persistence --------------------------------------------------------

    fn load_projects(&mut self) {
        self.projects.clear();
        let Ok(entries) = fs::read_dir(&self.dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("ff") {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(project) = serde_json::from_str::<Project>(&content) {
                    self.projects.push(project);
                }
            }
        }
    }

    fn project_path(&self, project: &Project) -> PathBuf {
        self.dir.join(format!("{}.ff", slug(&project.id)))
    }

    /// Backup disimpan terpisah dari `projects/` agar data utama tidak pernah
    /// dihapus atau ditimpa oleh proses pembersihan otomatis.
    fn backup_root() -> PathBuf {
        let base = std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(".").join(".finflow"));
        base.join("FinFlow").join("backups")
    }

    fn backup_dir(&self, project: &Project) -> PathBuf {
        Self::backup_root().join(slug(&project.id))
    }

    fn create_backup(&self, path: &Path, project: &Project) {
        let Ok(bytes) = fs::read(path) else {
            return;
        };
        let dir = self.backup_dir(project);
        if fs::create_dir_all(&dir).is_err() {
            return;
        }
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let _ = fs::write(dir.join(format!("{}.ff", stamp)), bytes);
        self.prune_backups(&dir);
    }

    fn prune_backups(&self, dir: &Path) {
        const MAX: usize = 20;
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let mut files: Vec<_> = entries
            .flatten()
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("ff"))
            .collect();
        if files.len() <= MAX {
            return;
        }
        files.sort_by_key(|e| e.file_name());
        let remove_count = files.len() - MAX;
        for old in files.into_iter().take(remove_count) {
            let _ = fs::remove_file(old.path());
        }
    }

    pub fn save_project(&self, project: &Project) {
        let _ = fs::create_dir_all(&self.dir);
        if let Ok(json) = serde_json::to_string_pretty(project) {
            let path = self.project_path(project);
            if path.exists() {
                self.create_backup(&path, project);
            }
            let temp = path.with_extension("ff.tmp");
            if fs::write(&temp, json).is_ok() {
                let _ = fs::remove_file(&path);
                let _ = fs::rename(&temp, &path);
            }
        }
    }

    pub fn save_current(&mut self) {
        if let Some(project) = self.current_project().cloned() {
            self.save_project(&project);
        }
    }

    pub fn current_project(&self) -> Option<&Project> {
        let id = self.current.as_ref()?;
        self.projects.iter().find(|p| &p.id == id)
    }

    pub fn current_project_mut(&mut self) -> Option<&mut Project> {
        let id = self.current.clone()?;
        self.projects.iter_mut().find(|p| p.id == id)
    }

    fn commit(&mut self, mutate: impl FnOnce(&mut Project)) {
        let mut updated = None;
        if let Some(project) = self.current_project_mut() {
            mutate(project);
            project.touch();
            updated = Some(project.clone());
        }
        if let Some(project) = updated {
            self.save_project(&project);
        }
    }

    // -- project actions ----------------------------------------------------

    pub fn create_project(&mut self, title: String, owner: String) {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let mut project = Project::blank(self.projects.len() + 1);
        project.id = format!("proj_{}", stamp);
        project.title = title;
        project.owner = owner;
        project.touch();
        self.save_project(&project);
        self.current = Some(project.id.clone());
        self.projects.push(project);
        self.view = View::Dashboard;
        self.toast("Proyek baru dibuat dan disimpan sebagai file .ff.");
    }

    pub fn delete_current_project(&mut self) {
        let Some(id) = self.current.clone() else {
            return;
        };
        if let Some(project) = self.projects.iter().find(|p| p.id == id) {
            let _ = fs::remove_file(self.project_path(project));
        }
        self.projects.retain(|p| p.id != id);
        self.current = self.projects.first().map(|p| p.id.clone());
        self.view = View::Dashboard;
        self.toast("Proyek dihapus dari disk.");
    }

    pub fn save_profile(&mut self, title: String, owner: String) {
        if title.trim().is_empty() {
            self.toast("Nama proyek tidak boleh kosong.");
            return;
        }
        self.commit(|project| {
            project.title = title;
            project.owner = owner;
        });
        self.toast("Profil proyek diperbarui.");
    }

    pub fn open_project_dialog(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("FinFlow Project", &["ff"])
            .pick_file()
        else {
            return;
        };
        self.import_project(&path);
    }

    /// Pilih salinan otomatis dari folder backup lokal, lalu pulihkan sebagai proyek aktif.
    pub fn restore_backup_dialog(&mut self) {
        let root = Self::backup_root();
        let _ = fs::create_dir_all(&root);
        let Some(path) = rfd::FileDialog::new()
            .set_directory(root)
            .add_filter("FinFlow Backup", &["ff"])
            .pick_file()
        else {
            return;
        };
        self.import_project(&path);
        self.toast("Backup dipulihkan. Periksa kembali isi proyek Anda.");
    }

    fn import_project(&mut self, path: &Path) {
        if let Ok(meta) = fs::metadata(path) {
            if meta.len() > 50 * 1024 * 1024 {
                self.toast("File .ff melebihi 50 MB dan ditolak demi keamanan.");
                return;
            }
        }
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => {
                self.toast(&format!("Gagal membaca file: {}", err));
                return;
            }
        };
        match serde_json::from_str::<Project>(&content) {
            Ok(project) => {
                if let Some(existing) = self.projects.iter_mut().find(|p| p.id == project.id) {
                    *existing = project.clone();
                } else {
                    self.projects.push(project.clone());
                }
                self.save_project(&project);
                self.current = Some(project.id.clone());
                self.view = View::Dashboard;
                self.toast(&format!("Proyek '{}' dibuka.", project.title));
            }
            Err(err) => self.toast(&format!("File .ff tidak valid: {}", err)),
        }
    }

    pub fn save_copy_dialog(&mut self) {
        let Some(project) = self.current_project().cloned() else {
            return;
        };
        let Some(path) = rfd::FileDialog::new()
            .add_filter("FinFlow Project", &["ff"])
            .set_file_name(format!("{}.ff", slug(&project.title)))
            .save_file()
        else {
            return;
        };
        let path = if path.extension().is_none() {
            path.with_extension("ff")
        } else {
            path
        };
        match serde_json::to_string_pretty(&project).map_err(|e| e.to_string()).and_then(|json| {
            fs::write(&path, json).map_err(|e| e.to_string())
        }) {
            Ok(()) => self.toast("Salinan proyek disimpan."),
            Err(err) => self.toast(&format!("Gagal menyimpan salinan: {}", err)),
        }
    }

    pub fn open_projects_folder(&mut self) {
        let _ = fs::create_dir_all(&self.dir);
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("explorer").arg(&self.dir).spawn();
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = std::process::Command::new("xdg-open").arg(&self.dir).spawn();
        }
        self.toast("Folder proyek dibuka.");
    }

    // -- transaction actions ------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn add_transaction(
        &mut self,
        kind: TxKind,
        amount: f64,
        category: String,
        account: String,
        note: String,
        date: String,
        time: String,
        image: Option<String>,
        image_mime: Option<String>,
        image_name: Option<String>,
    ) {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let note = if note.trim().is_empty() {
            category.clone()
        } else {
            note
        };
        self.commit(|project| {
            project.transactions.push(Transaction {
                id: format!("trx_{}", stamp),
                date,
                time,
                amount,
                category,
                account,
                note,
                kind,
                image,
                image_mime,
                image_name,
            });
        });
        self.toast("Transaksi tersimpan ke file proyek.");
    }

    pub fn open_edit_transaction(&mut self, id: &str) {
        let Some(tx) = self
            .current_project()
            .and_then(|p| p.transactions.iter().find(|t| t.id == id))
            .cloned()
        else {
            return;
        };
        self.form.amount = format!("{:.0}", tx.amount);
        self.form.category = tx.category.clone();
        self.form.account = tx.account.clone();
        self.form.note = tx.note.clone();
        self.form.date = tx.date.clone();
        self.form.time = tx.time.clone();
        self.form.image = tx.image.clone();
        self.form.image_mime = tx.image_mime.clone();
        self.form.image_name = tx.image_name.clone();
        self.editing_tx = Some(tx.id.clone());
        self.modal = match tx.kind {
            TxKind::Income => Modal::Income,
            TxKind::Expense => Modal::Expense,
        };
    }

    pub fn apply_edit_transaction(&mut self, kind: TxKind) {
        let Some(id) = self.editing_tx.clone() else {
            return;
        };
        let amount = parse_amount(&self.form.amount);
        if amount <= 0.0 {
            self.toast("Nominal harus lebih besar dari nol.");
            return;
        }
        let form = self.form.clone();
        let mut updated = None;
        if let Some(project) = self.current_project_mut() {
            if let Some(tx) = project.transactions.iter_mut().find(|t| t.id == id) {
                tx.amount = amount;
                tx.category = form.category.clone();
                tx.account = form.account.clone();
                tx.note = if form.note.trim().is_empty() { form.category.clone() } else { form.note.clone() };
                tx.date = form.date.clone();
                tx.time = form.time.clone();
                tx.kind = kind;
                tx.image = form.image.clone();
                tx.image_mime = form.image_mime.clone();
                tx.image_name = form.image_name.clone();
            }
            project.touch();
            updated = Some(project.clone());
        }
        if let Some(project) = updated {
            self.save_project(&project);
        }
        self.editing_tx = None;
        self.toast("Perubahan transaksi disimpan.");
    }

    pub fn delete_transaction(&mut self, id: &str) {
        let undo = self.current_project().and_then(|project| {
            let index = project.transactions.iter().position(|t| t.id == id)?;
            Some(UndoDelete {
                project_id: project.id.clone(),
                index,
                transaction: project.transactions[index].clone(),
            })
        });
        self.commit(|project| project.transactions.retain(|t| t.id != id));
        self.detail_tx = None;
        if let Some(undo) = undo {
            self.toast_with_undo("Transaksi dihapus.", undo);
        } else {
            self.toast("Transaksi dihapus.");
        }
    }

    // -- bill actions -------------------------------------------------------

    pub fn add_bill(&mut self, name: String, amount: f64, due_date: String, category: String, account: String) {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        self.commit(|project| {
            project.bills.push(Bill {
                id: format!("bill_{}", stamp),
                name,
                amount,
                due_date,
                category,
                status: BillStatus::Pending,
                account,
            });
        });
        self.toast("Tagihan ditambahkan.");
    }

    pub fn mark_bill_paid(&mut self, id: &str) {
        self.commit(|project| {
            if let Some(bill) = project.bills.iter_mut().find(|b| b.id == id) {
                bill.status = BillStatus::Paid;
            }
        });
        self.toast("Tagihan ditandai lunas.");
    }

    pub fn delete_bill(&mut self, id: &str) {
        self.commit(|project| project.bills.retain(|b| b.id != id));
        self.toast("Tagihan dihapus.");
    }

    // -- modal & toast ------------------------------------------------------

    pub fn open_modal(&mut self, modal: Modal) {
        match modal {
            Modal::AddTransaction => {
                self.editing_tx = None;
                self.add_tx_tab = 0;
                self.form.reset_entry(TxKind::Income);
            }
            Modal::Income => {
                self.form.reset_entry(TxKind::Income);
                self.editing_tx = None;
            }
            Modal::Expense => {
                self.form.reset_entry(TxKind::Expense);
                self.editing_tx = None;
            }
            Modal::Bill => self.form.reset_bill(),
            Modal::NewProject => {
                self.form.name.clear();
                self.form.owner = self.current_project().map(|p| p.owner.clone()).unwrap_or_default();
            }
            _ => {}
        }
        self.modal = modal;
    }

    pub fn close_modal(&mut self) {
        self.modal = Modal::None;
        self.editing_tx = None;
        self.form.amount.clear();
        self.form.note.clear();
        self.form.name.clear();
        self.form.image = None;
        self.form.image_mime = None;
        self.form.image_name = None;
    }

    pub fn toast(&mut self, message: &str) {
        self.toast_message = Some(Toast {
            message: message.to_string(),
            created: Instant::now(),
            undo: None,
        });
    }

    pub fn toast_with_undo(&mut self, message: &str, undo: UndoDelete) {
        self.toast_message = Some(Toast {
            message: message.to_string(),
            created: Instant::now(),
            undo: Some(undo),
        });
    }

    pub fn set_check_updates(&mut self, enabled: bool) {
        self.settings.check_updates = enabled;
        self.settings.save();
    }

    pub fn undo_delete(&mut self) {
        let Some(undo) = self
            .toast_message
            .as_ref()
            .and_then(|toast| toast.undo.clone())
        else {
            return;
        };
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == undo.project_id) {
            let index = undo.index.min(project.transactions.len());
            project.transactions.insert(index, undo.transaction.clone());
            project.touch();
            let project = project.clone();
            self.save_project(&project);
        }
        self.toast("Transaksi dikembalikan.");
    }

    pub fn ensure_evidence_texture(&mut self, ctx: &egui::Context, key: &str, encoded: &str) {
        if self.evidence_tex.as_ref().map(|(k, _)| k == key).unwrap_or(false) {
            return;
        }
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(encoded) else {
            return;
        };
        let Ok(image) = image::load_from_memory(&bytes) else {
            return;
        };
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        let texture = ctx.load_texture(format!("evidence_{}", key), color_image, egui::TextureOptions::LINEAR);
        self.evidence_tex = Some((key.to_string(), texture));
    }

    pub fn run_export(&mut self, project: &Project) {
        let (ext, filter) = match self.form.format {
            ExportFormat::Csv => ("csv", "CSV"),
            ExportFormat::Excel => ("xls", "Excel"),
            ExportFormat::Pdf => ("pdf", "PDF"),
        };
        let Some(path) = rfd::FileDialog::new()
            .add_filter(filter, &[ext])
            .set_file_name(format!("{}-{}.{}", slug(&project.title), today_iso(), ext))
            .save_file()
        else {
            return;
        };
        let path = if path.extension().is_none() {
            path.with_extension(ext)
        } else {
            path
        };
        let result = match self.form.format {
            ExportFormat::Csv => export_csv(project, &path),
            ExportFormat::Excel => export_excel(project, &path),
            ExportFormat::Pdf => export_pdf(project, &path),
        };
        match result {
            Ok(()) => {
                self.toast(&format!("Ekspor berhasil: {}", path.display()));
                self.close_modal();
            }
            Err(err) => self.toast(&format!("Ekspor gagal: {}", err)),
        }
    }

    // -- chrome -------------------------------------------------------------

    fn top_bar(&mut self, ctx: &egui::Context) {
        let tc = self.theme_colors;
        egui::TopBottomPanel::top("finflow_top")
            .exact_height(56.0)
            .frame(
                egui::Frame::none()
                    .fill(tc.surface_low)
                    .stroke(Stroke::new(1.0_f32, tc.border_soft))
                    .inner_margin(egui::Margin::symmetric(20.0, 10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if let Some(tex) = &self.logo_tex {
                        ui.add(
                            egui::Image::new(tex)
                                .fit_to_exact_size(Vec2::splat(26.0))
                                .rounding(Rounding::same(6.0)),
                        );
                    } else {
                        let (logo, _) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::hover());
                        ui.painter().rect_filled(logo, Rounding::same(8.0), tc.primary);
                        ui.painter().text(
                            logo.center(),
                            egui::Align2::CENTER_CENTER,
                            "F",
                            FontId::proportional(16.0),
                            tc.white,
                        );
                    }
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("FinFlow").size(15.0).color(tc.text).strong());
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new("KEUANGAN PRIBADI")
                            .size(9.5)
                            .color(tc.muted_dim)
                            .strong(),
                    );

                    ui.add_space(20.0);
                    let (line, _) = ui.allocate_exact_size(Vec2::new(1.0, 22.0), Sense::hover());
                    ui.painter().rect_filled(line, Rounding::ZERO, tc.border);
                    ui.add_space(14.0);

                    let current_title = self
                        .current_project()
                        .map(|p| p.title.clone())
                        .unwrap_or_else(|| "Pilih proyek".to_string());
                    let items: Vec<(String, String)> = self
                        .projects
                        .iter()
                        .map(|p| (p.id.clone(), p.title.clone()))
                        .collect();
                    let mut switch_to: Option<String> = None;
                    let mut new_project = false;
                    let mut open_file = false;
                    styled_combo_scope(ui, &tc, |ui| {
                        egui::ComboBox::from_id_source("project_switcher")
                            .selected_text(
                                egui::RichText::new(current_title)
                                    .size(12.0)
                                    .color(tc.text)
                                    .strong(),
                            )
                            .width(210.0)
                            .height(34.0)
                            .show_ui(ui, |ui| {
                                for (id, title) in &items {
                                    if ui
                                        .selectable_label(self.current.as_deref() == Some(id.as_str()), title)
                                        .clicked()
                                    {
                                        switch_to = Some(id.clone());
                                    }
                                }
                                ui.separator();
                                if ui.button("+ Proyek Baru").clicked() {
                                    new_project = true;
                                }
                                if ui.button("Buka file .ff...").clicked() {
                                    open_file = true;
                                }
                            });
                    });
                    if let Some(id) = switch_to {
                        self.current = Some(id);
                        self.view = View::Dashboard;
                        self.page = 0;
                    }
                    if new_project {
                        self.open_modal(Modal::NewProject);
                    }
                    if open_file {
                        self.open_project_dialog();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::Frame::none()
                            .fill(tc.income_soft)
                            .rounding(Rounding::same(999.0))
                            .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 7.0;
                                    let (dot, _) = ui.allocate_exact_size(Vec2::splat(7.0), Sense::hover());
                                    ui.painter().circle_filled(dot.center(), 3.5, tc.income);
                                    ui.label(
                                        egui::RichText::new("Tersimpan otomatis")
                                            .size(10.0)
                                            .color(tc.income),
                                    );
                                });
                            });
                    });
                });
            });
    }

    fn sidebar(&mut self, ctx: &egui::Context) {
        let tc = self.theme_colors;
        egui::SidePanel::left("finflow_sidebar")
            .exact_width(216.0)
            .resizable(false)
            .frame(
                egui::Frame::none()
                    .fill(tc.surface_low)
                    .stroke(Stroke::new(1.0_f32, tc.border_soft))
                    .inner_margin(egui::Margin::symmetric(14.0, 14.0)),
            )
            .show(ctx, |ui| {
                overline(ui, "Menu Utama", &tc);
                ui.add_space(10.0);
                if nav_item(ui, Icon::Grid, "Dashboard", self.view == View::Dashboard, &tc).clicked() {
                    self.view = View::Dashboard;
                }
                ui.add_space(3.0);
                if nav_item(ui, Icon::Receipt, "Buku Transaksi", self.view == View::Transactions, &tc).clicked() {
                    self.view = View::Transactions;
                    self.page = 0;
                }
                ui.add_space(2.0);
                if nav_item(ui, Icon::Calendar, "Kelola Tagihan", self.view == View::Bills, &tc).clicked() {
                    self.view = View::Bills;
                }
                ui.add_space(2.0);
                if nav_item(ui, Icon::Pie, "Analisis & Anggaran", self.view == View::Analysis, &tc).clicked() {
                    self.view = View::Analysis;
                }

                ui.add_space(14.0);
                divider(ui, &tc);
                ui.add_space(10.0);
                overline(ui, "Preferensi", &tc);
                ui.add_space(8.0);
                let settings_selected = self.view == View::Settings;
                let resp = nav_item(ui, Icon::Gear, "Pengaturan", settings_selected, &tc);
                if resp.clicked() {
                    self.view = View::Settings;
                    self.settings_expanded = !self.settings_expanded;
                }
                // Chevron indicator on the right of the settings entry.
                let c = egui::Pos2::new(resp.rect.right() - 20.0, resp.rect.center().y);
                let color = if settings_selected { tc.text } else { tc.muted };
                let stroke = Stroke::new(1.5_f32, color);
                if self.settings_expanded {
                    ui.painter().line_segment(
                        [egui::Pos2::new(c.x - 4.5, c.y - 2.0), egui::Pos2::new(c.x, c.y + 2.5)],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [egui::Pos2::new(c.x, c.y + 2.5), egui::Pos2::new(c.x + 4.5, c.y - 2.0)],
                        stroke,
                    );
                } else {
                    ui.painter().line_segment(
                        [egui::Pos2::new(c.x - 2.0, c.y - 4.5), egui::Pos2::new(c.x + 2.5, c.y)],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [egui::Pos2::new(c.x + 2.5, c.y), egui::Pos2::new(c.x - 2.0, c.y + 4.5)],
                        stroke,
                    );
                }
                if self.settings_expanded {
                    ui.add_space(3.0);
                    let sections = [
                        SettingsSection::Payments,
                        SettingsSection::Expenses,
                        SettingsSection::Income,
                        SettingsSection::Bills,
                        SettingsSection::Default,
                        SettingsSection::Privacy,
                    ];
                    ui.scope(|ui| {
                        ui.spacing_mut().item_spacing.y = 3.0;
                        for section in sections {
                            let active = settings_selected && self.settings_section == section;
                            if sub_nav_item(ui, section.label(), active, &tc).clicked() {
                                self.view = View::Settings;
                                self.settings_section = section;
                            }
                        }
                    });
                }

                if ui.available_height() > 70.0 {
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new("Penggunaan pribadi")
                                .size(9.5)
                                .color(tc.muted_dim),
                        );
                        ui.label(
                            egui::RichText::new("by Cakra Bisnis")
                                .size(10.0)
                                .color(tc.muted),
                        );
                        ui.add_space(8.0);
                        divider(ui, &tc);
                        ui.add_space(8.0);
                    });
                }
            });
    }

    fn central(&mut self, ctx: &egui::Context) {
        let tc = self.theme_colors;
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(tc.bg).inner_margin(egui::Margin::symmetric(28.0, 20.0)))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        match self.view {
                            View::Dashboard => self.dashboard(ui),
                            View::Transactions => self.transactions(ui),
                            View::Bills => self.bills(ui),
                            View::Analysis => self.analysis(ui),
                            View::Settings => self.settings(ui),
                        }
                        ui.add_space(24.0);
                    });
            });
    }

    fn show_toast(&mut self, ctx: &egui::Context) {
        let tc = self.theme_colors;
        let Some(toast) = self.toast_message.clone() else {
            return;
        };
        let timeout = if toast.undo.is_some() { 6 } else { 4 };
        if toast.created.elapsed() > Duration::from_secs(timeout) {
            self.toast_message = None;
            return;
        }
        let mut undo_clicked = false;
        egui::Area::new(egui::Id::new("finflow_toast"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-22.0, -22.0))
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(tc.surface_high)
                    .rounding(Rounding::same(12.0))
                    .stroke(Stroke::new(1.0_f32, tc.border))
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 8.0),
                        blur: 24.0,
                        spread: -6.0,
                        color: Color32::from_black_alpha(70),
                    })
                    .inner_margin(egui::Margin::symmetric(16.0, 11.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let (rect, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                            paint_icon(ui.painter(), rect, tc.income, Icon::Check, 1.4);
                            ui.label(egui::RichText::new(toast.message).size(11.0).color(tc.text));
                            if toast.undo.is_some() {
                                ui.add_space(8.0);
                                if ui
                                    .add(
                                        egui::Button::new(
                                            egui::RichText::new("Urungkan")
                                                .size(10.5)
                                                .color(tc.primary_light)
                                                .strong(),
                                        )
                                        .fill(tc.primary_soft)
                                        .rounding(Rounding::same(8.0))
                                        .min_size(Vec2::new(0.0, 26.0)),
                                    )
                                    .clicked()
                                {
                                    undo_clicked = true;
                                }
                            }
                        });
                    });
            });
        if undo_clicked {
            self.undo_delete();
        }
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (save, new_tx, export, escape) = ctx.input(|i| {
            (
                i.modifiers.command && i.key_pressed(egui::Key::S),
                i.modifiers.command && i.key_pressed(egui::Key::N),
                i.modifiers.command && i.key_pressed(egui::Key::E),
                i.key_pressed(egui::Key::Escape),
            )
        });
        if save {
            self.save_current();
            self.toast("Proyek disimpan.");
        }
        if new_tx && self.current.is_some() {
            self.form.reset_entry(TxKind::Income);
            self.open_modal(Modal::AddTransaction);
        }
        if export && self.current.is_some() {
            self.open_modal(Modal::Export);
        }
        if escape && self.modal != Modal::None {
            self.close_modal();
        }

        self.top_bar(ctx);
        self.sidebar(ctx);
        self.central(ctx);
        self.show_modal(ctx);
        self.show_toast(ctx);
    }
}

/// Top-left position that centres a `w × h` window on the primary monitor.
fn centered_pos(w: f32, h: f32) -> egui::Pos2 {
    #[cfg(target_os = "windows")]
    {
        extern "system" {
            fn GetSystemMetrics(index: i32) -> i32;
        }
        const SM_CXSCREEN: i32 = 0;
        const SM_CYSCREEN: i32 = 1;
        unsafe {
            let sw = GetSystemMetrics(SM_CXSCREEN) as f32;
            let sh = GetSystemMetrics(SM_CYSCREEN) as f32;
            return egui::pos2((sw - w) / 2.0, (sh - h) / 2.0);
        }
    }
    #[allow(unreachable_code)]
    egui::pos2(80.0, 80.0)
}

/// Jalankan pemeriksa update di latar belakang. Script menunggu aplikasi
/// ditutup sebelum mengganti executable, sehingga sesi aktif tidak terganggu.
fn spawn_auto_updater() {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;

        let exe = match std::env::current_exe() {
            Ok(p) => p,
            Err(_) => return,
        };
        let dir = match exe.parent() {
            Some(p) => p.to_path_buf(),
            None => return,
        };
        let script = dir.join("update.ps1");
        if !script.exists() {
            return;
        }
        let _ = std::process::Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-ExecutionPolicy",
                "Bypass",
                "-WindowStyle",
                "Hidden",
                "-File",
            ])
            .arg(&script)
            .arg("-AppDir")
            .arg(&dir)
            .arg("-ParentPid")
            .arg(std::process::id().to_string())
            .arg("-CurrentVersion")
            .arg(env!("CARGO_PKG_VERSION"))
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
    #[cfg(not(target_os = "windows"))]
    let _ = ();
}

/// Baca dan hapus penanda bahwa update baru saja terpasang.
fn update_notice() -> Option<String> {
    let local = std::env::var("LOCALAPPDATA").ok()?;
    let path = PathBuf::from(local).join("FinFlow").join("update_applied.txt");
    let version = fs::read_to_string(&path).ok()?;
    let _ = fs::remove_file(&path);
    let version = version.trim();
    if version.is_empty() {
        None
    } else {
        Some(version.to_string())
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([1080.0, 660.0])
            .with_position(centered_pos(1296.0, 759.0))
            .with_title("FinFlow — Keuangan Pribadi")
            .with_icon(theme::window_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "FinFlow — Keuangan Pribadi",
        options,
        Box::new(|cc| {
            let mut app = App::new();
            app.logo_tex = Some(theme::logo_texture(&cc.egui_ctx));
            theme::install(&cc.egui_ctx, &app.theme_colors);
            Box::new(app)
        }),
    )
}
