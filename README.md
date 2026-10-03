# Google Meet Masaüstü İstemcisi (v0.1.0)

CachyOS (Arch Linux) için Rust ve Tauri v2 kullanılarak geliştirilmiş, ultra hafif ve modern Google Meet masaüstü istemcisi.

## 🚀 Temel Özellikler

1. **User-Agent Spoofing (Tarayıcı Kimliği)**:
   - WebKitGTK motoru güncel bir Linux Chrome (`Chrome/131.0.0.0`) olarak tanıtılmıştır. Böylece Google Meet'in "desteklenmeyen tarayıcı" engeli tamamen aşılmıştır.
2. **Kamera, Mikrofon ve Ekran Paylaşımı (WebRTC & PipeWire)**:
   - WebKitGTK donanım izinleri otomatik olarak karşılanır; Linux PipeWire ve xdg-desktop-portal üzerinden ekran ve kamera paylaşımı sorunsuz çalışır.
   - Oturumlar ve Google hesabı kalıcı çerez/veri dizini (`~/.local/share/com.google.meet.desktop`) sayesinde açık kalır.
3. **Pencere ve Sistem Tepsisi (Tray Icon)**:
   - 1280x800 ebatlarında, ekran ortasında açılır ve yeniden boyutlandırılabilir.
   - 'X' kapatma butonuna tıklandığında toplantı kesilmez; pencere arka plana gizlenir.
   - Sistem tepsisinde "Toplantıyı Aç / Gizle" ve "Çıkış" seçenekleri bulunur. Tepsi ikonuna sol tıklayarak da pencere açılıp gizlenebilir.
   - Sadece tepsiden "Çıkış" seçildiğinde uygulama tamamen sonlandırılır.
4. **Arka Plan Ses Oynatımı**:
   - `WEBKIT_DISABLE_BACKGROUND_TAB_THROTTLING=1` ortam değişkeniyle pencere simge durumuna küçültüldüğünde veya gizlendiğinde ses akışı asla dondurulmaz.

---

## 🛠️ Kurulum ve Çalıştırma

### 1. Doğrudan Çalıştırma
Sistem entegrasyonu tamamlandığı için uygulamayı iki şekilde çalıştırabilirsiniz:
- **Terminalden**:
  ```bash
  google-meet
  ```
- **Uygulama Menüsünden**:
  KDE veya GNOME başlat menüsünde **Google Meet** adıyla aratarak açabilirsiniz.

### 2. Kaynak Koddan Yeniden Derleme
Gelecekte kodda bir değişiklik yaparsanız optimize release sürümünü derlemek için:
```bash
cd /home/mustafao/.gemini/antigravity/scratch/google-meet-desktop
cargo build --release
```
`~/.local/bin/google-meet` sembolik bağı otomatik olarak derlenen en güncel dosyaya bağlı kalacaktır.

---

## 📦 Sürüm ve Lisans
- **Sürüm**: v0.1.0 (SemVer)
- **Paket Boyutu**: Sadece ~4.7 MB!
