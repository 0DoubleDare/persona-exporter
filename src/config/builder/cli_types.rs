use argh::FromArgs;
use compact_str::CompactString;

#[derive(FromArgs, Debug)]
#[argh(description = "Persona Exporter CLI")]
/// A Metrics exporter (JSON / line_protocol)
pub struct MainCliArguments {
    #[argh(option, short = 'v', default = "0")]
    /// log levels
    /// 1 - Info
    /// 2 - Debug
    /// 3 - Trace
    pub verbose: u8,
    // #[argh(switch, short = 'v')]
    // enable debugging level info
    // pub verbose_1: bool,
    // #[argh(switch, short = 'd')]
    // enable debugging level debug
    // pub verbose_2: bool,
    #[argh(option, short = 'c')]
    /// set custom config path
    pub config_path: Option<CompactString>,
    #[argh(switch, short = 't')]
    /// validate configuration
    pub config_test: bool,
}
