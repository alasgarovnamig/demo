# policies_old/authz.rego
package authz

import future.keywords.if
import future.keywords.in

default allow = false

# OPA'ya yüklenen veriler
# data.system.partners
# data.system.users
# data.system.roles
# data.system.permissions
# data.system.user_roles
# data.system.role_permissions

# Main partner users with full access can do anything
allow if {
    user := data.system.users[_]
    user.id == input.user.id
    user.can_access_all_partners == true
    partner := data.system.partners[_]
    partner.id == user.partner_id
    partner.is_main_partner == true
}

# Check if user has required permission through roles
allow if {
    has_permission[_]
}

# Get user's permissions through roles
has_permission[permission_id] if {
    # Find user's roles
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id

    # Find role's permissions
    role_permission := data.system.role_permissions[_]
    role_permission.role_id == user_role.role_id

    # Get permission details
    permission := data.system.permissions[_]
    permission.id == role_permission.permission_id

    # Check if permission matches request
    permission.resource == input.resource.resource_type
    permission.action == input.action

    # Check scope
    check_scope(permission)

    permission_id := permission.id
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

# Get user's roles
user_roles[role] if {
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id
    role_obj := data.system.roles[_]
    role_obj.id == user_role.role_id
    role := role_obj.code
}

# Check if user is partner admin
is_partner_admin if {
    "partner_admin" in user_roles[_]
}

is_partner_admin if {
    user := data.system.users[_]
    user.id == input.user.id
    user.is_admin == true
}

# Get user's partner
user_partner := partner if {
    user := data.system.users[_]
    user.id == input.user.id
    partner := data.system.partners[_]
    partner.id == user.partner_id
}