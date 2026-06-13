# Command Reference

Detailed documentation for all Available Machine commands.

## System Health

### status
High-level summary of system resources and health.
- **Flags**: `--json`, `--verbose`
- **Example**: `machine status --verbose`

### wtf
Automatic diagnosis of likely system problems.
- **Flags**: `--json`, `--verbose`
- **Example**: `machine wtf`

### doctor
Heuristic-based health scoring and repair recommendations.
- **Example**: `machine doctor`

## Process Analysis

### explain
Detailed behavioral analysis of a specific process.
- **Arguments**: `<target>` (PID or process name)
- **Example**: `machine explain firefox`

### trace
Visualizes the ancestry and ownership tree of a process.
- **Arguments**: `<target>` (PID or process name)
- **Example**: `machine trace 1234`

## Historical & Snapshots

### snapshot
Saves the current system state to the local SQLite database.
- **Example**: `machine snapshot`

### history
Lists previously recorded snapshots.
- **Example**: `machine history`

### diff
Compares two system snapshots to identify changes.
- **Arguments**: `<snapshot_a> <snapshot_b>`
- **Example**: `machine diff snap_20240101_120000 snap_20240101_130000`

### compare
Compare current state against a historical period (e.g., yesterday, 7d).
- **Example**: `machine compare yesterday`

## Diagnostics & Analysis

### services
Analyze systemd units for failures or recent changes.
- **Example**: `machine services`

### security
Perform a read-only audit of listeners, permissions, and listeners.
- **Example**: `machine security`

### ports
Show all listening network sockets and their owning processes.
- **Example**: `machine ports`

### latency
Measure DNS and network responsiveness metrics.
- **Example**: `machine latency`

## Analysis & Trends

### forecast
Predictive resource usage based on historical snapshots.
- **Example**: `machine forecast disk`

### anomalies
Detect unusual spikes or patterns in system behavior.
- **Example**: `machine anomalies`

## Reporting

### report
Generate a comprehensive system diagnostic report.
- **Flags**: `--output <file>`
- **Example**: `machine report --output report.txt`
