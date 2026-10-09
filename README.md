# Google Meet Desktop 🚀

CachyOS (Arch Linux) ve diğer Linux dağıtımları için Rust ve Tauri v2 mimarisiyle geliştirilmiş, ultra hafif, modern ve performanslı Google Meet masaüstü istemcisidir.

Tarayıcı sekmeleri arasında kaybolmadan, doğrudan sistem tepsisi (tray) üzerinden yönetilebilen, düşük kaynak tüketimli bağımsız bir Google Meet deneyimi sunar.

---

## ✨ Öne Çıkan Özellikler

- **Bağımsız ve Hafif:** Chrome veya Electron gibi ağır tarayıcı motorları yerine işletim sisteminin yerleşik WebKitGTK motorunu ve Rust'ı kullanır. Sadece birkaç megabayt bellek harcar!
- **Sistem Tepsisi (Tray) Entegrasyonu:** Uygulama penceresini kapatsanız dahi kapanmaz, sistem tepsisinde arka planda çalışmaya devam eder. Toplantılarınız ve sesiniz asla kesilmez!
- **Gelişmiş Tarayıcı Kimliği (Spoofing):** WebKitGTK motoru üzerinden Google'a Firefox/Chrome gibi modern tarayıcı kimlikleriyle bağlanır. "Desteklenmeyen Tarayıcı" hatasını tamamen ortadan kaldırır.
- **Kesintisiz Medya Oynatımı:** Arka plana alındığında veya simge durumuna küçültüldüğünde WebKit'in arka plan uyutma modunu (throttling) devre dışı bırakır. Böylece toplantı sesi ve mikrofonunuz daima aktif kalır.
- **Kusursuz İzin Yönetimi:** Mikrofon, kamera ve PipeWire (Wayland) üzerinden ekran paylaşımı izinleri otomatik ve güvenli bir şekilde WebKit ayarlarına bağlanmıştır.

## 🛠️ Sistem Gereksinimleri

- Linux işletim sistemi (Arch Linux, CachyOS vb. önerilir)
- `webkit2gtk-4.1` (WebKitGTK kütüphanesi)
- Rust (derleme için)

## 📦 Kurulum ve Çalıştırma

### 1. Doğrudan Çalıştırma
Uygulamayı çalıştırılabilir dosya olarak sisteminize entegre edebilirsiniz:
```bash
google-meet
```
Ayrıca, KDE veya GNOME uygulama menünüzde "Google Meet" adıyla aratarak başlatabilirsiniz.

### 2. Kaynak Koddan Yeniden Derleme
Kendi sisteminizde en yüksek optimizasyonla (release profile) derlemek isterseniz:
```bash
git clone https://github.com/mustozilla11/google-meet-desktop.git
cd google-meet-desktop
cargo build --release
```
Derlenen ikili dosya `target/release/google-meet-desktop` yolunda olacaktır.

---

## tray Menüsü

Sistem tepsisinde yer alan Google Meet ikonuna tıklayarak şu hızlı işlemlere erişebilirsiniz:
- **Toplantıyı Aç / Gizle:** Ana pencereyi görünür kılar veya arka plana gönderir.
- **Arama Yap ya da Katıl:** Doğrudan toplantı arama ve oluşturma sayfasına atlar.
- **Yeni Toplantı Başlat:** `meet.new` kısayoluyla anında yeni bir toplantı başlatır.
- **Oturum Aç:** Google hesabınızla giriş yapma sayfasına yönlendirir.
- **Çıkış:** Uygulamayı tamamen kapatır.

## 📝 Sürüm Geçmişi

- **v0.1.6:** WebRTC (Görüntülü Görüşme) API uyumluluğu için User-Agent yapısı güncellendi.
- **v0.1.5:** WebRTC ayarlarının (RTCPeerConnection) sayfa yüklenmeden önce başlatılması düzeltildi.
- **v0.1.4:** Boş ekran hatalarına karşı WebKit ayarları ve DMABUF renderer devredışı bırakıldı.
- **v0.1.3:** Ana ekran yönlendirme hataları (`404 Not Found`) giderildi.
- **v0.1.0:** İlk stabil sürüm yayınlandı. (Tauri v2 + WebKitGTK)

---
*Rust ve Tauri ile sevgiyle geliştirilmiştir. 🦀*
