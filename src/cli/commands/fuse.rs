#[derive(clap::Parser)]
pub enum FuseCommand {
    #[command(
        about = "Fuse two personas together"
    )]
    Calculate {
        #[arg(long = "first")]
        first: String,
        #[arg(long = "second")]
        second: String
    },
    #[command(about = "Show possible fusions with this persona")]
    From {
        #[arg(long = "name")]
        name: String
    },
    #[command(about = "Show possible fusions resulting in this persona")]
    To {
        #[arg(long = "name")]
        name: String
    }
}