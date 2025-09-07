# policies_old/authz.rego
package authz

import future.keywords.if
import future.keywords.in

default allow = false

# Main partner users with full access can do anything
allow if {
    input.user.is_main_partner
    input.user.can_access_all_partners
}

# Users can access their own partner's resources
allow if {
    input.resource.partner_id == input.user.partner_id
    has_required_permission
}

# Partner admins can manage their own partner
allow if {
    input.user.is_admin
    input.resource.partner_id == input.user.partner_id
    input.resource.resource_type in ["user", "role", "permission"]
}

# Check if user has required permission
has_required_permission if {
    required_permission := sprintf("%s:%s", [input.resource.resource_type, input.action])
    required_permission in input.user.permissions
}

# Check permission scope
check_scope(permission) if {
    permission.scope == "all"
}

check_scope(permission) if {
    permission.scope == "partner"
    input.resource.partner_id == input.user.partner_id
}

check_scope(permission) if {
    permission.scope == "own"
    input.resource.owner_id == input.user.id
}
