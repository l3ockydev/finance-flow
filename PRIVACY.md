# Kebijakan Privasi / Privacy Policy

🇮🇩 [Bahasa Indonesia](#bahasa-indonesia) · 🇬🇧 [English](#english)

---

<a id="bahasa-indonesia"></a>

## Bahasa Indonesia

**FinFlow tidak mengumpulkan data pribadi Anda.**

Aplikasi ini berjalan sepenuhnya di komputer Anda:

- **Tanpa akun** — tidak ada login, email, atau identitas pengguna.
- **Tanpa telemetri** — tidak ada statistik penggunaan, analytics, atau pelacakan.
- **Tanpa iklan** — tidak ada jaringan iklan atau konten pihak ketiga.
- **Tanpa server** — data keuangan Anda tidak pernah diunggah ke mana pun.

### Di mana data disimpan?

Setiap laporan disimpan sebagai satu file `.ff` (JSON) di folder `projects\`
di sebelah `finance_flow.exe`. File ini berisi transaksi, tagihan, anggaran,
dan bukti gambar (sebagai base64). Anda dapat memindahkan, menyalin, atau
menghapusnya kapan saja.

Salinan otomatis (maksimal 20 per proyek) disimpan di:

```text
%LOCALAPPDATA%\FinFlow\backups\
```

Backup ini dibuat setiap kali proyek disimpan dan dapat dipulihkan melalui
**Pengaturan → Privasi & Update → Pulihkan dari Backup...**.

### Koneksi internet

Satu-satunya koneksi opsional adalah **pemeriksaan pembaruan** ke GitHub Releases.
Pemeriksaan ini dapat dimatikan di **Pengaturan → Privasi & Update**. Jika
dimatikan, FinFlow tidak menghubungi jaringan sama sekali. Saat offline,
pemeriksaan gagal secara senyap dan aplikasi tetap berjalan normal.

### Izin yang diminta

FinFlow hanya meminta izin sistem operasi standar untuk:

- Membaca/menulis file `.ff` di folder aplikasi.
- Memilih file melalui dialog Windows (buka proyek, bukti gambar, restore backup).
- Menjalankan `update.ps1` hanya jika pemeriksaan pembaruan diaktifkan.

Tidak ada izin untuk kamera, mikrofon, lokasi, kontak, atau notifikasi.

---

<a id="english"></a>

## English

**FinFlow does not collect your personal data.**

The app runs entirely on your computer:

- **No account** — no login, email, or user identity.
- **No telemetry** — no usage statistics, analytics, or tracking.
- **No ads** — no ad networks or third-party content.
- **No server** — your financial data is never uploaded anywhere.

### Where is data stored?

Each report is a single `.ff` (JSON) file in the `projects\` folder next to
`finance_flow.exe`. It contains transactions, bills, budgets, and image
evidence (as base64). You can move, copy, or delete it at any time.

Automatic copies (up to 20 per project) are stored in:

```text
%LOCALAPPDATA%\FinFlow\backups\
```

These are created on every save and can be restored via
**Settings → Privacy & Update → Restore from Backup...**.

### Internet connection

The only optional connection is the **update check** against GitHub Releases.
It can be disabled in **Settings → Privacy & Update**. When disabled, FinFlow
does not contact the network at all. When offline, the check fails silently
and the app keeps working normally.

### Permissions requested

FinFlow only asks for standard OS permissions to:

- Read/write `.ff` files in its application folder.
- Pick files via Windows dialogs (open project, image evidence, restore backup).
- Run `update.ps1` only when the update check is enabled.

No permission is requested for camera, microphone, location, contacts, or
notifications.
