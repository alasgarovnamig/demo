# policies_old/api.rego
package api

import future.keywords.if
import future.keywords.in

# API endpoint access control
allow_endpoint if {
    # Check if partner has access to this API endpoint
    endpoint_match
    has_api_permission
}

endpoint_match if {
    # Exact match
    input.resource.endpoint == input.request.path
}

endpoint_match if {
    # Pattern match (e.g., /api/users/* matches /api/users/123)
    pattern := input.resource.endpoint
    startswith(input.request.path, trim_suffix(pattern, "*"))
    endswith(pattern, "*")
}

has_api_permission if {
    # Check if partner has been granted access to this API
    input.resource.api_permission_id in input.user.partner_api_access
}

# Module-based access control
allow_module if {
    # Check if user has access to module
    input.resource.module in input.user.allowed_modules
}

# Rate limiting rules
rate_limit := limit if {
    # Different rate limits based on partner type
    input.user.is_main_partner
    limit := 10000
}

rate_limit := limit if {
    not input.user.is_main_partner
    limit := 1000
}
