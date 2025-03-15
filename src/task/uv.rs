use crate::{errors::Error, helpers::exec};
use log::{info, warn};
use std::{
    fs::{self, File},
    io::{self, BufRead},
    process::Command,
};
use time::{format_description::well_known::Rfc2822, Duration, OffsetDateTime};

pub fn update() -> Result<(), Error> {
    info!("updating uv");
    exec(Command::new("uv").args(["self", "update"]))?;
    Ok(())
}
