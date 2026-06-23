# ⬡ ProcWatch — Ratatui Process Monitor

A beautiful terminal process monitor built with Ratatui, showing real-time RAM usage, CPU usage, and process states.

## Features

- **Live process table** — PID, name, RAM (MB), CPU%, and status with visual bars
- **Color-coded health** — green → teal → yellow → red based on load
- **Process state badges** — RUN, SLP, IDL, STP, ZMB (zombie), DED
- **System gauges** — global CPU and RAM bars with GB/% display
- **History view** — 60-second bar charts for CPU and RAM (Tab to switch)
- **Sorting** — by name, RAM, CPU, or PID (press key again to reverse)
- **Live filter** — type `/` then a process name to narrow results
- **Auto-refresh** — updates every 1.5 seconds automatically

## Controls

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `↑` / `↓` / `j` / `k` | Navigate |
| `PgUp` / `PgDn` | Jump 10 rows |
| `Tab` | Toggle Processes ↔ Overview |
| `/` | Enter filter mode |
| `x` | Clear filter |
| `r` | Force refresh |
| `1` | Sort by name |
| `2` | Sort by RAM |
| `3` | Sort by CPU |
| `4` | Sort by PID |

> Pressing a sort key again reverses the sort order (▲/▼ indicator in header).

## Quick Start

```bash
# Clone or copy the project, then:
cd process_monitor
cargo run --release
```

## Dependencies

```toml
ratatui  = "0.29"    # Terminal UI framework
crossterm = "0.28"   # Cross-platform terminal control
sysinfo  = "0.32"    # System/process info
```

## Screenshots

```
┌ ⬡ PROCWATCH  247 processes ──────────────────┐ ┌──────────────────────────┐
│                                               │ │  Processes │ Overview    │
└───────────────────────────────────────────────┘ └──────────────────────────┘
┌ CPU  12.3% ──────────────┐ ┌ RAM  5.1/16.0 GB (32%) ──┐
│ ████░░░░░░░░░░░░░░░░░░░░ │ │ ████████░░░░░░░░░░░░░░░░ │
└──────────────────────────┘ └──────────────────────────┘
┌ Processes (247/247 shown) ─────────────────────────────────────────────────┐
│ PID    NAME               RAM (MB)  RAM BAR       CPU%  CPU BAR   STATE   │
│▶ 1842  chrome            1024.3    ████████████  12.3%  ██████░░  ● RUN   │
│  1123  python              512.1   ██████░░░░░░   8.1%  ████░░░░  ● RUN   │
│  ...                                                                        │
└────────────────────────────────────────────────────────────────────────────┘
 [q] quit  [↑↓] navigate  [Tab] switch view  [/] filter  [1-4] sort
```