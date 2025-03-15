use crate::{errors::Error, helpers::exec};
use log::info;
use std::process::Command;

pub fn update() -> Result<(), Error> {
    info!("updating uv");
    exec(Command::new("uv").args(["self", "update"]))?;
    Ok(())
}
