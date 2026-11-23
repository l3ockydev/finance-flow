//! Data model, Indonesian formatting helpers, demo seed, and export engines.
//! Everything is stored locally in a single `.ff` JSON document per project.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TxKind {
    Income,
    Expense,
}

impl TxKind {
    pub fn label(self) -> &'static str {
        match self {
            TxKind::Income => "Pemasukan",
            TxKind::Expense => "Pengeluaran",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    #[serde(default)]
    pub id: String,
    pub date: String, // yyyy-mm-dd
    #[serde(default)]
    pub time: String, // HH:MM
    pub amount: f64,
    pub category: String,
    #[serde(default)]
    pub account: String,
    pub note: String,
    #[serde(default = "default_kind")]
    pub kind: TxKind,
    #[serde(default)]
    pub image: Option<String>, // base64 payload, no data-url prefix
    #[serde(default)]
    pub image_mime: Option<String>,
    #[serde(default)]
    pub image_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BillStatus {
    Paid,
    Pending,
    Scheduled,
    Overdue,
}

impl BillStatus {
    pub fn label(self) -> &'static str {
        match self {
            BillStatus::Paid => "Lunas",
            BillStatus::Pending => "Menunggu",
            BillStatus::Scheduled => "Terjadwal",
            BillStatus::Overdue => "Terlambat",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bill {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub due_date: String,
    pub category: String,
    pub status: BillStatus,
    #[serde(default)]
    pub account: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Budget {
    pub category: String,
    pub limit: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    #[serde(default = "default_version")]
    pub version: u32,
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default)]
    pub transactions: Vec<Transaction>,
    #[serde(default)]
    pub bills: Vec<Bill>,
    #[serde(default)]
    pub budgets: Vec<Budget>,
    #[serde(default)]
    pub bank_accounts: Vec<String>,
    #[serde(default = "default_income_categories")]
    pub income_categories: Vec<String>,
    #[serde(default = "default_expense_categories")]
    pub expense_categories: Vec<String>,
    #[serde(default = "default_bill_categories")]
    pub bill_categories: Vec<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

fn default_version() -> u32 {
    1
}
fn default_currency() -> String {
    "IDR".to_string()
}
fn default_kind() -> TxKind {
    TxKind::Expense
}

pub fn default_income_categories() -> Vec<String> {
    vec![
        "Gaji & Bonus".into(),
        "Bisnis & Penjualan".into(),
        "Freelance".into(),
        "Pendapatan Sampingan".into(),
        "Investasi".into(),
        "Lainnya".into(),
    ]
}

pub fn default_expense_categories() -> Vec<String> {
    vec![
        "Kebutuhan Pokok".into(),
        "Makanan & Minuman".into(),
        "Transportasi".into(),
        "Hiburan".into(),
        "Kesehatan".into(),
        "Pajak & Tagihan".into(),
        "Pendidikan".into(),
        "Lainnya".into(),
    ]
}

pub fn default_bill_categories() -> Vec<String> {
    vec![
        "Internet".into(),
        "Utilitas".into(),
        "Cicilan".into(),
        "Asuransi".into(),
        "Langganan".into(),
        "Lainnya".into(),
    ]
}

pub fn default_bank_accounts() -> Vec<String> {
    vec![
        "BCA Prioritas".into(),
        "Kartu Mandiri Debit".into(),
        "QRIS GoPay".into(),
        "m-BCA".into(),
        "RDN Mandiri Sekuritas".into(),
        "Tunai".into(),
    ]
}

impl Project {
    pub fn blank(index: usize) -> Self {
        let now = today_iso();
        Self {
            version: 1,
            id: format!("proj_{}_{}", now.replace('-', ""), index),
            title: format!("Proyek Keuangan {}", index),
            owner: String::new(),
            currency: "IDR".to_string(),
            transactions: Vec::new(),
            bills: Vec::new(),
            budgets: vec![
                Budget { category: "Kebutuhan Pokok".into(), limit: 4_000_000.0 },
                Budget { category: "Transportasi".into(), limit: 1_000_000.0 },
                Budget { category: "Hiburan".into(), limit: 800_000.0 },
                Budget { category: "Investasi".into(), limit: 2_500_000.0 },
            ],
            bank_accounts: default_bank_accounts(),
            income_categories: default_income_categories(),
            expense_categories: default_expense_categories(),
            bill_categories: default_bill_categories(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn total_income(&self) -> f64 {
        self.transactions.iter().filter(|t| t.kind == TxKind::Income).map(|t| t.amount).sum()
    }

    pub fn total_expense(&self) -> f64 {
        self.transactions.iter().filter(|t| t.kind == TxKind::Expense).map(|t| t.amount).sum()
    }

    pub fn balance(&self) -> f64 {
        self.total_income() - self.total_expense()
    }

    pub fn pending_bills_total(&self) -> f64 {
        self.bills.iter().filter(|b| b.status != BillStatus::Paid).map(|b| b.amount).sum()
    }

    pub fn paid_bills_total(&self) -> f64 {
        self.bills.iter().filter(|b| b.status == BillStatus::Paid).map(|b| b.amount).sum()
    }

    pub fn spent_for(&self, category: &str) -> f64 {
        self.transactions
            .iter()
            .filter(|t| t.kind == TxKind::Expense && t.category == category)
            .map(|t| t.amount)
            .sum()
    }

    pub fn month_totals(&self, months_back: usize) -> Vec<(String, f64, f64)> {
        let today = today_iso();
        let (ty, tm, _) = parse_iso(&today).unwrap_or((2026, 11, 1));
        let mut out = Vec::new();
        for step in (0..months_back).rev() {
            let mut y = ty;
            let mut m = tm as i32 - step as i32;
            while m <= 0 {
                m += 12;
                y -= 1;
            }
            let prefix = format!("{:04}-{:02}", y, m);
            let income: f64 = self
                .transactions
                .iter()
                .filter(|t| t.date.starts_with(&prefix) && t.kind == TxKind::Income)
                .map(|t| t.amount)
                .sum();
            let expense: f64 = self
                .transactions
                .iter()
                .filter(|t| t.date.starts_with(&prefix) && t.kind == TxKind::Expense)
                .map(|t| t.amount)
                .sum();
            out.push((month_short(m as u32).to_string(), income, expense));
        }
        out
    }

    pub fn savings_ratio(&self) -> f64 {
        let income = self.total_income();
        if income <= 0.0 {
            0.0
        } else {
            (self.balance() / income).clamp(0.0, 1.0)
        }
    }

    /// Pemasukan dan pengeluaran pada bulan berjalan (bukan total seluruh proyek).
    pub fn this_month_totals(&self) -> (f64, f64) {
        let prefix = today_iso()[..7].to_string();
        let income = self
            .transactions
            .iter()
            .filter(|t| t.kind == TxKind::Income && t.date.starts_with(&prefix))
            .map(|t| t.amount)
            .sum();
        let expense = self
            .transactions
            .iter()
            .filter(|t| t.kind == TxKind::Expense && t.date.starts_with(&prefix))
            .map(|t| t.amount)
            .sum();
        (income, expense)
    }

    pub fn touch(&mut self) {
        self.updated_at = today_iso();
    }
}

// ---------------------------------------------------------------------------
// Currency & date helpers (Indonesian conventions)
// ---------------------------------------------------------------------------

pub fn format_rp(amount: f64) -> String {
    let negative = amount < 0.0;
    let rounded = amount.abs().round() as i64;
    let digits = rounded.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push('.');
        }
        out.push(ch);
    }
    if negative {
        format!("- Rp {}", out)
    } else {
        format!("Rp {}", out)
    }
}

pub fn format_signed(kind: TxKind, amount: f64) -> String {
    let sign = match kind {
        TxKind::Income => "+",
        TxKind::Expense => "-",
    };
    format!("{} {}", sign, format_rp(amount))
}

pub fn parse_amount(text: &str) -> f64 {
    let cleaned: String = text.chars().filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',').collect();
    let normalized = if cleaned.contains(',') {
        cleaned.replace('.', "").replace(',', ".")
    } else {
        cleaned.replace('.', "")
    };
    normalized.parse::<f64>().unwrap_or(0.0)
}

pub fn today_iso() -> String {
    let days = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| (d.as_secs() / 86_400) as i64)
        .unwrap_or(0);
    let (y, m, d) = civil_from_days(days);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

pub fn parse_iso(iso: &str) -> Option<(i32, u32, u32)> {
    let mut parts = iso.split('-');
    let y = parts.next()?.parse().ok()?;
    let m = parts.next()?.parse().ok()?;
    let d = parts.next()?.parse().ok()?;
    Some((y, m, d))
}

pub fn format_date_short(iso: &str) -> String {
    match parse_iso(iso) {
        Some((y, m, d)) => format!("{} {} {}", d, month_short(m), y),
        None => iso.to_string(),
    }
}

pub fn format_date_long(iso: &str) -> String {
    match parse_iso(iso) {
        Some((y, m, d)) => {
            let days = days_from_civil(y as i64, m as i64, d as i64);
            format!("{}, {} {} {}", weekday_name(days), d, month_long(m), y)
        }
        None => iso.to_string(),
    }
}

pub fn days_until(iso: &str) -> i64 {
    match (parse_iso(iso), parse_iso(&today_iso())) {
        (Some((y1, m1, d1)), Some((y2, m2, d2))) => {
            days_from_civil(y1 as i64, m1 as i64, d1 as i64) - days_from_civil(y2 as i64, m2 as i64, d2 as i64)
        }
        _ => 0,
    }
}

pub fn shift_iso(iso: &str, days: i64) -> String {
    match parse_iso(iso) {
        Some((y, m, d)) => {
            let (y2, m2, d2) = civil_from_days(days_from_civil(y as i64, m as i64, d as i64) + days);
            format!("{:04}-{:02}-{:02}", y2, m2, d2)
        }
        None => iso.to_string(),
    }
}

pub fn month_short(m: u32) -> &'static str {
    const NAMES: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des"];
    NAMES.get((m as usize).saturating_sub(1)).copied().unwrap_or("-")
}

pub fn month_long(m: u32) -> &'static str {
    const NAMES: [&str; 12] = [
        "Januari", "Februari", "Maret", "April", "Mei", "Juni",
        "Juli", "Agustus", "September", "Oktober", "November", "Desember",
    ];
    NAMES.get((m as usize).saturating_sub(1)).copied().unwrap_or("-")
}

fn weekday_name(days: i64) -> &'static str {
    const NAMES: [&str; 7] = ["Minggu", "Senin", "Selasa", "Rabu", "Kamis", "Jumat", "Sabtu"];
    NAMES[((days + 4).rem_euclid(7)) as usize]
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (y + if m <= 2 { 1 } else { 0 }, m as u32, d as u32)
}

// ---------------------------------------------------------------------------
// Export: CSV, Excel (SpreadsheetML 2003), PDF, and printing
// ---------------------------------------------------------------------------

pub fn slug(text: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    let trimmed = out.trim_end_matches('-').to_string();
    if trimmed.is_empty() { "finflow".to_string() } else { trimmed }
}

fn csv_cell(text: &str) -> String {
    format!("\"{}\"", text.replace('"', "\"\""))
}

pub fn export_csv(project: &Project, path: &Path) -> io::Result<()> {
    let mut out = String::from("Tanggal,Waktu,Keterangan,Kategori,Rekening,Tipe,Jumlah,Status\n");
    for t in &project.transactions {
        out.push_str(&format!(
            "{},{},{},{},{},{},{:.2},{}\n",
            csv_cell(&t.date),
            csv_cell(&t.time),
            csv_cell(&t.note),
            csv_cell(&t.category),
            csv_cell(&t.account),
            csv_cell(t.kind.label()),
            t.amount,
            csv_cell("Selesai"),
        ));
    }
    out.push('\n');
    out.push_str(&format!("Total Pemasukan,,,,,,{:.2},\n", project.total_income()));
    out.push_str(&format!("Total Pengeluaran,,,,,,{:.2},\n", project.total_expense()));
    out.push_str(&format!("Saldo,,,,,,{:.2},\n", project.balance()));
    fs::write(path, out)
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn export_excel(project: &Project, path: &Path) -> io::Result<()> {
    let mut rows = String::new();
    let header = ["Tanggal", "Waktu", "Keterangan", "Kategori", "Rekening", "Tipe", "Jumlah", "Status"];
    rows.push_str("<Row>");
    for h in header {
        rows.push_str(&format!("<Cell ss:StyleID=\"sHead\"><Data ss:Type=\"String\">{}</Data></Cell>", xml_escape(h)));
    }
    rows.push_str("</Row>");
    for t in &project.transactions {
        let cells = [
            (format_date_short(&t.date), "String"),
            (t.time.clone(), "String"),
            (t.note.clone(), "String"),
            (t.category.clone(), "String"),
            (t.account.clone(), "String"),
            (t.kind.label().to_string(), "String"),
            (format!("{:.2}", t.amount), "Number"),
            ("Selesai".to_string(), "String"),
        ];
        rows.push_str("<Row>");
        for (value, ty) in cells {
            rows.push_str(&format!("<Cell><Data ss:Type=\"{}\">{}</Data></Cell>", ty, xml_escape(&value)));
        }
        rows.push_str("</Row>");
    }
    for (label, value) in [
        ("Total Pemasukan", project.total_income()),
        ("Total Pengeluaran", project.total_expense()),
        ("Saldo", project.balance()),
    ] {
        rows.push_str(&format!(
            "<Row><Cell ss:StyleID=\"sHead\"><Data ss:Type=\"String\">{}</Data></Cell><Cell><Data ss:Type=\"Number\">{:.2}</Data></Cell></Row>",
            label, value
        ));
    }
    let doc = format!(
        "<?xml version=\"1.0\"?>\n<?mso-application progid=\"Excel.Sheet\"?>\n\
         <Workbook xmlns=\"urn:schemas-microsoft-com:office:spreadsheet\" \
         xmlns:ss=\"urn:schemas-microsoft-com:office:spreadsheet\">\
         <Styles><Style ss:ID=\"sHead\"><Font ss:Bold=\"1\"/></Style></Styles>\
         <Worksheet ss:Name=\"Transaksi\"><Table>{}</Table></Worksheet></Workbook>",
        rows
    );
    fs::write(path, doc)
}

fn pdf_escape(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            '\u{2014}' | '\u{2013}' | '\u{2022}' => out.push('-'),
            c if (c as u32) < 128 => out.push(c),
            _ => out.push('?'),
        }
    }
    out
}

fn pdf_line(x: f32, y: f32, size: f32, text: &str) -> String {
    format!("BT /F1 {:.1} Tf {:.1} {:.1} Td ({}) Tj ET\n", size, x, y, pdf_escape(text))
}

pub fn export_pdf(project: &Project, path: &Path) -> io::Result<()> {
    let mut rows: Vec<[String; 5]> = Vec::new();
    for t in project.transactions.iter().rev() {
        rows.push([
            format_date_short(&t.date),
            t.note.clone(),
            t.category.clone(),
            t.kind.label().to_string(),
            format_signed(t.kind, t.amount),
        ]);
    }

    let per_page = 22usize;
    let page_count = rows.len().div_ceil(per_page).max(1);

    let mut streams: Vec<String> = Vec::new();
    for index in 0..page_count {
        let start = index * per_page;
        let end = (start + per_page).min(rows.len());
        let chunk = &rows[start..end];
        let mut s = String::new();
        s.push_str(&pdf_line(40.0, 560.0, 18.0, "FinFlow - Laporan Keuangan"));
        s.push_str(&pdf_line(40.0, 538.0, 11.0, &format!("{}  |  Pemilik: {}", project.title, if project.owner.is_empty() { "-" } else { &project.owner })));
        s.push_str(&pdf_line(40.0, 522.0, 9.0, &format!("Dicetak {}  |  Halaman {} dari {}", format_date_long(&today_iso()), index + 1, page_count)));
        if index == 0 {
            s.push_str(&pdf_line(40.0, 492.0, 10.0, &format!("Total Pemasukan: {}     Total Pengeluaran: {}     Saldo: {}", format_rp(project.total_income()), format_rp(project.total_expense()), format_rp(project.balance()))));
            s.push_str(&pdf_line(40.0, 476.0, 10.0, &format!("Tagihan Tertunda: {}     Jumlah Transaksi: {}", format_rp(project.pending_bills_total()), project.transactions.len())));
        }
        let mut y = 444.0;
        for (label, x) in [("Tanggal", 40.0), ("Keterangan", 125.0), ("Kategori", 410.0), ("Tipe", 560.0), ("Jumlah", 660.0)] {
            s.push_str(&pdf_line(x, y, 9.0, label));
        }
        s.push_str(&format!("0.5 w 40 {:.1} m 800 {:.1} l S\n", y - 4.0, y - 4.0));
        y -= 20.0;
        for row in chunk.iter() {
            let cols = [
                (row[0].as_str(), 40.0),
                (row[1].as_str(), 125.0),
                (row[2].as_str(), 410.0),
                (row[3].as_str(), 560.0),
                (row[4].as_str(), 660.0),
            ];
            for (text, x) in cols {
                let clipped: String = text.chars().take(if x == 125.0 { 40 } else { 22 }).collect();
                s.push_str(&pdf_line(x, y, 8.5, &clipped));
            }
            y -= 17.0;
        }
        s.push_str(&pdf_line(40.0, 40.0, 8.0, "Dokumen dibuat lokal oleh FinFlow. Tidak ada data yang dikirim ke internet."));
        streams.push(s);
    }

    let page_count = streams.len();
    let kids: Vec<String> = (0..page_count)
        .map(|i| format!("{} 0 R", 4 + i * 2))
        .collect();
    let mut out: Vec<u8> = Vec::new();
    let mut offsets: Vec<usize> = vec![0; 4 + page_count * 2];
    out.extend_from_slice(b"%PDF-1.4\n");

    let push_obj = |out: &mut Vec<u8>, offsets: &mut Vec<usize>, number: usize, body: String| {
        offsets[number] = out.len();
        out.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", number, body).as_bytes());
    };

    push_obj(&mut out, &mut offsets, 1, "<< /Type /Catalog /Pages 2 0 R >>".to_string());
    push_obj(&mut out, &mut offsets, 2, format!("<< /Type /Pages /Kids [{}] /Count {} >>", kids.join(" "), page_count));
    push_obj(&mut out, &mut offsets, 3, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".to_string());

    for (i, stream) in streams.iter().enumerate() {
        let page_obj = 4 + i * 2;
        let content_obj = page_obj + 1;
        push_obj(
            &mut out,
            &mut offsets,
            page_obj,
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 842 595] /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>",
                content_obj
            ),
        );
        offsets[content_obj] = out.len();
        out.extend_from_slice(format!("{} 0 obj\n<< /Length {} >>\nstream\n", content_obj, stream.len()).as_bytes());
        out.extend_from_slice(stream.as_bytes());
        out.extend_from_slice(b"endstream\nendobj\n");
    }

    let xref_at = out.len();
    let total = offsets.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", total).as_bytes());
    for offset in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{:010} 00000 n \n", offset).as_bytes());
    }
    out.extend_from_slice(
        format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", total, xref_at).as_bytes(),
    );
    fs::write(path, out)
}

pub fn print_pdf(project: &Project) -> io::Result<()> {
    let dir = std::env::temp_dir();
    let file = dir.join(format!("finflow-{}.pdf", slug(&project.title)));
    export_pdf(project, &file)?;
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let command = format!("Start-Process -FilePath '{}' -Verb Print", file.display());
        let _ = std::process::Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", command.as_str()])
            .creation_flags(0x0800_0000)
            .spawn()?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("xdg-open").arg(&file).spawn()?;
    }
    Ok(())
}
