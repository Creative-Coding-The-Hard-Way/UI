use ::{flexi_logger::FlexiLoggerError, thiserror::Error};

#[derive(Debug, Error)]
pub enum DemoError {
    #[error("Unable to enable pretty multiline logging!")]
    MultiLineLogSetupError(#[source] FlexiLoggerError),
}
