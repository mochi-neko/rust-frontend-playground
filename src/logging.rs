use log::LevelFilter;

pub(crate) fn initialize() -> anyhow::Result<()> {
    dioxus_logger::init(LevelFilter::Info)?;

    Ok(())
}
