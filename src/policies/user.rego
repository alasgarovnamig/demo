# policies_old/user.rego
package user

import future.keywords.if
import future.keywords.in

# User creation
allow_create if {
    # Main partner can create users for any partner
    input.user.is_main_partner
    input.user.can_access_all_partners
}

allow_create if {
    # Partner admin can create users for own partner
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
}

# User update
allow_update if {
    # Users can update their own profile (limited fields)
    input.resource.user_id == input.user.id
    input.action in ["update_profile", "change_password"]
}

allow_update if {
    # Main partner can update any user
    input.user.is_main_partner
    input.user.can_access_all_partners
}

allow_update if {
    # Partner admin can update users in own partner
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
}

# User delete
allow_delete if {
    # Main partner can delete any user
    input.user.is_main_partner
    input.user.can_access_all_partners
    input.resource.user_id != input.user.id  # Cannot delete self
}

allow_delete if {
    # Partner admin can delete users in own partner
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
    input.resource.user_id != input.user.id  # Cannot delete self
}

# User view
allow_view if {
    # Users can view their own profile
    input.resource.user_id == input.user.id
}

allow_view if {
    # Main partner can view any user
    input.user.can_access_all_partners
}

allow_view if {
    # Partner admin can view users in own partner
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
}