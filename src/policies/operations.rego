# policies/operations.rego
package operations

import future.keywords.if
import future.keywords.in

# Operational permissions
allow_view_operations if {
    # Main partner can view all operations
    input.user.can_access_all_partners
}

allow_view_operations if {
    # Users with operations role can view partner operations
    "operations_viewer" in input.user.roles
    input.resource.partner_id == input.user.partner_id
}

allow_manage_operations if {
    # Users with operations manager role can manage operations
    "operations_manager" in input.user.roles
    input.resource.partner_id == input.user.partner_id
}

# Cross-partner operations (for main partner users)
allow_cross_partner_operation if {
    input.user.is_main_partner
    input.user.can_access_all_partners
    "cross_partner_operator" in input.user.roles
}