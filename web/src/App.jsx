import React, { useState, useEffect, useRef } from 'react';
import AdminPage from './AdminPage';


// Initialize session ID
let initialSessionId = localStorage.getItem("sessionId");
if (!initialSessionId) {
  initialSessionId = "user-" + Math.random().toString(36).substring(2, 9);
  localStorage.setItem("sessionId", initialSessionId);
}

const STATUSES = ["PLANNED", "READY", "EXECUTING", "VERIFIED"];

function getTimestamp() {
  return new Date().toISOString().replace(/\.\d{3}Z$/, 'Z');
}

export default function App() {
  const [sessionId] = useState(initialSessionId);
  const [tasks, setTasks] = useState([]);
  const [logs, setLogs] = useState([
    { id: 'init-1', level: 'INFO', time: getTimestamp(), message: 'System initialized.' },
    { id: 'init-2', level: 'INFO', time: getTimestamp(), message: 'Connected to TeaQL Runtime...' }
  ]);
  const [cmdInput, setCmdInput] = useState('');
  
  const logContentRef = useRef(null);
  const cmdInputRef = useRef(null);

  const addLog = (level, message) => {
    setLogs(prev => [...prev, { id: Math.random().toString(), level, time: getTimestamp(), message }]);
  };

  const fetchTasks = async () => {
    try {
      const res = await fetch("/api/tasks", {
        headers: { "x-session-id": sessionId }
      });
      const data = await res.json();
      setTasks(data.tasks || Object.values(data).flat().filter(t => t && t.id) || []);
      // The API returns { planned_tasks, ready_tasks, ... } based on Rust struct
      // We'll flatten it to a single array for easier rendering based on our current logic.
      if (data.planned_tasks) {
        let allTasks = [];
        allTasks.push(...(data.planned_tasks || []).map(t => ({...t, status: 'PLANNED'})));
        allTasks.push(...(data.ready_tasks || []).map(t => ({...t, status: 'READY'})));
        allTasks.push(...(data.executing_tasks || []).map(t => ({...t, status: 'EXECUTING'})));
        allTasks.push(...(data.verified_tasks || []).map(t => ({...t, status: 'VERIFIED'})));
        setTasks(allTasks);
      }
    } catch (e) {
      addLog("WARN", "Failed to fetch tasks: " + e.message);
    }
  };

  const pollLogs = async () => {
    try {
      const res = await fetch("/api/logs", {
        headers: { "x-session-id": sessionId }
      });
      if (res.ok) {
        const newLogs = await res.json();
        if (newLogs && newLogs.length > 0) {
          setLogs(prev => {
            const next = [...prev];
            newLogs.forEach(line => {
              next.push({ id: Math.random().toString(), level: 'INFO', time: getTimestamp(), message: line });
            });
            return next;
          });
        }
      }
    } catch (e) {
      // silent
    }
  };

  useEffect(() => {
    fetchTasks();
    const interval = setInterval(pollLogs, 1000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (logContentRef.current) {
      logContentRef.current.scrollTop = logContentRef.current.scrollHeight;
    }
  }, [logs]);

  useEffect(() => {
    const handleGlobalKeyDown = (e) => {
      if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && document.activeElement !== cmdInputRef.current) {
        cmdInputRef.current?.focus();
      }
    };
    window.addEventListener("keydown", handleGlobalKeyDown);
    return () => window.removeEventListener("keydown", handleGlobalKeyDown);
  }, []);

  const handleCommand = async (e) => {
    e.preventDefault();
    const cmdStr = cmdInput;
    setCmdInput('');
    const trimmed = cmdStr.trim();
    if (trimmed === "") return;

    let cmd = "add";
    let args = trimmed;

    if (trimmed.startsWith("/")) {
      const withoutSlash = trimmed.substring(1);
      const parts = withoutSlash.split(" ");
      cmd = parts[0].toLowerCase();
      args = parts.slice(1).join(" ");
    }

    if (cmd === "add") {
      if (args.trim() === "") {
        addLog("INFO", "Error: Task name cannot be empty. Usage: /add <task name>");
      } else {
        try {
          const res = await fetch("/api/tasks", {
            method: "POST",
            headers: { "Content-Type": "application/json", "x-session-id": sessionId },
            body: JSON.stringify({ name: args })
          });
          if (res.ok) await fetchTasks();
          else addLog("WARN", "Error adding task: " + await res.text());
        } catch (e) {
          addLog("WARN", "Error adding task: " + e.message);
        }
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
        const currentIdx = STATUSES.indexOf(task.status);
        let nextStatus = task.status;
        
        if (target === "planned") nextStatus = "PLANNED";
        else if (target === "ready") nextStatus = "READY";
        else if (target === "executing") nextStatus = "EXECUTING";
        else if (target === "verified") nextStatus = "VERIFIED";
        else if (target === "next" || target === "") {
           if (currentIdx < STATUSES.length - 1) nextStatus = STATUSES[currentIdx + 1];
        }

        if (nextStatus === task.status) {
          addLog("INFO", `WARNING: Task ${id} is already in its final status.`);
        } else {
          try {
            const res = await fetch(`/api/tasks/${id}/move`, {
              method: "PUT",
              headers: { "Content-Type": "application/json", "x-session-id": sessionId },
              body: JSON.stringify({ status: nextStatus })
            });
            if (res.ok) await fetchTasks();
            else addLog("WARN", "Error moving task: " + await res.text());
          } catch (e) {
            addLog("WARN", "Error moving task: " + e.message);
          }
        }
      }
    } else if (cmd === "delete" || cmd === "del" || cmd === "rm") {
      if (args.trim() === "") {
        addLog("INFO", "Error: Missing task ID. Usage: /rm <id>");
      } else {
        const id = parseInt(args, 10);
        try {
          const res = await fetch(`/api/tasks/${id}`, {
            method: "DELETE",
            headers: { "x-session-id": sessionId }
          });
          if (res.ok) await fetchTasks();
          else addLog("WARN", "Error deleting task: " + await res.text());
        } catch (e) {
          addLog("WARN", "Error deleting task: " + e.message);
        }
      }
    } else if (cmd === "reload" || cmd === "r") {
      addLog("INFO", "Reloading task data from database...");
      await fetchTasks();
    } else {
      addLog("INFO", `Unknown command: '/${cmd}'. Type a task name directly or use /r, /mv, /rm`);
    }
    
    // We don't add the "----" separator anymore, we rely on the clean visual separation.
  };

  if (window.location.pathname === '/admin') {
    return <AdminPage />;
  }

  return (
    <>
      <div className="panel log-panel">
        <div className="panel-header">
          <div className="title">System Log</div>
          <div className="actions">Session: {sessionId}</div>
        </div>
        <div className="panel-content">
          <div className="log-content-area" ref={logContentRef}>
            {logs.map(log => (
              <div key={log.id} className="log-line">
                <span className="log-time">[{log.time}]</span> - <span className={`log-level-${log.level}`}>[{log.level}]</span> - {log.message}
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="columns-container">
        {STATUSES.map(status => {
          const colTasks = tasks.filter(t => t.status === status);
          return (
            <div key={status} className={`panel kanban-col status-${status}`}>
              <div className="panel-header">
                <div className="title flex items-center">
                  <span className="status-indicator"></span>
                  {status}
                </div>
                <div className="count">{colTasks.length}</div>
              </div>
              <div className="panel-content">
                <div className="kanban-content">
                  {colTasks.map(task => (
                    <div key={task.id} className="task-item">
                      <div className="task-id">ID: {task.id}</div>
                      <div className="task-name">{task.name}</div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      <div className="panel cmd-line-panel">
        <div className="panel-content">
          <form onSubmit={handleCommand} className="cmd-input-wrapper">
            <span className="cmd-prompt">&gt;</span>
            <input 
              type="text" 
              id="cmd-input" 
              ref={cmdInputRef}
              value={cmdInput}
              onChange={e => setCmdInput(e.target.value)}
              autoComplete="off" 
              spellCheck="false" 
              placeholder="Type a task name to add, or /mv <id> <status>, /rm <id>"
              autoFocus
            />
          </form>
        </div>
      </div>
    </>
  );
}
