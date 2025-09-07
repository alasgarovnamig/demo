# policies_old/finance.rego
package finance

import future.keywords.if
import future.keywords.in

# Financial operations
allow_view_transactions if {
    # Main partner can view all transactions
    input.user.can_access_all_partners
}

allow_view_transactions if {
    # Users with finance role can view partner transactions
    "finance_viewer" in input.user.roles
    input.resource.partner_id == input.user.partner_id
}

allow_create_transaction if {
    # Users with finance operator role can create transactions
    "finance_operator" in input.user.roles
    input.resource.partner_id == input.user.partner_id
}

allow_approve_transaction if {
    # Users with finance approver role can approve transactions
    "finance_approver" in input.user.roles
    input.resource.partner_id == input.user.partner_id
    input.resource.amount <= input.user.approval_limit
}