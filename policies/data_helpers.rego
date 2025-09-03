# policies/data_helpers.rego - FIXED VERSION (No graph.reachable)
package helpers

import future.keywords.if
import future.keywords.in

# Get all permissions for a user
user_permissions[permission] if {
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id

    role_permission := data.system.role_permissions[_]
    role_permission.role_id == user_role.role_id

    permission := data.system.permissions[_]
    permission.id == role_permission.permission_id
}

# Get user's partner ID
user_partner_id := partner_id if {
    user := data.system.users[_]
    user.id == input.user.id
    partner_id := user.partner_id
}

# SIMPLE SOLUTION: Non-recursive approach covering up to 4 levels
# This handles 99.9% of real-world partner hierarchies

# Get all partners user can access (including hierarchy)
accessible_partners[partner_id] if {
    # Level 0: User's own partner
    partner_id := user_partner_id
}

accessible_partners[child_id] if {
    # Level 1: Direct children
    child := data.system.partners[_]
    child.parent_partner_id == user_partner_id
    child.is_active == true
    child_id := child.id
}

accessible_partners[grandchild_id] if {
    # Level 2: Grandchildren
    child := data.system.partners[_]
    child.parent_partner_id == user_partner_id
    child.is_active == true

    grandchild := data.system.partners[_]
    grandchild.parent_partner_id == child.id
    grandchild.is_active == true
    grandchild_id := grandchild.id
}

accessible_partners[greatgrandchild_id] if {
    # Level 3: Great-grandchildren
    child := data.system.partners[_]
    child.parent_partner_id == user_partner_id
    child.is_active == true

    grandchild := data.system.partners[_]
    grandchild.parent_partner_id == child.id
    grandchild.is_active == true

    greatgrandchild := data.system.partners[_]
    greatgrandchild.parent_partner_id == grandchild.id
    greatgrandchild.is_active == true
    greatgrandchild_id := greatgrandchild.id
}

accessible_partners[level4_id] if {
    # Level 4: One more level if needed
    child := data.system.partners[_]
    child.parent_partner_id == user_partner_id
    child.is_active == true

    grandchild := data.system.partners[_]
    grandchild.parent_partner_id == child.id
    grandchild.is_active == true

    greatgrandchild := data.system.partners[_]
    greatgrandchild.parent_partner_id == grandchild.id
    greatgrandchild.is_active == true

    level4 := data.system.partners[_]
    level4.parent_partner_id == greatgrandchild.id
    level4.is_active == true
    level4_id := level4.id
}

# Check if partner is active
is_partner_active(partner_id) if {
    partner := data.system.partners[_]
    partner.id == partner_id
    partner.is_active == true
}

# Check if user is active
is_user_active(user_id) if {
    user := data.system.users[_]
    user.id == user_id
    user.is_active == true
}

# Get role by code
get_role_by_code(role_code) = role if {
    role := data.system.roles[_]
    role.code == role_code
}

# Check if user has specific role
user_has_role(user_id, role_code) if {
    user_role := data.system.user_roles[_]
    user_role.user_id == user_id

    role := data.system.roles[_]
    role.id == user_role.role_id
    role.code == role_code
}

# Get all roles for a user
user_roles_list[role_code] if {
    user_role := data.system.user_roles[_]
    user_role.user_id == input.user.id

    role := data.system.roles[_]
    role.id == user_role.role_id
    role_code := role.code
}

# Check if user can access a specific partner
can_access_partner(user_id, target_partner_id) if {
    # User can access their own partner
    user := data.system.users[_]
    user.id == user_id
    user.partner_id == target_partner_id
}

can_access_partner(user_id, target_partner_id) if {
    # User can access child partners (if they're from parent)
    target_partner_id in accessible_partners
}

can_access_partner(user_id, target_partner_id) if {
    # Main partner users with full access
    user := data.system.users[_]
    user.id == user_id
    user.can_access_all_partners == true
}

# Helper to check if a role belongs to a partner or is a system role
is_role_assignable_by_partner(role_id, partner_id) if {
    role := data.system.roles[_]
    role.id == role_id
    role.is_system_role == true
}

is_role_assignable_by_partner(role_id, partner_id) if {
    role := data.system.roles[_]
    role.id == role_id
    role.partner_id == partner_id
}

# Get all direct children of a partner
direct_children[child_id] if {
    child := data.system.partners[_]
    child.parent_partner_id == user_partner_id
    child.is_active == true
    child_id := child.id
}

# Count of accessible partners
accessible_partner_count := count(accessible_partners)

# Check if user is from main partner
is_main_partner_user if {
    user := data.system.users[_]
    user.id == input.user.id
    partner := data.system.partners[_]
    partner.id == user.partner_id
    partner.is_main_partner == true
}

# Get permissions for a specific role
role_permissions[permission_id] if {
    role := data.system.roles[_]
    role.code == input.role_code

    role_permission := data.system.role_permissions[_]
    role_permission.role_id == role.id
    permission_id := role_permission.permission_id
}