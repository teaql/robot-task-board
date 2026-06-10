let tasks = [
  { id: 1001, name: "Calibrate arm sensors", status: "PLANNED" },
  { id: 1002, name: "Load pallet alpha", status: "READY" },
  { id: 1003, name: "Transport parts to sector B", status: "EXECUTING" },
  { id: 1004, name: "System diagnostic check", status: "VERIFIED" }
];

let nextId = 1005;

const statuses = ["PLANNED", "READY", "EXECUTING", "VERIFIED"];

const colElements = {
  PLANNED: document.getElementById("col-planned"),
  READY: document.getElementById("col-ready"),
  EXECUTING: document.getElementById("col-executing"),
  VERIFIED: document.getElementById("col-verified")
};

const titleElements = {
  PLANNED: document.getElementById("title-planned"),
  READY: document.getElementById("title-ready"),
  EXECUTING: document.getElementById("title-executing"),
  VERIFIED: document.getElementById("title-verified")
};

const cmdInput = document.getElementById("cmd-input");
const logContent = document.getElementById("log-content");

function getTimestamp() {
  return new Date().toISOString().replace(/\.\d{3}Z$/, 'Z');
}

function addLog(level, message) {
  const el = document.createElement("div");
  el.className = "log-line";
  const colorClass = level === "INFO" ? "log-level-info" : (level === "WARN" ? "" : ""); // simplified
  el.innerHTML = `<span class="log-time">[${getTimestamp()}]</span> - <span class="${colorClass}">[${level}]</span> - ${message}`;
  logContent.appendChild(el);
  logContent.scrollTop = logContent.scrollHeight;
}

function renderTasks() {
  // Clear lists
  for (const status of statuses) {
    colElements[status].innerHTML = "";
    let count = 0;
    
    tasks.filter(t => t.status === status).forEach(task => {
      count++;
      const el = document.createElement("div");
      el.className = "task-item";
      el.innerHTML = `<span class="task-id">  ${task.id.toString().padStart(4, ' ')}  </span><span class="task-name">${task.name}</span>`;
      colElements[status].appendChild(el);
    });

    // Update title counts
    titleElements[status].textContent = ` ${status} [ ${count} ] `;
  }
}

function handleCommand(cmdStr) {
  const trimmed = cmdStr.trim();
  if (trimmed === "") return;

  // Setup command and args similar to Rust
  let cmd = "add";
  let args = trimmed;

  if (trimmed.startsWith("/")) {
    const withoutSlash = trimmed.substring(1);
    const parts = withoutSlash.split(" ");
    cmd = parts[0].toLowerCase();
    args = parts.slice(1).join(" ");
  }

  const username = "philip";

  function getU64Id(status) {
    if(status === "PLANNED") return 1001;
    if(status === "READY") return 1002;
    if(status === "EXECUTING") return 1003;
    if(status === "VERIFIED") return 1004;
    return 1001;
  }

  if (cmd === "add") {
    if (args.trim() === "") {
      addLog("INFO", "Error: Task name cannot be empty. Usage: /add <task name>");
    } else {
      const name = args;
      const id = nextId++;
      tasks.push({ id, name, status: "PLANNED" });
      
      const trace1 = `[${username}]-[AUDIT]-Entity [Task(${id})] CREATED. [DOMAIN: User Requested -> Task Created] {id: U64(${id}),  name: Text("${name}"),  platform_id: NULL,  status_id: U64(1001),  version: I64(1)}`;
      addLog("INFO", trace1);
      
      addLog("INFO", "SqlLogEntry\nDOMAIN: User Requested -> Task Created\n✔ INSERT INTO task_data (id, name, version, status, platform) VALUES (" + id + ", '" + name + "', 1, 1001, 1)");
    }
  } else if (cmd === "move" || cmd === "mv") {
    const moveParts = args.split(" ");
    if (args.trim() === "" || moveParts.length === 0) {
      addLog("INFO", "Error: Missing arguments. Usage: /mv <id> [planned|ready|executing|verified|next]");
      return;
    }
    const id = parseInt(moveParts[0], 10);
    const target = moveParts.length > 1 ? moveParts[1].toLowerCase() : "";
    const task = tasks.find(t => t.id === id);
    if (!task) {
      addLog("INFO", `WARNING: Task ${id} not found.`);
    } else {
      const currentIdx = statuses.indexOf(task.status);
      let nextStatus = task.status;
      
      if (target === "planned") nextStatus = "PLANNED";
      else if (target === "ready") nextStatus = "READY";
      else if (target === "executing") nextStatus = "EXECUTING";
      else if (target === "verified") nextStatus = "VERIFIED";
      else if (target === "next" || target === "") {
         if (currentIdx < statuses.length - 1) nextStatus = statuses[currentIdx + 1];
      }

      if (nextStatus === task.status) {
        addLog("INFO", `WARNING: Task ${id} is already in its final status and cannot be moved further.`);
      } else {
        const oldStatus = task.status;
        task.status = nextStatus;
        
        const trace1 = `[${username}]-[AUDIT]-Entity [Task] UPDATED. [DOMAIN: Move '${task.name}' ${oldStatus} => ${nextStatus}] {status: U64(${getU64Id(nextStatus)})}`;
        addLog("INFO", trace1);
        
        const trace2 = `[${username}]-[AUDIT]-Entity [TaskExecutionLog] CREATED. [DOMAIN: Move '${task.name}' ${oldStatus} => ${nextStatus} -> Generate execution log for action 'STATUS_CHANGED'] {action: Text("STATUS_CHANGED"),  detail: Text("Task ${id} moved from ${oldStatus} to ${nextStatus}"),  id: U64(999),  task_id: U64(${id}),  version: I64(1)}`;
        addLog("INFO", trace2);
        
        const trace3 = `[${username}]-[INFO]-Business Log: Task ${id} moved from ${oldStatus} to ${nextStatus} [DOMAIN: Move '${task.name}' ${oldStatus} => ${nextStatus} -> Generate execution log for action 'STATUS_CHANGED']`;
        addLog("INFO", trace3);
        
        addLog("INFO", `SqlLogEntry\nDOMAIN: Move '${task.name}' ${oldStatus} => ${nextStatus}\n✔ UPDATE task_data SET status = ${getU64Id(nextStatus)} WHERE id = ${id}`);
      }
    }
  } else if (cmd === "delete" || cmd === "del") {
    if (args.trim() === "") {
      addLog("INFO", "Error: Missing task ID. Usage: /del <id>");
    } else {
      const id = parseInt(args, 10);
      const initialLen = tasks.length;
      const task = tasks.find(t => t.id === id);
      tasks = tasks.filter(t => t.id !== id);
      if (tasks.length < initialLen) {
        addLog("INFO", `[${username}]-[AUDIT]-Entity [Task] DELETED. [DOMAIN: User Requested -> Delete Task ${id}] {id: U64(${id})}`);
        addLog("INFO", `SqlLogEntry\nDOMAIN: User Requested -> Delete Task ${id}\n✔ DELETE FROM task_data WHERE id = ${id}`);
      } else {
        addLog("INFO", `Error: Invalid task ID '${args}'`);
      }
    }
  } else if (cmd === "search" || cmd === "s") {
    if (args.trim() === "") {
      addLog("INFO", "Cleared active search query.");
    } else {
      addLog("INFO", `Searching for tasks by keyword: '${args}'`);
    }
  } else if (cmd === "reload" || cmd === "r") {
    addLog("INFO", "Reloading task data from database...");
  } else if (cmd === "exit" || cmd === "quit" || cmd === "q") {
    addLog("INFO", "Exiting application...");
  } else {
    addLog("INFO", `Unknown command: '/${cmd}'. Type a task name directly or use /r, /mv, /del, /s, /q`);
  }
  
  addLog("INFO", "--------------------------------------------------------------------------------");
  renderTasks();
}

cmdInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    const val = cmdInput.value;
    handleCommand(val);
    cmdInput.value = "";
  }
});

// Focus command line if user types any character (unless they are selecting text and pressing ctrl/cmd)
window.addEventListener("keydown", (e) => {
  if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && document.activeElement !== cmdInput) {
    // If user is just typing normal characters and input isn't focused, focus it
    // But allow copying (Ctrl+C) when text is selected!
    cmdInput.focus();
  }
});

// Init
renderTasks();
cmdInput.focus();
