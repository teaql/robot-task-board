import os
import re
import subprocess
import json

def run_test(format_type, log_file):
    print(f"Running observe_log with TEAQL_LOG_FORMAT={format_type} into {log_file}...")
    if os.path.exists("observe_test.db"):
        os.remove("observe_test.db")
    if os.path.exists(log_file):
        os.remove(log_file)
        
    env = os.environ.copy()
    env["TEAQL_LOG_FORMAT"] = format_type
    env["TEAQL_LOG_ENDPOINT"] = log_file
    
    result = subprocess.run(
        ["cargo", "run", "--bin", "observe_log"],
        env=env,
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        print(f"Error running observe_log for {format_type}: {result.stderr}")
        return False
    return True

def evaluate_human_log(log_file):
    print(f"\n--- Evaluating {log_file} (Human Readable) ---")
    with open(log_file, "r", encoding="utf-8") as f:
        content = f.read()
        
    # Check 1: No Chinese characters
    if re.search(r'[\u4e00-\u9fff]', content):
        print("❌ FAILED: Chinese characters found in the log.")
        return False
    print("✅ PASSED: No Chinese characters found.")
    
    # Check 2: Trace chain formatting (contains ->)
    if "->" not in content:
        print("❌ FAILED: Trace chain separator '->' not found.")
        return False
    print("✅ PASSED: Trace chain separator '->' is present.")
    
    # Check 3: SQL Redundancy (version > 0) AND (version > 0)
    if "(version > 0) AND (version > 0)" in content or "(version > 0)   AND (version > 0)" in content:
        print("❌ FAILED: Redundant '(version > 0) AND (version > 0)' found in SQL.")
        return False
    print("✅ PASSED: No redundant '(version > 0)' filters found.")

    # Check 4: Time format
    if not re.search(r'\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3}\]', content):
        print("❌ FAILED: Standard time format [YYYY-MM-DD HH:MM:SS.ms] not found.")
        return False
    print("✅ PASSED: Standard time format is correct.")
    
    return True

def evaluate_json_log(log_file):
    print(f"\n--- Evaluating {log_file} (Machine/Debug Readable) ---")
    with open(log_file, "r", encoding="utf-8") as f:
        lines = f.readlines()
        
    if not lines:
        print("❌ FAILED: Log file is empty.")
        return False
        
    # Check 1: Valid Debug Log
    valid_count = 0
    for line in lines:
        line = line.strip()
        if not line: continue
        if line.startswith("[SQL_LOG]") or line.startswith("[AUDIT_LOG]"):
            valid_count += 1
        else:
            print(f"❌ FAILED: Line is not a valid structured log. Line: {line}")
            return False
            
    print(f"✅ PASSED: All {valid_count} lines are valid structured logs.")
    return True

if __name__ == "__main__":
    success = True
    
    # Run and test Human Log
    if run_test("human", "eval_human_output.log"):
        if not evaluate_human_log("eval_human_output.log"):
            success = False
    else:
        success = False
        
    # Run and test JSON Log
    if run_test("json", "eval_json_output.log"):
        if not evaluate_json_log("eval_json_output.log"):
            success = False
    else:
        success = False
        
    if success:
        print("\n🎉 ALL EVALUATION TESTS PASSED SUCCESSFULLY! 🎉")
    else:
        print("\n❌ SOME EVALUATION TESTS FAILED.")
