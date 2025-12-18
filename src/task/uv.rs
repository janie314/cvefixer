use crate::{
    errors::Error,
    helpers::{cmd_exists, exec},
};
use log::info;
use std::process::Command;

pub fn update() -> Result<(), Error> {
    if cmd_exists("uv") {
        info!("updating uv");
        exec(Command::new("uv").args(["self", "update"]))
    } else {
        info!("no uv found");
        Ok(())
    }
}
