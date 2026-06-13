use clap::{Parser, Subcommand};
use machine::{commands, ProcessCommands};

#[derive(Parser)]
#[command(name = "machine")]
#[command(about = "Your Linux system can talk. Machine translates.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Increase logging verbosity
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Show high-level system health summary
    Status,
    /// Diagnose likely system problems
    Wtf,
    /// Explain what a process is doing and why it exists
    Why {
        /// Process name or PID
        target: String,
    },
    /// Show high-level system health snapshot
    Pulse,
    /// Explain a specific process or PID
    Explain {
        /// Process name or PID
        target: String,
    },
    /// Show process ancestry tree
    Trace {
        /// Process name or PID
        target: String,
    },
    /// Live monitoring mode
    Watch,
    /// Comprehensive system investigation
    Investigate,
    /// Comprehensive system health check
    Doctor,
    /// Analyze startup performance
    Boot,
    /// Show network connections
    Net,
    /// Find reclaimable storage
    Clean,
    /// Show system event timeline
    History,
    /// Compare system state changes
    Compare {
        /// Time period to compare against (e.g., yesterday, 7d)
        period: Option<String>,
    },
    /// Dedicated RAM analysis
    Memory,
    /// Dedicated storage analysis
    Disk,
    /// Process-focused analysis
    Process {
        #[command(subcommand)]
        sub: Option<ProcessCommands>,
    },
    /// Create a system snapshot
    Snapshot,
    /// Compare two snapshots
    Diff {
        snapshot_a: String,
        snapshot_b: String,
    },
    /// Forecast resource exhaustion
    Forecast {
        /// Resource to forecast (cpu, memory, disk)
        resource: Option<String>,
    },
    /// Detect unusual system activity
    Anomalies,
    /// Analyze system services
    Services {
        /// Only show slow or failed services
        #[arg(long)]
        failed: bool,
    },
    /// Basic security audit
    Security,
    /// Show listening ports
    Ports,
    /// Analyze network latency
    Latency,
    /// Laptop battery diagnostics
    Battery,
    /// CPU thermal analysis
    Thermal,
    /// Generate comprehensive report
    Report {
        /// Save report to file
        #[arg(short, long)]
        output: Option<String>,
    },
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => commands::status::run(cli.json, cli.verbose).await?,
        Commands::Wtf => commands::wtf::run(cli.json, cli.verbose).await?,
        Commands::Why { target } => commands::why::run(target, cli.json).await?,
        Commands::Pulse => commands::pulse::run().await?,
        Commands::Explain { target } => commands::explain::run(target, cli.json, cli.verbose).await?,
        Commands::Trace { target } => commands::trace::run(target, cli.json, cli.verbose).await?,
        Commands::Watch => commands::watch::run().await?,
        Commands::Investigate => commands::investigate::run(cli.verbose, cli.json).await?,
        Commands::Doctor => commands::doctor::run().await?,
        Commands::Boot => commands::boot::run().await?,
        Commands::Net => commands::net::run().await?,
        Commands::Clean => commands::clean::run().await?,
        Commands::History => commands::history::run().await?,
        Commands::Compare { period } => commands::compare::run(period).await?,
        Commands::Memory => commands::memory::run().await?,
        Commands::Disk => commands::disk::run().await?,
        Commands::Process { sub } => commands::process::run(sub).await?,
        Commands::Snapshot => commands::snapshot::run().await?,
        Commands::Diff { snapshot_a, snapshot_b } => commands::diff::run(snapshot_a, snapshot_b).await?,
        Commands::Forecast { resource } => commands::forecast::run(resource).await?,
        Commands::Anomalies => commands::anomalies::run().await?,
        Commands::Services { failed: _ } => commands::services::run().await?,
        Commands::Security => commands::security::run().await?,
        Commands::Ports => commands::ports::run().await?,
        Commands::Latency => commands::latency::run().await?,
        Commands::Battery => commands::battery::run().await?,
        Commands::Thermal => commands::thermal::run().await?,
        Commands::Report { output } => commands::report::run(output).await?,
    }

    Ok(())
}
