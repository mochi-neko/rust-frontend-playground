pub(crate) struct SignupInfo {
    pub(crate) mail_address: String,
    pub(crate) password: String,
}

pub(crate) fn register(info: &SignupInfo) -> anyhow::Result<()> {
    Ok(())
}