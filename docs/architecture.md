# Fidel Amharic — System Architecture

Fidel Amharic is a native, system-wide Ethiopic input method architected to provide zero-latency, Unicode-compliant typing in any application across Windows, macOS, and Linux.

## Core Architecture Principles

1. **True System-Wide Integration:** Integrates into native OS input frameworks (Windows TSF, macOS InputMethodKit, Linux IBus/Fcitx5). Never uses simulated keystrokes, clipboard paste hacks, or global keyloggers.
2. **Shared Composition Engine:** 100% deterministic Rust state machine (`fidel-engine`) shared across all operating systems.
3. **Decoupled Settings UI:** Tauri 2 is used exclusively for configuration, visual keyboards, tutorials, and onboarding. System-wide typing functions uninterrupted even when the settings app is completely closed.
4. **Declarative Layout Profiles:** All phonetic sequences, vowel orders, punctuation, and numerals are defined in versioned TOML profiles (`fidel-core`), with clean separation between engine logic and keyboard layouts.

## Component Map

```mermaid
flowchart TD
    subgraph ClientApplications["Desktop Applications"]
        A1["Microsoft Word / LibreOffice"]
        A2["Chrome / Firefox / Edge / Safari"]
        A3["System Text Fields / Terminal"]
    end

    subgraph NativeFrameworks["OS Text Frameworks"]
        W["Windows Text Services Framework (TSF)"]
        M["macOS InputMethodKit (IMK)"]
        L["Linux IBus / Fcitx5"]
    end

    subgraph NativeAdapters["Platform Adapters"]
        WA["Windows TSF In-Proc COM DLL\n(Rust via windows-rs)"]
        MA["macOS Input Method App\n(Swift/ObjC → libfidel_protocol.a)"]
        LA["Linux IBus Engine Daemon\n(Rust via zbus D-Bus)"]
    end

    subgraph CoreEngine["Shared Rust Engine (no GUI)"]
        ABI["fidel-protocol (C ABI)"]
        ENG["fidel-engine (Trie Composer)"]
        COR["fidel-core (Profile Registry & Validator)"]
        DIAG["fidel-diagnostics (Privacy-Safe Metrics)"]
    end

    subgraph CompanionUI["Tauri 2 Companion App (Optional)"]
        T["Settings & Keyboard Visualizer\n(TypeScript + React)"]
    end

    A1 -.-> NativeFrameworks
    A2 -.-> NativeFrameworks
    A3 -.-> NativeFrameworks

    W --> WA
    M --> MA
    L --> LA

    WA --> ABI
    MA --> ABI
    LA --> ENG

    ABI --> ENG
    ENG --> COR
    ENG --> DIAG

    T -.->|"Reads/Writes Config"| COR
```

## Composition Lifecycle

```mermaid
sequenceDiagram
    participant User
    participant App as Target Application
    participant OS as OS Text Framework
    participant Adapter as Native Adapter
    participant Engine as fidel-engine

    User->>OS: Key Press ('h')
    OS->>Adapter: KeyEvent
    Adapter->>Engine: process_key('h')
    Engine->>Engine: Trie lookup: 'h' -> MatchWithMore("ሀ")
    Engine-->>Adapter: SetComposition { display: "ሀ", raw: "h" }
    Adapter->>OS: Update Preedit ("ሀ" with underline)
    OS->>App: Render inline marked text

    User->>OS: Key Press ('a')
    OS->>Adapter: KeyEvent
    Adapter->>Engine: process_key('a')
    Engine->>Engine: Trie lookup: "ha" -> ExactMatch("ሃ")
    Engine-->>Adapter: Commit { text: "ሃ" }
    Adapter->>OS: Commit text "ሃ" & Hide Preedit
    OS->>App: Insert Unicode "ሃ"
```
