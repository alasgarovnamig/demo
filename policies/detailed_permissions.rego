# policies/detailed_permissions.rego
package detailed_permissions

import future.keywords.if
import future.keywords.in

# User management permissions
allow_user_create if {
    # User must have users:create permission
    permission := data.system.permissions[_]
    permission.resource == "users"
    permission.action == "create"

    # Check if user has this permission through roles
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id

    role_permission := data.system.role_permissions[_]
    role_permission.role_id == user_role.role_id
    role_permission.permission_id == permission.id

    # Check target partner
    input.resource.partner_id == input.user.partner_id
}

allow_user_create if {
    # Main partner users can create users for any partner
    user := data.system.users[_]
    user.id == input.user.id
    user.can_access_all_partners == true
}

# Partner management permissions
allow_partner_create if {
    # Only main partner can create new partners
    user := data.system.users[_]
    user.id == input.user.id
    partner := data.system.partners[_]
    partner.id == user.partner_id
    partner.is_main_partner == true
}

# Role assignment permissions
allow_role_assign if {
    # Get user's roles
    assigner_roles := { role_id |
        user_role := data.system.user_roles[_]
        user_role.user_id == input.user.id
        role_id := user_role.role_id
    }

    # Check if the role being assigned is in user's roles
    input.resource.role_id in assigner_roles
}

allow_role_assign if {
    # Partner admins can assign any role within their partner
    user := data.system.users[_]
    user.id == input.user.id
    user.is_admin == true

    # Check if role belongs to partner or is system role
    role := data.system.roles[_]
    role.id == input.resource.role_id
    role_valid := role.is_system_role == true
}

allow_role_assign if {
    # Partner admins can assign any role within their partner
    user := data.system.users[_]
    user.id == input.user.id
    user.is_admin == true

    # Check if role belongs to partner
    role := data.system.roles[_]
    role.id == input.resource.role_id
    role.partner_id == user.partner_id
}

# Financial permissions with amount limits
allow_financial_approve if {
    # Check if user has finance_approver role
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id

    role := data.system.roles[_]
    role.id == user_role.role_id
    role.code == "finance_approver"

    # Check amount limit (example: 10000)
    input.resource.amount <= 10000
}

allow_financial_approve if {
    # Higher approval for larger amounts
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id

    role := data.system.roles[_]
    role.id == user_role.role_id
    role.code == "finance_director"

    # Higher limit for directors
    input.resource.amount <= 100000
}

# Cross-partner operations
allow_cross_partner_operation if {
    # User must have cross_partner_operator role
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id

    role := data.system.roles[_]
    role.id == user_role.role_id
    role.code == "cross_partner_operator"

    # User must be from main partner
    user := data.system.users[_]
    user.id == input.user.id
    partner := data.system.partners[_]
    partner.id == user.partner_id
    partner.is_main_partner == true
}