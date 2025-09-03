# Partner Yetkilendirme Sistemi

## Sistem Nedir?

**Düşünün ki bir AVM yönetim sistemisiniz:**
- **Siz** → AVM Yönetimi (Ana Şirket)
- **Partnerler** → Mağazalar (Zara, Nike, McDonald's...)
- **Kullanıcılar** → Mağaza çalışanları
- **Yetkiler** → Kimin neyi yapabileceği (Kasa açma, ürün ekleme, rapor görme...)

Bu sistem, her mağazanın sadece kendi verilerini görmesini, kendi çalışanlarını yönetmesini sağlar. AVM yönetimi ise tüm mağazaları kontrol edebilir.

---

## Problem ve Çözüm

### Problem
Bir yazılım şirketisiniz ve müşterilerinize (partnerlere) hizmet veriyorsunuz. Her müşteri:
- Kendi kullanıcılarını yönetmek istiyor
- Sadece kendi verilerini görmek istiyor
- Farklı çalışanlarına farklı yetkiler vermek istiyor

### Çözüm
Bu sistem sayesinde:
- Her partner kendi "küçük krallığını" yönetir
- Siz ise tüm sistemi kontrol edersiniz
- Herkes sadece yetkisi kadar işlem yapabilir

---

## Sistem Bileşenleri

### 1. PARTNERS (Şirketler/Müşteriler)

#### Ne Tutar?
```
Ana Şirket (SİZ)
   ├── Partner A (Tekstil Firması)
   ├── Partner B (Restoran Zinciri)
   └── Partner C (E-ticaret Sitesi)
```

#### Gerçek Hayat Örneği
McDonald's gibi düşünün:
- **Ana Şirket**: McDonald's Corporation
- **Partner A**: İstanbul Franchise Sahibi
- **Partner B**: Ankara Franchise Sahibi

#### Örnek Veri
| Şirket Adı | Tipi | Ana Şirket mi? | Aktif mi? |
|------------|------|----------------|-----------|
| TechCorp (SİZ) | Ana | Evet | Evet |
| Moda AŞ | Standart | Hayır | Evet |
| Yemek Ltd | Standart | Hayır | Evet |

---

### 2. USERS (Kullanıcılar)

#### Ne Tutar?
Her şirketin çalışanları:
```
Partner A (Moda AŞ)
   ├── Ali (Müdür)
   ├── Ayşe (Muhasebe)
   └── Mehmet (Satış)
```

#### Kullanıcı Tipleri

| Tip | Ne Yapar? | Örnek |
|-----|-----------|-------|
| **Admin** | Her şeyi yönetir | Şirket sahibi |
| **Portal** | Web üzerinden işlem yapar | Muhasebeci |
| **System** | Otomatik işlemler | API robotu |
| **Operator** | Günlük işlemler | Satış elemanı |

#### Örnek
```
Ali Yılmaz
- E-posta: ali@modaas.com
- Şirket: Moda AŞ
- Tip: Admin
- Tüm partnerlere erişim: Hayır (Sadece kendi şirketi)
```

---

### 3. ROLES (Roller/Görevler)

#### Ne Tutar?
Şirketteki pozisyonlar gibi:
```
Sistem Rolleri (Herkes kullanabilir)
   ├── Süper Admin (Sadece siz)
   ├── Partner Admin (Şirket yöneticisi)
   ├── Muhasebe Müdürü
   └── Rapor Görüntüleyici
```

#### Rol = Yetki Paketi
**Muhasebe Müdürü** rolü = 
- Fatura oluşturabilir
- Fatura düzenleyebilir
- Rapor görebilir
- Kullanıcı silemez

#### Örnek
| Rol Adı | Açıklama | Sistem Rolü mü? |
|---------|----------|-----------------|
| Süper Admin | Her şeyi yapabilir | Evet |
| Muhasebe Müdürü | Mali işlemler | Evet |
| Özel Onaylayıcı | Partner A'ya özel | Hayır |

---

### 4. PERMISSIONS (İzinler/Yetkiler)

#### Ne Tutar?
Sistemde yapılabilecek her işlem:
```
Kaynak: Faturalar
   ├── create (Oluştur)
   ├── read (Görüntüle)
   ├── update (Düzenle)
   ├── delete (Sil)
   └── approve (Onayla)
```

#### Kapsam (Scope) Nedir?

| Kapsam | Ne Demek? | Örnek |
|--------|-----------|-------|
| **Own** | Sadece kendin oluşturdukların | Ali sadece kendi faturalarını görebilir |
| **Partner** | Tüm şirket verileri | Ali tüm Moda AŞ faturalarını görebilir |
| **All** | Tüm sistem | Sadece siz tüm faturaları görebilirsiniz |

#### Örnek İzin
```
İzin: "Fatura Oluşturma"
- Kaynak: invoices
- İşlem: create
- Kapsam: partner (Kendi şirketinde oluşturabilir)
```

---

### 5. USER_ROLES (Kullanıcı-Rol Eşleşmesi)

#### Ne Tutar?
Hangi kullanıcının hangi görevi var:
```
Ali Yılmaz
   ├── Partner Admin (15.01.2024'te atandı)
   └── Muhasebe Müdürü (20.01.2024'te atandı)
```

#### Örnek
| Kullanıcı | Rol | Atanma Tarihi | Kim Atadı |
|-----------|-----|---------------|-----------|
| Ali | Partner Admin | 15.01.2024 | Sistem |
| Ali | Muhasebe Müdürü | 20.01.2024 | CEO |
| Ayşe | Rapor Görüntüleyici | 01.02.2024 | Ali |

---

### 6. ROLE_PERMISSIONS (Rol-Yetki Eşleşmesi)

#### Ne Tutar?
Her rolün hangi yetkileri var:
```
Muhasebe Müdürü Rolü
   ├── Fatura oluşturma
   ├── Fatura düzenleme
   ├── Fatura onaylama
   └── Rapor görüntüleme
```

#### Örnek
| Rol | Yetki | Ne Yapabilir? |
|-----|-------|---------------|
| Muhasebe Müdürü | invoices:create | Fatura oluşturabilir |
| Muhasebe Müdürü | invoices:approve | Fatura onaylayabilir |
| Satış Elemanı | invoices:read | Sadece fatura görebilir |

---

### 7. API_PERMISSIONS (API Yetkileri)

#### Ne Tutar?
Hangi API adresi hangi yetkiyi istiyor:
```
API: /api/invoices
   ├── GET (Görüntüle) → "invoices:read" yetkisi gerekli
   ├── POST (Oluştur) → "invoices:create" yetkisi gerekli
   └── DELETE (Sil) → "invoices:delete" + "admin" yetkisi gerekli
```

#### Örnek
| API Adresi | Method | Gerekli Yetki | Modül |
|------------|--------|---------------|-------|
| /api/invoices | GET | invoices:read | Finance |
| /api/invoices | POST | invoices:create | Finance |
| /api/users | DELETE | users:delete + admin | Admin |

---

### 8. PARTNER_API_ACCESS (Partner API Erişimleri)

#### Ne Tutar?
Hangi partner hangi API'leri kullanabilir:
```
Moda AŞ
   ├── Fatura API'leri (İzin var)
   ├── Rapor API'leri (İzin var)
   └── Admin API'leri (İzin yok)
```

#### Örnek
| Partner | API | Erişim | Kim Verdi | Ne Zaman |
|---------|-----|--------|-----------|----------|
| Moda AŞ | /api/invoices/* | Var | Süper Admin | 01.01.2024 |
| Moda AŞ | /api/admin/* | Yok | - | - |
| Yemek Ltd | /api/invoices/* | Var | Süper Admin | 15.01.2024 |

---

### 9. AUDIT_LOGS (Denetim Kayıtları)

#### Ne Tutar?
Kim, ne zaman, ne yaptı:
```
İşlem Kayıtları
├── 10:30 - Ali fatura oluşturdu (INV-001)
├── 10:45 - Ayşe rapor indirdi (RPT-055)
└── 11:00 - Mehmet kullanıcı güncelledi (USR-123)
```

#### Örnek
| Tarih/Saat | Kullanıcı | İşlem | Kaynak | Detay |
|------------|-----------|-------|--------|-------|
| 15.01.2024 10:30 | ali@modaas.com | CREATE | invoice | INV-001 oluşturuldu |
| 15.01.2024 10:45 | ayse@modaas.com | EXPORT | report | Aylık rapor indirildi |
| 15.01.2024 11:00 | admin@techcorp.com | DELETE | user | USR-456 silindi |

---

## Sistem Nasıl Çalışır?

### Basit Bir Hikaye

**Senaryo**: Ali (Moda AŞ çalışanı) fatura oluşturmak istiyor

#### Adım 1: Giriş Yapma
```
Ali: "Sisteme girmek istiyorum"
     ↓
Sistem: "Tamam, şifren doğru. Al sana bilet (JWT Token)"
```

#### Adım 2: Bilet İçeriği
```json
{
  "kim": "Ali",
  "şirket": "Moda AŞ",
  "rolleri": ["Muhasebe Müdürü"],
  "yetkiler": ["fatura_oluştur", "fatura_gör"]
}
```

#### Adım 3: Fatura Oluşturma İsteği
```
Ali: "Fatura oluşturmak istiyorum"
     ↓
Sistem Kontrolleri:
1. Biletin geçerli mi? (Token kontrolü)
2. Muhasebe Müdürü müsün? (Rol kontrolü)
3. Fatura oluşturma yetkin var mı? (Yetki kontrolü)
4. Moda AŞ'nin fatura API erişimi var mı? (API erişim kontrolü)
5. Kendi şirketin için mi oluşturuyorsun? (Kapsam kontrolü)
     ↓
Sistem: "Tamam, faturan oluşturuldu!"
     ↓
Sistem: "Bu işlemi kayıt defterine yazdım" (Audit log)
```

---

## Kullanım Senaryoları

### Senaryo 1: Yeni Partner Ekleme
```
Süper Admin → Yeni partner oluşturur
          → Otomatik admin kullanıcı oluşur
          → Partner kendi kullanıcılarını ekler
          → Kullanıcılara rol atar
          → Sistem çalışmaya başlar
```

### Senaryo 2: Yetki Verme
```
Partner Admin → Yeni çalışan ekler
             → "Satış Elemanı" rolünü atar
             → Çalışan sadece satış işlemleri yapabilir
```

### Senaryo 3: Raporlama
```
CEO → Tüm partnerların raporlarını görür
Partner Admin → Sadece kendi şirket raporlarını görür
Çalışan → Sadece izin verilen raporları görür
```

---

## Kurulum ve Başlangıç

### Gereksinimler
- PostgreSQL veritabanı
- Rust programlama dili
- Docker (opsiyonel)

### Hızlı Başlangıç
```bash
# 1. Projeyi indirin
git clone <proje-adresi>

# 2. Veritabanını başlatın
docker-compose up -d postgres

# 3. Programı çalıştırın
cargo run

# 4. Tarayıcıdan açın
http://localhost:8080
```

### İlk Giriş
```
E-posta: admin@maincompany.com
Şifre: Admin@123
```

---

## Örnek Veri Akışı

```
Kullanıcı → Giriş Yapar → Authentication
         ↓
    Token Alır → JWT Token
         ↓
   İstek Gönderir → API Endpoint
         ↓
    Kontrol 1 → Token Geçerli mi?
         ↓
    Kontrol 2 → Rolü Var mı?
         ↓
    Kontrol 3 → Yetkisi Var mı?
         ↓
    Kontrol 4 → API Erişimi Var mı?
         ↓
    Kontrol 5 → Kapsam Uygun mu?
         ↓
      Onay → İşlem Yapılır
         ↓
      Kayıt → Audit Log
```

---

## Sık Sorulan Sorular

### S: Partner nedir?
**C:** Sizin müşteriniz olan şirketler. Sisteminizi kullanan farklı organizasyonlar.

### S: Bir kullanıcı birden fazla rol alabilir mi?
**C:** Evet! Ali hem "Muhasebe Müdürü" hem "Rapor Yöneticisi" olabilir.

### S: Ana şirket ne demek?
**C:** Sistemi yöneten şirket (SİZ). Tüm partnerleri görebilir ve yönetebilirsiniz.

### S: Scope (kapsam) ne işe yarar?
**C:** Yetkinin ne kadar geniş olduğunu belirler:
- **Own**: Sadece kendi oluşturdukların
- **Partner**: Tüm şirket
- **All**: Tüm sistem

### S: Audit log neden önemli?
**C:** Kim ne zaman ne yaptı kayıt altında. Güvenlik ve yasal zorunluluklar için kritik.

---

## Özet

Bu sistem bir **apartman yönetimi** gibidir:
- **Siz**: Bina yöneticisi (her daireyi görebilirsiniz)
- **Partnerler**: Daireler (sadece kendi dairelerini görebilirler)
- **Kullanıcılar**: Daire sakinleri
- **Roller**: Ev sahibi, kiracı, misafir
- **Yetkiler**: Kapı açma, otopark kullanma, ortak alan kullanma
- **Audit Log**: Giriş-çıkış kayıtları

Her daire kendi kurallarını koyar, kendi sakinlerini yönetir ama bina kurallarına uymak zorundadır!

---

## Destek

Sorularınız için: support@techcorp.com

---

*Bu doküman, sistemin temel mantığını açıklar. Teknik detaylar için API dokümantasyonuna bakınız.*