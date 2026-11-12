# Privacy and Security Policy — Fidel Amharic

An Input Method Editor (IME) receives keyboard input across the operating system. Privacy is a foundational, non-negotiable architectural requirement of Fidel Amharic.

## Core Privacy Principles

1. **Zero Keystroke Recording:** Fidel Amharic never logs, records, or caches typed text or keystrokes to disk or persistent memory.
2. **Zero Network Transmission:** The core composition engine and native input method adapters contain no networking code, telemetry libraries, or remote analytics. All text processing is strictly local and runs 100% offline.
3. **Password & Secure Field Protection:** In password or secure text fields, the native adapter respects OS privacy modes and either automatically bypasses composition or switches to direct password handling as dictated by the platform text services framework.
4. **Transparent Diagnostics:** The diagnostic reporting module (`fidel-diagnostics`) reports only static system metadata:
   - Operating system platform and architecture
   - Engine and profile version numbers
   - It strictly excludes all buffers, text snippets, and document contents.
5. **Open Source & Auditable:** All engine and adapter code is open source and verifiable by the community.
