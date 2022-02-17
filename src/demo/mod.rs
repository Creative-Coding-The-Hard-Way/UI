//! This module defines a standard "demo" application which includes some state.

mod app_state;
mod application;
mod demo_error;
mod multiline_format;

use ::anyhow::{Context, Result};

pub use self::{
    app_state::State, application::Application, demo_error::DemoError,
};

pub fn run_application<S: State>() -> Result<()> {
    multiline_format::enable_multiline_logging()?;

    let result = Application::<S>::new()
        .context("failed to construct the application!")?
        .run()
        .context("application exited with an error");

    if let Err(ref error) = result {
        log::error!(
            "Application exited unsuccessfully!\n{:?}\n\nroot cause: {:?}",
            error,
            error.root_cause()
        );
    }
    result
}
