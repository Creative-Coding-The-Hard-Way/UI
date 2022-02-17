use ::{
    flexi_logger::{DeferredNow, Logger, Record},
    std::fmt::Write as FmtWrite,
    textwrap::{termwidth, Options},
};

use crate::demo::DemoError;

/// Enable multiline logging for this application.
pub(super) fn enable_multiline_logging() -> Result<(), DemoError> {
    Logger::with_env_or_str("info")
        .format(multiline_format)
        .start()
        .map_err(DemoError::MultiLineLogSetupError)?;
    log::info!(
        "adjust log level by setting the RUST_LOG env var - RUST_LOG = 'info'"
    );
    Ok(())
}

/// A formatting function for logs which automaticaly wrap to the terminal
/// width.
pub fn multiline_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let size = termwidth().min(74);
    let wrap_options = Options::new(size)
        .initial_indent("┏ ")
        .subsequent_indent("┃ ");

    let mut full_line = String::new();
    writeln!(
        full_line,
        "{} [{}] [{}:{}]",
        record.level(),
        now.now().format("%H:%M:%S%.6f"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )
    .expect("unable to format first log line");

    write!(&mut full_line, "{}", &record.args())
        .expect("unable to format log!");

    writeln!(w, "{}", textwrap::fill(&full_line, wrap_options))
}
