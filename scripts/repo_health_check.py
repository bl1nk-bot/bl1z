#!/usr/bin/env python3
import subprocess
import json
import sys
import re
from datetime import datetime

def run_command(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        return None
    return result.stdout.strip()

def get_orchestrator(branch_name):
    # Detect who ordered the bot/branch creation
    if "dependabot" in branch_name:
        return "System (Security Update)"
    if "session/agent_" in branch_name:
        return "Human Architect (via Gemini CLI)"
    return "Unknown"

def triage_branch(b, is_merged):
    name = b['name']
    age = b['age_days']
    email = b['email']
    author = b['author']
    
    # 1. Identify Identity
    is_team = bool(re.search(r'billlzzz', email, re.I))
    is_bot = "bot" in author.lower() or "dependabot" in name
    
    identity = "TEAM/ARCHITECT" if is_team else ("BOT" if is_bot else "EXTERNAL/HUMAN")
    orchestrator = get_orchestrator(name) if is_bot else "N/A"

    # 2. Decision Logic
    action = "KEEP"
    reason = ""

    if is_merged:
        action = "DELETE (Merged)"
        reason = "Safely integrated into main"
    elif age > 60: # > 2 months
        action = "DELETE (Stale > 2M)"
        reason = "Legacy branch exceeding 2-month threshold"
    elif age > 10:
        action = "WARN (Stale > 10D)"
        reason = "Needs review/audit before deletion"
    
    # Stricter scrutiny for Team
    if is_team:
        reason += " [STRICT SCRUTINY REQUIRED]"

    return {
        "name": name,
        "identity": identity,
        "orchestrator": orchestrator,
        "age": age,
        "action": action,
        "reason": reason,
        "email": email
    }

def main():
    print("🛡️  Autonomous Repository Triage Starting...")
    
    # Get branches with emails
    cmd = "git for-each-ref --sort=-committerdate refs/remotes/origin --format='%(refname:short)|%(committerdate:relative)|%(committerdate:iso8601)|%(authorname)|%(authoremail)'"
    output = run_command(cmd)
    if not output: return

    merged_output = run_command("git branch -r --merged origin/main") or ""
    
    now = datetime.now()
    results = []

    for line in output.split('\n'):
        if 'origin/HEAD' in line: continue
        parts = line.split('|')
        name = parts[0].replace('origin/', '')
        iso_date = datetime.fromisoformat(parts[2].replace(' ', 'T'))
        author = parts[3]
        email = parts[4].strip('<>')
        
        age = (now - iso_date.replace(tzinfo=None)).days
        is_merged = f"origin/{name}" in merged_output and name != 'main'
        
        results.append(triage_branch({
            "name": name, "age_days": age, "author": author, "email": email
        }, is_merged))

    # Display Triage Table
    print(f"{'BRANCH':<40} | {'AGE':<4} | {'IDENTITY':<15} | {'ACTION':<15} | {'REASON'}")
    print("-" * 120)
    
    to_delete = []
    for r in results:
        if r['name'] == 'main': continue
        print(f"{r['name']:<40} | {r['age']:<3}d | {r['identity']:<15} | {r['action']:<15} | {r['reason']}")
        if "DELETE" in r['action']:
            to_delete.append(r['name'])

    if to_delete:
        print(f"\n🚀 Suggested Cleanup Command:")
        print(f"git push origin --delete {' '.join(to_delete)}")
        sys.exit(1)
    
    print("\n✅ Repository is clean and compliant.")
    sys.exit(0)

if __name__ == "__main__":
    main()
