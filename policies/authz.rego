# policies/authz.rego
# Ana yetkilendirme politikası - tüm izin kontrolleri burada başlar

package authz

# Future keywords'leri import et (Rego'nun yeni özelliklerini kullanmak için)
import future.keywords.if
import future.keywords.in

# Varsayılan olarak erişim reddedilir (güvenlik için önemli!)
default allow = false

# ANA KURAL 1: Ana şirket (UPA) kullanıcıları her şeye erişebilir
allow if {
    # Input'tan gelen user ID'sini al
    user_id := input.user.id

    # Bu user'ı veritabanı verilerinden bul
    user := data.system.users[_]  # "_" herhangi bir index demek
    user.id == user_id

    # User'ın partner'ını bul
    partner := data.system.partners[_]
    partner.id == user.partner_id

    # Eğer partner ana şirketse VE user tüm partnerlere erişebiliyorsa
    partner.is_main_partner == true
    user.can_access_all_partners == true

    # İZİN VER
}

# ANA KURAL 2: Partner API erişimi VE user permission kontrolü
allow if {
    # Önce partner'ın bu API'ye erişimi var mı kontrol et
    check_partner_api_access

    # Sonra user'ın gerekli permission'ları var mı kontrol et
    check_user_permissions

    # İkisi de OK ise İZİN VER
}

# YARDIMCI KURAL: Partner'ın API erişimini kontrol et
check_partner_api_access if {
    # Input'tan gelen bilgileri al
    endpoint := input.resource.resource_id  # Örn: "/api/invoices"
    method := input.action                  # Örn: "POST"
    partner_id := input.user.partner_id

    # Bu endpoint ve method için API permission tanımını bul
    api_perm := data.system.api_permissions[_]
    api_perm.endpoint == endpoint
    api_perm.method == method

    # Partner'ın bu API'ye erişimi var mı kontrol et
    access := data.system.partner_api_access[_]
    access.partner_id == partner_id
    access.api_permission_id == api_perm.id
    access.is_granted == true
}

# YARDIMCI KURAL: User'ın gerekli permission'ları var mı kontrol et
check_user_permissions if {
    # API için gerekli permission'ları bul
    endpoint := input.resource.resource_id
    method := input.action

    api_perm := data.system.api_permissions[_]
    api_perm.endpoint == endpoint
    api_perm.method == method

    # Her gerekli permission için kontrol yap
    required_permission_id := api_perm.required_permissions[_]

    # User'ın bu permission'a sahip olup olmadığını kontrol et
    user_has_permission(input.user.id, required_permission_id)
}

# YARDIMCI FONKSİYON: User'ın belirli bir permission'ı var mı?
user_has_permission(user_id, permission_id) if {
    # User'ın rollerini bul
    user_role := data.system.user_roles[_]
    user_role.user_id == user_id

    # Bu rolün permission'larını bul
    role_perm := data.system.role_permissions[_]
    role_perm.role_id == user_role.role_id
    role_perm.permission_id == permission_id

    # Permission detayını al ve scope kontrolü yap
    permission := data.system.permissions[_]
    permission.id == permission_id

    # Scope kontrolü
    check_permission_scope(permission)
}

# SCOPE KONTROLÜ: Permission'ın kapsamı uygun mu?
check_permission_scope(permission) if {
    # Scope "all" ise - her zaman izin ver (sadece UPA kullanabilir)
    permission.scope == "all"
}

check_permission_scope(permission) if {
    # Scope "partner" ise - kendi partner'ı için mi kontrol et
    permission.scope == "partner"
    input.resource.partner_id == input.user.partner_id
}

check_permission_scope(permission) if {
    # Scope "own" ise - kendi kaynağı mı kontrol et
    permission.scope == "own"
    input.resource.owner_id == input.user.id
}