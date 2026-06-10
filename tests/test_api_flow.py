import requests
import sys

BASE_URL = "http://localhost:3000/api"

def print_step(msg):
    print(f"\n--- {msg} ---")

def test_api_flow():
    print_step("Checking initial tenants")
    r = requests.get(f"{BASE_URL}/admin/tenants")
    r.raise_for_status()
    initial_tenants = r.json()
    print(f"Initial tenants count: {len(initial_tenants)}")

    tenant_a = "tenant-a-test"
    tenant_b = "tenant-b-test"

    print_step(f"Creating tasks for {tenant_a}")
    r1 = requests.post(f"{BASE_URL}/tasks", headers={"x-session-id": tenant_a}, json={"name": "Task 1 (A)"})
    r1.raise_for_status()
    task1_a = r1.json()["id"]
    print(f"Created Task 1 (A): {task1_a}")

    r2 = requests.post(f"{BASE_URL}/tasks", headers={"x-session-id": tenant_a}, json={"name": "Task 2 (A)"})
    r2.raise_for_status()
    task2_a = r2.json()["id"]
    print(f"Created Task 2 (A): {task2_a}")

    print_step(f"Creating task for {tenant_b}")
    r3 = requests.post(f"{BASE_URL}/tasks", headers={"x-session-id": tenant_b}, json={"name": "Task 1 (B)"})
    r3.raise_for_status()
    task1_b = r3.json()["id"]
    print(f"Created Task 1 (B): {task1_b}")

    print_step(f"Listing tasks for {tenant_a}")
    r_list = requests.get(f"{BASE_URL}/tasks", headers={"x-session-id": tenant_a})
    r_list.raise_for_status()
    data_a = r_list.json()
    tasks_a_count = len(data_a["planned_tasks"]) + len(data_a["ready_tasks"]) + len(data_a["executing_tasks"]) + len(data_a["verified_tasks"])
    print(f"Tasks for {tenant_a}: {tasks_a_count}")
    assert tasks_a_count >= 2, "Tenant A should have at least 2 tasks"

    print_step(f"Listing tasks for {tenant_b}")
    r_list_b = requests.get(f"{BASE_URL}/tasks", headers={"x-session-id": tenant_b})
    r_list_b.raise_for_status()
    data_b = r_list_b.json()
    tasks_b_count = len(data_b["planned_tasks"]) + len(data_b["ready_tasks"]) + len(data_b["executing_tasks"]) + len(data_b["verified_tasks"])
    print(f"Tasks for {tenant_b}: {tasks_b_count}")
    assert tasks_b_count >= 1, "Tenant B should have at least 1 task"

    print_step(f"Updating task {task1_a} status")
    r_put = requests.put(f"{BASE_URL}/tasks/{task1_a}/move", headers={"x-session-id": tenant_a}, json={"status": "IN_PROGRESS"})
    r_put.raise_for_status()
    print("Status updated successfully")

    print_step("Checking admin tenants dashboard")
    r_admin = requests.get(f"{BASE_URL}/admin/tenants")
    r_admin.raise_for_status()
    tenants = r_admin.json()
    
    tenant_a_name = f"Session {tenant_a}"
    tenant_b_name = f"Session {tenant_b}"
    tenant_a_data = next((t for t in tenants if t["name"] == tenant_a_name), None)
    tenant_b_data = next((t for t in tenants if t["name"] == tenant_b_name), None)
    
    assert tenant_a_data is not None, f"Tenant {tenant_a} not found in admin view"
    assert tenant_b_data is not None, f"Tenant {tenant_b} not found in admin view"
    
    print(f"Tenant A Task Count: {tenant_a_data['task_count']}, Recent Tasks: {[t['name'] for t in tenant_a_data['recent_tasks']]}")
    print(f"Tenant B Task Count: {tenant_b_data['task_count']}, Recent Tasks: {[t['name'] for t in tenant_b_data['recent_tasks']]}")
    
    assert tenant_a_data["task_count"] >= 2, "Tenant A task count incorrect"
    assert tenant_b_data["task_count"] >= 1, "Tenant B task count incorrect"
    assert len(tenant_a_data["recent_tasks"]) > 0, "Tenant A recent tasks missing"

    print_step("ALL TESTS PASSED!")

if __name__ == "__main__":
    test_api_flow()
