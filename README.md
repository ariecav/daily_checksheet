# Daily Checksheet

Full-stack Rust evaluation/checksheet application. See `CODING_STANDARDS.md` for architecture and coding conventions.

## Setup

```bash
git clone https://github.com/ariecav/daily_checksheet.git
cd daily_checksheet
./setup.sh
```

## Database

`daily_checksheet.db` is **not tracked in git** and must be set up separately on each machine/environment (phone, WSL, Windows, etc). This repo only syncs code — database file and its data stay local to each device and are never shared via git push/pull.

If no database exists yet, the application creates one on first run (see `src/backend/db.rs`). If the connection target (IP/port, remote SQL Server, etc) differs between environments, configure it locally per machine instead of hardcoding it in committed code.
