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
  const args = cmdStr.trim().split(" ");
  const cmd = args[0].toLowerCase();
  
  if (cmd === "add") {
    const name = args.slice(1).join(" ");
    if (name) {
      tasks.push({ id: nextId++, name, status: "PLANNED" });
      addLog("INFO", `Created task: ${name}`);
    }
  } else if (cmd === "move") {
    const id = parseInt(args[1], 10);
    const task = tasks.find(t => t.id === id);
    if (task) {
      const idx = statuses.indexOf(task.status);
      if (idx < statuses.length - 1) {
        task.status = statuses[idx + 1];
        addLog("INFO", `Moved task ${id} to ${task.status}`);
      } else {
        addLog("WARN", `Task ${id} is already in final state.`);
      }
    } else {
      addLog("WARN", `Task ${id} not found.`);
    }
  } else if (cmd === "rm") {
    const id = parseInt(args[1], 10);
    const initialLen = tasks.length;
    tasks = tasks.filter(t => t.id !== id);
    if (tasks.length < initialLen) {
      addLog("INFO", `Deleted task ${id}`);
    } else {
      addLog("WARN", `Task ${id} not found.`);
    }
  } else if (cmd === "q") {
    addLog("INFO", "Quit command received. (Browser mode: ignoring)");
  } else if (cmdStr.trim() !== "") {
    addLog("WARN", `Unknown command: ${cmd}`);
  }
  
  renderTasks();
}

cmdInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    const val = cmdInput.value;
    handleCommand(val);
    cmdInput.value = "";
  }
});

// Focus command line on any click
document.body.addEventListener("click", () => {
  cmdInput.focus();
});

// Keep focus
cmdInput.addEventListener("blur", () => {
  setTimeout(() => cmdInput.focus(), 10);
});

// Init
renderTasks();
cmdInput.focus();
