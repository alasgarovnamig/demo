# policies/role.rego
package role

import future.keywords.if
import future.keywords.in

# Role assignment
allow_assign if {
    # Main partner can assign any role
    input.user.is_main_partner
    input.user.can_access_all_partners
}

allow_assign if {
    # Partner admin can only assign roles they have
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
    input.resource.role_code in input.user.roles
}

# Role creation
allow_create if {
    # Main partner can create system roles
    input.user.is_main_partner
    input.user.can_access_all_partners
}

allow_create if {
    # Partner admin can create partner-specific roles
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
    not input.resource.is_system_role
}

# Role deletion
allow_delete if {
    # Main partner can delete any role
    input.user.is_main_partner
    input.user.can_access_all_partners
}

allow_delete if {
    # Partner admin can delete partner-specific roles
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
    not input.resource.is_system_role
}