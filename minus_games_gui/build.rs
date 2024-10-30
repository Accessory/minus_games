use std::env;
use vergen::{BuildBuilder, Emitter};
use vergen_git2::Git2Builder;

fn main() -> anyhow::Result<()> {
    let target = env::var("TARGET").expect("Failed to read env var TARGET");
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("../other/assets/common/MinusGames.rc", embed_resource::NONE);
    }

    let build = BuildBuilder::all_build()?;
    let git = Git2Builder::all_git()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&git)?
        .emit()
}
