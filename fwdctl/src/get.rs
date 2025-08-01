// Copyright (C) 2023, Ava Labs, Inc. All rights reserved.
// See the file LICENSE.md for licensing terms.

use clap::Args;
use std::str;

use firewood::db::{Db, DbConfig};
use firewood::v2::api::{self, Db as _, DbView as _};

#[derive(Debug, Args)]
pub struct Options {
    /// The key to get the value for
    #[arg(required = true, value_name = "KEY", help = "Key to get")]
    pub key: String,

    /// The database path (if no path is provided, return an error). Defaults to firewood.
    #[arg(
        long,
        required = false,
        value_name = "DB_NAME",
        default_value_t = String::from("firewood"),
        help = "Name of the database"
    )]
    pub db: String,
}

pub(super) async fn run(opts: &Options) -> Result<(), api::Error> {
    log::debug!("get key value pair {opts:?}");
    let cfg = DbConfig::builder().truncate(false);

    let db = Db::new(opts.db.clone(), cfg.build()).await?;

    let hash = db.root_hash().await?;

    let Some(hash) = hash else {
        println!("Database is empty");
        return Ok(());
    };

    let rev = db.revision(hash).await?;

    match rev.val(opts.key.as_bytes()).await {
        Ok(Some(val)) => {
            let s = String::from_utf8_lossy(val.as_ref());
            println!("{s:?}");
            Ok(())
        }
        Ok(None) => {
            eprintln!("Key '{}' not found", opts.key);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
