# policies_old/partner.rego
package partner

import future.keywords.if
import future.keywords.in

# Partner creation - only main partner can create new partners
allow_create if {
    input.user.is_main_partner
    input.user.can_access_all_partners
}

# Partner update - main partner or own partner admin
allow_update if {
    input.user.is_main_partner
    input.user.can_access_all_partners
}

allow_update if {
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
}

# Partner view - can view own partner or if has access to all
allow_view if {
    input.resource.partner_id == input.user.partner_id
}

allow_view if {
    input.user.can_access_all_partners
}

# Partner delete - only main partner
allow_delete if {
    input.user.is_main_partner
    input.user.can_access_all_partners
    input.resource.partner_id != input.user.partner_id  # Cannot delete own partner
}