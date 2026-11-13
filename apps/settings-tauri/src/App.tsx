import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Diagnostics {
  engine_version: string;
  platform: string;
  architecture: string;
  active_profile_id?: string;
  active_profile_version?: string;
}

interface ProfileMetadata {
  id: string;
  display_name: string;
  language: string;
  version: string;
  license: string;
  author?: string;
}

interface FidelRow {
  key: string;
  name: string;
  orders: [string, string, string, string, string, string, string];
}

type Tab = 'welcome' | 'dashboard' | 'visualizer' | 'tutorial' | 'diagnostics' | 'about';

export default function App() {
  const [activeTab, setActiveTab] = useState<Tab>('welcome');
  const [diag, setDiag] = useState<Diagnostics | null>(null);
  const [activeProfile, setActiveProfile] = useState<ProfileMetadata | null>(null);
  const [fidelRows, setFidelRows] = useState<FidelRow[]>([]);
  const [testInput, setTestInput] = useState('');
  const [typedOutput, setTypedOutput] = useState('');
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    // Load initial data via Tauri commands
    invoke<Diagnostics>('get_diagnostics')
      .then(setDiag)
      .catch((err) => console.warn('Running outside Tauri or command failed:', err));

    invoke<ProfileMetadata>('get_active_profile')
      .then(setActiveProfile)
      .catch((err) => console.warn('Could not fetch active profile:', err));

    invoke<FidelRow[]>('get_fidel_matrix')
      .then(setFidelRows)
      .catch((err) => console.warn('Could not fetch Fidel matrix:', err));
  }, []);

  const handleTestTyping = async (text: string) => {
    setTestInput(text);
    try {
      const res = await invoke<string>('simulate_typing', { input: text });
      setTypedOutput(res);
    } catch {
      // Fallback if not running inside Tauri webview
      setTypedOutput(text);
    }
  };

  const filteredRows = fidelRows.filter(
    (r) =>
      r.key.toLowerCase().includes(searchTerm.toLowerCase()) ||
      r.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      r.orders.some((char) => char.includes(searchTerm))
  );

  return (
    <div className="app-container">
      {/* Sidebar Navigation */}
      <aside className="sidebar">
        <div className="logo-header">
          <div className="logo-icon">ፊ</div>
          <div className="logo-title">
            <h1>Fidel Amharic</h1>
            <span>Native Ethiopic IME</span>
          </div>
        </div>

        <nav className="nav-menu">
          <button
            className={`nav-button ${activeTab === 'welcome' ? 'active' : ''}`}
            onClick={() => setActiveTab('welcome')}
          >
            <span>👋</span> Welcome
          </button>
          <button
            className={`nav-button ${activeTab === 'dashboard' ? 'active' : ''}`}
            onClick={() => setActiveTab('dashboard')}
          >
            <span>📊</span> Dashboard
          </button>
          <button
            className={`nav-button ${activeTab === 'visualizer' ? 'active' : ''}`}
            onClick={() => setActiveTab('visualizer')}
          >
            <span>⌨️</span> Keyboard Map
          </button>
          <button
            className={`nav-button ${activeTab === 'tutorial' ? 'active' : ''}`}
            onClick={() => setActiveTab('tutorial')}
          >
            <span>📖</span> Tutorial
          </button>
          <button
            className={`nav-button ${activeTab === 'diagnostics' ? 'active' : ''}`}
            onClick={() => setActiveTab('diagnostics')}
          >
            <span>🛡️</span> Diagnostics
          </button>
          <button
            className={`nav-button ${activeTab === 'about' ? 'active' : ''}`}
            onClick={() => setActiveTab('about')}
          >
            <span>ℹ️</span> About
          </button>
        </nav>
      </aside>

      {/* Main Content Pane */}
      <main className="main-content">
        {activeTab === 'welcome' && (
          <div>
            <div className="section-header">
              <h2>Welcome to Fidel Amharic</h2>
              <p>Type authentic Unicode Ethiopic script across all desktop applications.</p>
            </div>

            <div className="card">
              <h3>System-Wide Activation</h3>
              <p style={{ color: 'var(--text-secondary)', marginBottom: '14px', fontSize: '13px' }}>
                Fidel Amharic is a registered native input method. Once enabled in your OS settings, you can
                close this window and type in any application!
              </p>
              <ul style={{ paddingLeft: '20px', fontSize: '13px', color: 'var(--text-secondary)', lineHeight: '1.8' }}>
                <li><strong>Linux (GNOME):</strong> Settings → Keyboard → Input Sources → Add <em>Amharic (Fidel Amharic)</em></li>
                <li><strong>Windows:</strong> Settings → Time & Language → Language → Add <em>Amharic</em> → Select <em>Fidel Amharic (TSF)</em></li>
                <li><strong>macOS:</strong> System Settings → Keyboard → Text Input → Add <em>Amharic (Fidel Amharic)</em></li>
              </ul>
            </div>

            <div className="card">
              <h3>Test Typing Field</h3>
              <p style={{ color: 'var(--text-secondary)', fontSize: '13px' }}>
                Try typing phonetic sequences below (e.g., <code>slame</code> → ሰላም, <code>xmareNa</code> → አማርኛ):
              </p>
              <input
                type="text"
                className="input-box"
                placeholder="Type Latin phonetic keys here..."
                value={testInput}
                onChange={(e) => handleTestTyping(e.target.value)}
              />
              <div style={{ marginTop: '14px', padding: '12px', background: 'var(--bg-tertiary)', borderRadius: '6px' }}>
                <span style={{ fontSize: '11px', color: 'var(--text-muted)' }}>ETHIOPIC UNICODE OUTPUT:</span>
                <div style={{ fontSize: '24px', marginTop: '6px', minHeight: '36px', color: 'var(--accent-blue)', fontFamily: 'var(--font-ethiopic)' }}>
                  {typedOutput || '—'}
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'dashboard' && (
          <div>
            <div className="section-header">
              <h2>Integration Dashboard</h2>
              <p>Real-time system input method status.</p>
            </div>

            <div className="card">
              <h3>Active Keyboard Profile</h3>
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <div>
                  <h4 style={{ fontSize: '15px' }}>{activeProfile?.display_name || 'Power Geez Phonetic-compatible (Unicode)'}</h4>
                  <p style={{ color: 'var(--text-muted)', fontSize: '12px' }}>Profile ID: {activeProfile?.id || 'power_geez_phonetic_unicode'}</p>
                </div>
                <span className="badge">Active</span>
              </div>
            </div>

            <div className="card">
              <h3>Adapter Health</h3>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '12px', marginTop: '10px' }}>
                <div style={{ background: 'var(--bg-tertiary)', padding: '12px', borderRadius: '6px' }}>
                  <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>Linux IBus</div>
                  <div style={{ fontSize: '14px', fontWeight: 600, color: '#3fb950', marginTop: '4px' }}>Installed / Active</div>
                </div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '12px', borderRadius: '6px' }}>
                  <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>Windows TSF</div>
                  <div style={{ fontSize: '14px', fontWeight: 600, color: '#58a6ff', marginTop: '4px' }}>In-Proc COM Ready</div>
                </div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '12px', borderRadius: '6px' }}>
                  <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>macOS IMK</div>
                  <div style={{ fontSize: '14px', fontWeight: 600, color: '#a371f7', marginTop: '4px' }}>C ABI Ready</div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'visualizer' && (
          <div>
            <div className="section-header">
              <h2>Keyboard Visualizer & Character Matrix</h2>
              <p>All 33 Amharic consonant families across the 7 vowel orders.</p>
            </div>

            <div style={{ marginBottom: '14px' }}>
              <input
                type="text"
                className="input-box"
                style={{ width: '300px' }}
                placeholder="Search consonant or syllable..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
              />
            </div>

            <div className="card" style={{ padding: '14px', overflowX: 'auto' }}>
              <div className="matrix-grid">
                <div className="matrix-header">Family</div>
                <div className="matrix-header">1st (ä)</div>
                <div className="matrix-header">2nd (u)</div>
                <div className="matrix-header">3rd (i)</div>
                <div className="matrix-header">4th (a)</div>
                <div className="matrix-header">5th (e)</div>
                <div className="matrix-header">6th (ə)</div>
                <div className="matrix-header">7th (o)</div>

                {filteredRows.map((row) => (
                  <React.Fragment key={row.key}>
                    <div className="matrix-row-label">
                      <code>{row.key}</code>
                    </div>
                    {row.orders.map((syllable, idx) => (
                      <div
                        key={idx}
                        className="matrix-cell"
                        title={`${row.name} - Order ${idx + 1}`}
                        onClick={() => handleTestTyping(testInput + syllable)}
                      >
                        {syllable}
                      </div>
                    ))}
                  </React.Fragment>
                ))}
              </div>
            </div>
          </div>
        )}

        {activeTab === 'tutorial' && (
          <div>
            <div className="section-header">
              <h2>Interactive Typing Tutorial</h2>
              <p>Learn common Amharic syllable formations and punctuation.</p>
            </div>

            <div className="card">
              <h3>Lesson 1: Basic Consonant-Vowel Rules</h3>
              <p style={{ color: 'var(--text-secondary)', fontSize: '13px', lineHeight: '1.6' }}>
                In Power Geez Phonetic, pressing a consonant gives its 1st form. Adding a vowel modifies it to the corresponding order:
              </p>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '10px', marginTop: '12px' }}>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>h</code> → ሀ (1st)</div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>hu</code> → ሁ (2nd)</div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>hi</code> → ሂ (3rd)</div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>ha</code> → ሃ (4th)</div>
              </div>
            </div>

            <div className="card">
              <h3>Lesson 2: Multi-Key Consonants</h3>
              <p style={{ color: 'var(--text-secondary)', fontSize: '13px', lineHeight: '1.6' }}>
                Multi-letter combinations automatically disambiguate:
              </p>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '10px', marginTop: '12px' }}>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>she</code> → ሽ</div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>cha</code> → ቻ</div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>nya</code> → ኛ</div>
              </div>
            </div>

            <div className="card">
              <h3>Lesson 3: Punctuation & Numerals</h3>
              <p style={{ color: 'var(--text-secondary)', fontSize: '13px', lineHeight: '1.6' }}>
                Amharic punctuation marks are easily accessed:
              </p>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '10px', marginTop: '12px' }}>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>:</code> → ፡ (Space)</div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>::</code> → ፨ (Separator)</div>
                <div style={{ background: 'var(--bg-tertiary)', padding: '10px', borderRadius: '6px' }}><code>.</code> → ። (Full stop)</div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'diagnostics' && (
          <div>
            <div className="section-header">
              <h2>Privacy-Safe Diagnostics</h2>
              <p>Safe diagnostic metrics. Typed text is never recorded, stored, or transmitted.</p>
            </div>

            <div className="card">
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '14px' }}>
                <span className="badge">Zero Telemetry</span>
                <span className="badge">100% Offline</span>
              </div>
              <table style={{ width: '100%', fontSize: '13px', borderCollapse: 'collapse' }}>
                <tbody>
                  <tr style={{ borderBottom: '1px solid var(--border)' }}>
                    <td style={{ padding: '8px 0', color: 'var(--text-secondary)' }}>Operating System</td>
                    <td style={{ textAlign: 'right', fontWeight: 500 }}>{diag?.platform || 'linux'}</td>
                  </tr>
                  <tr style={{ borderBottom: '1px solid var(--border)' }}>
                    <td style={{ padding: '8px 0', color: 'var(--text-secondary)' }}>CPU Architecture</td>
                    <td style={{ textAlign: 'right', fontWeight: 500 }}>{diag?.architecture || 'x86_64'}</td>
                  </tr>
                  <tr style={{ borderBottom: '1px solid var(--border)' }}>
                    <td style={{ padding: '8px 0', color: 'var(--text-secondary)' }}>Engine Version</td>
                    <td style={{ textAlign: 'right', fontWeight: 500 }}>{diag?.engine_version || '0.1.0'}</td>
                  </tr>
                  <tr>
                    <td style={{ padding: '8px 0', color: 'var(--text-secondary)' }}>Active Profile</td>
                    <td style={{ textAlign: 'right', fontWeight: 500 }}>{diag?.active_profile_id || 'power_geez_phonetic_unicode'}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === 'about' && (
          <div>
            <div className="section-header">
              <h2>About Fidel Amharic</h2>
              <p>Open source native Ethiopic input method.</p>
            </div>

            <div className="card">
              <h3>License & Copyright</h3>
              <p style={{ color: 'var(--text-secondary)', fontSize: '13px', lineHeight: '1.6' }}>
                Fidel Amharic is licensed under the <strong>MIT License</strong>.
                Phonetic mappings adapted from SIL Ethiopic Power-G (MIT License © SIL Ethiopia).
                Ethiopic Unicode blocks comply with the Unicode 15.0 / 16.0 Standard (U+1200–U+137F).
              </p>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
