import React, { useState, useEffect } from 'react';

export default function AdminPage() {
  const [tenants, setTenants] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/admin/tenants')
      .then(res => res.json())
      .then(data => {
        setTenants(data);
        setLoading(false);
      })
      .catch(err => {
        console.error("Failed to load tenants", err);
        setLoading(false);
      });
  }, []);

  return (
    <div style={{ padding: '2rem', maxWidth: '1200px', margin: '0 auto', color: '#e2e8f0' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
        <h1 style={{ fontSize: '2rem', fontWeight: 'bold' }}>Admin Dashboard - Tenants</h1>
        <a href="/" style={{ color: '#38bdf8', textDecoration: 'none', background: '#0f172a', padding: '0.5rem 1rem', borderRadius: '4px', border: '1px solid #1e293b' }}>
          &larr; Back to Kanban
        </a>
      </div>

      {loading ? (
        <div>Loading...</div>
      ) : (
        <div style={{ display: 'grid', gap: '1.5rem', gridTemplateColumns: 'repeat(auto-fill, minmax(320px, 1fr))' }}>
          {tenants.map(t => (
            <div key={t.id} style={{ background: '#1e293b', padding: '1.5rem', borderRadius: '8px', border: '1px solid #334155', boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)' }}>
              <div style={{ borderBottom: '1px solid #334155', paddingBottom: '0.75rem', marginBottom: '0.75rem' }}>
                <h3 style={{ margin: '0 0 0.5rem 0', color: '#f8fafc', fontSize: '1.25rem' }}>{t.name}</h3>
                <div style={{ fontSize: '0.875rem', color: '#94a3b8' }}>Tenant ID: {t.id}</div>
              </div>
              
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem', background: '#0f172a', padding: '0.5rem', borderRadius: '4px' }}>
                <span style={{ color: '#94a3b8', fontSize: '0.875rem' }}>Total Tasks</span>
                <span style={{ fontSize: '1.25rem', fontWeight: 'bold', color: '#38bdf8' }}>{t.task_count}</span>
              </div>

              <div>
                <h4 style={{ fontSize: '0.875rem', color: '#cbd5e1', textTransform: 'uppercase', letterSpacing: '0.05em', marginBottom: '0.5rem', marginTop: 0 }}>Recent Tasks (Top 3)</h4>
                {t.recent_tasks.length === 0 ? (
                  <div style={{ fontSize: '0.875rem', color: '#64748b', fontStyle: 'italic' }}>No tasks found</div>
                ) : (
                  <ul style={{ listStyle: 'none', padding: 0, margin: 0, display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                    {t.recent_tasks.map(task => (
                      <li key={task.id} style={{ background: '#0f172a', padding: '0.5rem', borderRadius: '4px', fontSize: '0.875rem', borderLeft: '3px solid #38bdf8' }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '0.25rem' }}>
                          <strong style={{ color: '#e2e8f0' }}>ID: {task.id}</strong>
                          <span style={{ fontSize: '0.75rem', padding: '0.1rem 0.4rem', background: '#334155', borderRadius: '9999px', color: '#cbd5e1' }}>{task.status}</span>
                        </div>
                        <div style={{ color: '#94a3b8', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                          {task.name}
                        </div>
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
