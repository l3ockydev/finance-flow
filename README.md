# FinFlow — Keuangan Pribadi / Personal Finance

🇮🇩 [Bahasa Indonesia](#bahasa-indonesia) · 🇬🇧 [English](#english)

Aplikasi desktop manajemen keuangan pribadi untuk Windows 10/11 (64-bit).
A personal finance desktop app for Windows 10/11 (64-bit).

---

<a id="bahasa-indonesia"></a>

## Bahasa Indonesia

Aplikasi ini berjalan **100% offline**, tanpa akun, tanpa telemetri, dan tanpa
dependensi runtime. Setiap laporan disimpan sebagai satu file `.ff` (JSON) yang
mandiri — mudah dipindah, dibackup, atau dibuka lagi di komputer lain.

### Install (paling cepat)

1. Buka halaman **Releases** di repository GitHub ini.
2. Unduh **`finance_flow-windows-x86_64.zip`** dari rilis terbaru.
3. Ekstrak ke folder mana pun, misalnya `Dokumen\FinFlow`.
4. Double-click **`finance_flow.exe`**.

Selesai. Tidak perlu menginstal Rust, tidak perlu internet, tidak perlu setup lain.
ZIP berisi executable (± 4 MB), README, LICENSE, dan file pendukung update.

> **SmartScreen / antivirus:** karena executable belum ditandatangani, Windows
> mungkin menampilkan peringatan *"Windows protected your PC"*. Klik
> **More info → Run anyway** untuk menjalankannya.

### Instal otomatis ke Start Menu (opsional)

Dari folder repository yang sudah di-clone, jalankan:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1 -Repo l3ockydev/finance-flow
```

Ganti `l3ockydev/finance-flow` dengan nama repository GitHub Anda, misalnya `budi/finance-flow`.
Jika dijalankan dari clone Git yang sudah punya remote `origin`, parameter `-Repo`
tidak perlu dicantumkan. Skrip akan mengunduh ZIP rilis terbaru, mengekstraknya
ke `%LOCALAPPDATA%\FinFlow`, membuat shortcut **FinFlow** di Start Menu, dan
**tidak** menghapus folder `projects\` yang sudah ada.

Uninstall: hapus shortcut *FinFlow* di Start Menu, lalu hapus folder
`%LOCALAPPDATA%\FinFlow`.

### Build dari source

Diperlukan [Rust](https://rustup.rs/) (stable) + Visual Studio Build Tools dengan
**Desktop development with C++**.

```powershell
git clone https://github.com/l3ockydev/finance-flow.git
cd finance-flow
cargo build --release
.\target\release\finance_flow.exe
```

Build release sudah dioptimalkan (`opt-level = "z"`, LTO, strip), sehingga
executable akhir hanya ± 4 MB.

### Data & backup

Saat pertama kali dijalankan, aplikasi membuat folder ini di sebelah executable:

```text
finance_flow.exe
projects\
  └── proj-....ff
```

- Setiap proyek = satu file `.ff` berisi transaksi, tagihan, anggaran, dan bukti gambar (base64).
- Auto-save dilakukan dengan penulisan atomik (`*.ff.tmp` → `*.ff`).
- **Backup cukup dengan menyalin folder `projects\`.**
- Untuk pindah komputer, salin file `.ff` ke folder `projects\` di instalasi baru.
- Proyek **kosong** dibuat otomatis jika folder `projects\` kosong — tidak ada transaksi atau tagihan contoh.

### Fitur

- Dashboard: saldo, pemasukan, pengeluaran, tagihan, grafik arus kas 6 bulan.
- Buku transaksi: tambah/ubah/hapus, pencarian, filter, paginasi.
- Bukti gambar PNG/JPEG/WebP ≤ 5 MB, tersimpan di dalam file proyek.
- Tagihan dengan status dan aksi “Tandai Lunas”.
- Analisis anggaran per kategori, rasio tabungan, komposisi pengeluaran.
- Ekspor CSV, Excel (`.xls`), PDF; cetak via dialog Windows.
- Proyek terpisah: buat baru, buka `.ff`, simpan salinan, hapus.
- 7 tema warna (Default, Banking, DarkFinance, Minimal, Emerald, Amber, Violet).

### Shortcut keyboard

| Tombol | Aksi |
|--------|------|
| `Ctrl+N` | Transaksi baru |
| `Ctrl+S` | Simpan proyek |
| `Ctrl+E` | Ekspor |
| `Esc` | Tutup modal |

### Struktur source

| File | Isi |
|------|-----|
| `main.rs` | Entry point, state aplikasi, shell UI, persistensi |
| `model.rs` | Model data, format Rupiah/tanggal, ekspor, cetak |
| `theme.rs` | Token desain, font, widget, ikon vektor |
| `views.rs` | Dashboard + baris tabel transaksi |
| `views_pages.rs` | Transaksi, tagihan, analisis, pengaturan |
| `modals.rs` | Formulir dan dialog |
| `assets/` | Logo (JPEG) dan font Plus Jakarta Sans (TTF) |
| `.github/workflows/release.yml` | Build ZIP otomatis saat tag `v*` di-push |

### Update otomatis

Saat aplikasi dibuka, FinFlow memeriksa rilis terbaru di GitHub melalui
`update.ps1` (maksimal sekali per jam). Jika ada versi lebih baru, ZIP diunduh,
lalu executable diganti **setelah aplikasi ditutup** — pekerjaan Anda yang sedang
berjalan tidak terganggu. Saat membuka versi baru, muncul notifikasi
*"FinFlow diperbarui ke v…"*.

- Internet hanya diperlukan untuk pemeriksaan update; jika offline, pemeriksaan
  gagal secara senyap dan aplikasi tetap berjalan normal.
- Nama repository dibaca dari `update.json`. File bawaan memakai placeholder
  `l3ockydev/finance-flow`; ganti dengan repo Anda, misalnya `{ "repo": "budi/finance-flow" }`.
- ZIP dari GitHub Actions sudah berisi `update.json` dengan nama repo yang benar.
- Update terjadi hanya jika ada **Release baru bertag `v…`** dengan ZIP
  `finance_flow-windows-x86_64.zip` — bukan setiap commit kode.

### Lisensi

FinFlow memakai **Personal Use License** (lihat [`LICENSE`](LICENSE)): boleh
dipakai, dimodifikasi, dan dibagikan **gratis untuk keperluan pribadi /
non-komersial**. Penggunaan komersial tidak diizinkan. Setiap salinan wajib
menyertakan file lisensi ini.

### Rilis baru (maintainer)

```powershell
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions akan membangun executable, membuat ZIP, dan melampirkannya ke
halaman Releases secara otomatis.

---

<a id="english"></a>

## English

FinFlow runs **100% offline** — no account, no telemetry, no runtime dependencies.
Each report is a single self-contained `.ff` (JSON) file, easy to move, back up,
or reopen on another computer.

### Install (quickest)

1. Open the **Releases** page of this GitHub repository.
2. Download **`finance_flow-windows-x86_64.zip`** from the latest release.
3. Extract it anywhere, for example `Documents\FinFlow`.
4. Double-click **`finance_flow.exe`**.

That's it. Rust, internet access, and extra setup are not required. The ZIP
contains the executable (~4 MB), README, LICENSE, and update support files.

> **SmartScreen / antivirus:** the executable is unsigned, so Windows may show
> *"Windows protected your PC"*. Click **More info → Run anyway** to launch it.

### Optional Start Menu installer

From a cloned copy of this repository, run:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1 -Repo l3ockydev/finance-flow
```

Replace `l3ockydev/finance-flow` with your GitHub repository name, e.g. `budi/finance-flow`.
When running inside a Git clone that already has an `origin` remote, you may omit
`-Repo`. The script downloads the latest release ZIP, extracts it to
`%LOCALAPPDATA%\FinFlow`, creates a **FinFlow** Start Menu shortcut, and never
deletes an existing `projects\` folder.

To uninstall, remove the *FinFlow* shortcut and delete `%LOCALAPPDATA%\FinFlow`.

### Build from source

You need [Rust](https://rustup.rs/) (stable) and Visual Studio Build Tools with
**Desktop development with C++**.

```powershell
git clone https://github.com/l3ockydev/finance-flow.git
cd finance-flow
cargo build --release
.\target\release\finance_flow.exe
```

The release profile is optimized (`opt-level = "z"`, LTO, strip), producing a
~4 MB executable.

### Data & backup

On first launch the app creates this folder next to the executable:

```text
finance_flow.exe
projects\
  └── proj-....ff
```

- Each project is one `.ff` file containing transactions, bills, budgets, and image evidence (base64).
- Auto-save uses atomic writes (`*.ff.tmp` → `*.ff`).
- **Back up simply by copying the `projects\` folder.**
- To move computers, copy the `.ff` files into `projects\` of the new installation.
- A **blank** project is created automatically when `projects\` is empty — no sample transactions or bills.

### Features

- Dashboard: balance, income, expenses, bills, and a 6-month cash-flow chart.
- Transaction ledger: add/edit/delete, search, filters, and pagination.
- Image evidence (PNG/JPEG/WebP, ≤ 5 MB) stored inside the project file.
- Bills with statuses and a “Mark as Paid” action.
- Budget analysis per category, savings ratio, and expense composition.
- Export to CSV, Excel (`.xls`), and PDF; print via the Windows dialog.
- Separate projects: create, open `.ff`, save a copy, delete.
- 7 live themes (Default, Banking, DarkFinance, Minimal, Emerald, Amber, Violet).

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `Ctrl+N` | New transaction |
| `Ctrl+S` | Save project |
| `Ctrl+E` | Export |
| `Esc` | Close modal |

### Source layout

| File | Contents |
|------|----------|
| `main.rs` | Entry point, app state, UI shell, persistence |
| `model.rs` | Data model, Rupiah/date formatting, export, print |
| `theme.rs` | Design tokens, font, widgets, vector icons |
| `views.rs` | Dashboard and transaction table rows |
| `views_pages.rs` | Transactions, bills, analysis, settings |
| `modals.rs` | Forms and dialogs |
| `assets/` | Logo (JPEG) and Plus Jakarta Sans font (TTF) |
| `.github/workflows/release.yml` | Automatic ZIP build on `v*` tag push |

### Automatic updates

On startup FinFlow checks GitHub Releases via `update.ps1` (at most once per hour).
If a newer version exists, the ZIP is downloaded and the executable is replaced
**after the app closes** — your current session is never interrupted. The next
launch shows a *"FinFlow updated to v…"* notification.

- Internet is only needed for the update check; when offline the check fails
  silently and the app works normally.
- The repository name comes from `update.json`. The bundled file uses the
  `l3ockydev/finance-flow` placeholder; replace it, e.g. `{ "repo": "budi/finance-flow" }`.
- ZIPs produced by GitHub Actions already contain the correct `update.json`.
- Updates only happen for a **new Release tagged `v…`** with
  `finance_flow-windows-x86_64.zip` attached — not for regular commits.

### License

FinFlow uses the **Personal Use License** (see [`LICENSE`](LICENSE)): free for
personal, non-commercial use, modification, and sharing. Commercial use is not
permitted. Every copy must retain the license file.

### Publishing a release (maintainers)

```powershell
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions will build the executable, create the ZIP, and attach it to the
Releases page automatically.
