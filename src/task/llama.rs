use crate::{
    errors::Error,
    helpers::{cmd_exists, exec},
};
use log::info;
use std::process::Command;

pub fn update() -> Result<(), Error> {
    if cmd_exists("llama") {
        info!("updating llama");
        exec(Command::new("llama").arg("update"))
    } else {
        info!("no llama found");
        Ok(())
    }
}
