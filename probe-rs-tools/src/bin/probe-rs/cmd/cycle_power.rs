use std::time::Duration;

use crate::{rpc::client::RpcClient, util::cli};

#[derive(clap::Parser)]
pub struct Cmd {
    /// The probe to power cycle, specified as VID:PID or VID:PID:Serial.
    #[arg(long, env = "PROBE_RS_PROBE", help_heading = "PROBE CONFIGURATION")]
    probe: Option<probe_rs::probe::DebugProbeSelector>,

    /// How long to keep port power off, in milliseconds.
    #[arg(long, default_value_t = 1000)]
    off_duration_ms: u64,

    /// How long to wait for the probe to re-enumerate after power-on, in milliseconds.
    #[arg(long, default_value_t = 10000)]
    reenumerate_timeout_ms: u64,
}

impl Cmd {
    pub async fn run(self, client: RpcClient) -> anyhow::Result<()> {
        let probe = cli::select_probe(&client, self.probe.map(Into::into)).await?;
        client
            .cycle_power(
                probe.selector(),
                Duration::from_millis(self.off_duration_ms),
                Duration::from_millis(self.reenumerate_timeout_ms),
            )
            .await
    }
}
