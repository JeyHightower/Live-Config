# Live-Config-Engine

A high-performance, real-time distributed Feature Flagging & Remote Configuration Infrastructure engineered in Rust. This system enables instant, thread-safe configuration updates across distributed client applications via persistent WebSockets, backed by a persistent Write-Ahead Log (WAL) storage engine.

Built entirely using an intentional architectural separation of concerns: Rule (Data), Engine (Core Logic), and System (I/O & Networking).

---

## 🏗️ System Architecture

The project is cleanly decoupled into three foundational layers to maximize horizontal scalability, memory safety, and thread concurrency:

```
               ┌─────────────────────────────────────────┐
               │          Administrative UI              │
               │        (HTML5 / JS / WebSockets)        │
               └────────────────────┬────────────────────┘
                                    │ HTTP POST / WS
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│ SYSTEM LAYER (Axum Web Server)                                         │
│                                                                        │
│  ┌───────────────────────┐            ┌─────────────────────────────┐  │
│  │  REST API Endpoints   │            │    WebSocket Broadcast      │  │
│  │     (/api/flags)      │            │       Server (/ws)          │  │
│  └───────────┬───────────┘            └──────────────▲──────────────┘  │
└──────────────┼───────────────────────────────────────┼─────────────────┘
               │ Reads / Writes                        │ Broadcasts
               ▼                                       │ Updates
┌──────────────────────────────────────────────────────┴─────────────────┐
│ ENGINE LAYER (Storage Core)                                            │
│                                                                        │
│  ┌───────────────────────────┐            ┌─────────────────────────┐  │
│  │   Write-Ahead Log (WAL)   │            │   In-Memory Cache       │  │
│  │        (flags.log)        │◄──────────►│ (Arc<RwLock<HashMap>>)  │  │
│  └───────────────────────────┘            └─────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────┘
                                                       │
                                                       │ WebSockets
                                                       ▼
                                        ┌─────────────────────────┐
                                        │    CLIENT SDK LAYER     │
                                        │  (Background Cache Sync)│
                                        └─────────────────────────┘
```

### 1. The Rule Layer (Data Models)

Defines the strict contracts and data schemas across the network. The system evaluates a rich `FeatureFlag` primitive schema supporting targeting variants and structural concurrency protection:

- **Name:** Unique identifier string for conditional application routing.
- **Is Enabled:** Explicit activation boolean.
- **Value:** Variable variant mapping string (enabling dynamic properties like theme variants or multi-variate test weights).
- **Version:** Monotonically increasing schema integer ensuring sequence integrity across clients.

### 2. The Engine Layer (Persistence & Concurrency)

Responsible for memory management and persistence.

- **In-Memory Cache:** Utilizing `Arc<RwLock<HashMap<String, FeatureFlag>>>` to guarantee lock-free reads across dozens of concurrent API and WebSocket threads, isolating mutations to transient write-locks.
- **Write-Ahead Log:** An append-only persistence log file that ensures durable state preservation across server restarts, cleanly hydrating RAM from disk upon system boot.

### 3. The System Layer (Networking & UI)

Handles the external interface, network routing, and streaming distribution:

- **Axum Server & Tower-HTTP:** Powers standard asynchronous REST endpoints (GET / POST) alongside an optimized static asset delivery channel.
- **Real-time Synchronization Tower:** Upgrades incoming client pipes into full-duplex WebSockets, broadcasting structured config packets down to listening nodes instantly when structural state shifts.
- **Control Room Dashboard:** A lightweight administrative dashboard rendering structural toggles and consuming backend WebSockets to reflect status registers fluidly without forcing manual page updates.

---

## 📦 Project Workspace Layout

```
Live-Config-Engine/
├── backend/
│   ├── src/
│   │   └── main.rs          # Axum server, routing configuration, & handlers
│   ├── static/
│   │   └── index.html       # Control Room Dashboard (Form, UI Table, & WS Client)
│   └── Cargo.toml           # Engine dependencies (axum, tokio, tower-http, serde)
├── client_sdk/
│   ├── src/
│   │   ├── lib.rs           # Thread-safe Client SDK core & WebSocket sync task
│   │   └── main.rs          # Integration loop testing binary execution
│   └── Cargo.toml           # SDK dependencies (tokio, reqwest, tokio-tungstenite)
└── target/                  # Optimized compilation artifacts
```

---

## ⚡ Getting Started

### Prerequisites

- Rust toolchain (Stable 2021 Edition or later)
- Standard internet browser

### 1. Spin Up the Backend Server & Dashboard

Navigate into the backend directory, ensure dependencies are fetched, and launch the asynchronous server runtime:

```bash
cd backend
cargo add tower-http --features "fs"  # Ensure asset driver feature flags are present
cargo run
```

The server will bind immediately to your local network context:
👉 **Control Room Dashboard Live at:** `http://127.0.0.1:3000`

### 2. Boot the Client SDK Simulation

Open a secondary terminal window, navigate into the SDK workspace, and run the client execution program loop:

```bash
cd client_sdk
cargo run --bin client_sdk
```

The client initializes an initial HTTP baseline handshake, updates its internal cache, and spawns an isolated background thread that sleeps while holding an active WebSocket connection open to the server.

---

## 🕹️ Verifying Real-Time Operation

1. Open your web browser and load `http://127.0.0.1:3000`.
2. Locate the **Update Feature Flag Form** on the left and enter the tracking details:
   - **Flag Name:** `premium_features`
   - **State Configuration:** Enabled (`true`)