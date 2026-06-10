import re
with open("src/service.rs", "r") as f:
    code = f.read()

# Replace Q::platforms() with Q::tenants()
code = code.replace("Q::platforms()", "Q::tenants()")
code = code.replace("next_id_for::<Platform>()", "next_id_for::<robot_kanban::Tenant>()")

# In get_or_create_platform, rename to get_or_create_tenant
code = code.replace("pub async fn get_or_create_platform", "pub async fn get_or_create_tenant")

# Find the get_or_create_tenant method body to modify it.
# We need to make sure the tenant is created under a platform.
# For simplicity, we can assume a single Platform with ID 1 exists or is created on startup,
# but we can also just set platform_id = 1 for all tenants since it's a multi-tenant system under one platform.
with open("src/service.rs", "w") as f:
    f.write(code)
