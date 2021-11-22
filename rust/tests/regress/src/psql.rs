use crate::Opts;
use anyhow::{bail, Context};
use log::info;
use tokio::process::Command;

const PG_DB_NAME: &str = "postgres";

pub(crate) struct Psql {
    opts: Opts,
}

pub(crate) struct PsqlCommandBuilder {
    database: String,
    cmd: Command,
}

impl Psql {
    pub(crate) fn new(opts: Opts) -> Self {
        Self { opts }
    }

    pub(crate) async fn init(&self) -> anyhow::Result<()> {
        info!("Initializing instances.");

        for db in [self.opts.database_name(), PG_DB_NAME] {
            Psql::drop_database_if_exists(db).await?;
            Psql::create_database(db).await?;
        }

        Ok(())
    }

    pub(crate) async fn create_database<S: AsRef<str>>(db: S) -> anyhow::Result<()> {
        info!("Creating database {}", db.as_ref());

        let mut cmd = PsqlCommandBuilder::new(PG_DB_NAME)
            .add_cmd(format!(
                r#"CREATE DATABASE "{}" TEMPLATE=template0 LC_COLLATE='C' LC_CTYPE='C'"#,
                db.as_ref()
            ))
            .build();

        let status = cmd
            .status()
            .await
            .with_context(|| format!("Failed to execute command: {:?}", cmd))?;
        if status.success() {
            info!("Succeeded to create database {}", db.as_ref());
            Ok(())
        } else {
            bail!("Failed to create database {}", db.as_ref())
        }
    }

    pub(crate) async fn drop_database_if_exists<S: AsRef<str>>(db: S) -> anyhow::Result<()> {
        info!("Dropping database {} if exists", db.as_ref());

        let mut cmd = PsqlCommandBuilder::new("postgres")
            .add_cmd("SET client_min_messages = warning")
            .add_cmd(format!(r#"DROP DATABASE IF EXISTS "{}""#, db.as_ref()))
            .build();

        let status = cmd
            .status()
            .await
            .with_context(|| format!("Failed to execute command: {:?}", cmd))?;

        if status.success() {
            info!("Succeeded to drop database {}", db.as_ref());
            Ok(())
        } else {
            bail!("Failed to drop database {}", db.as_ref())
        }
    }
}

impl PsqlCommandBuilder {
    pub(crate) fn new<S: ToString>(database: S) -> Self {
        let mut cmd = Command::new("psql");
        cmd.arg("-X");

        Self {
            database: database.to_string(),
            cmd,
        }
    }

    pub(crate) fn add_cmd<S: AsRef<str>>(mut self, cmd: S) -> Self {
        let cmd = cmd.as_ref();
        let mut escaped_cmd = "".to_string();

        // Escape any shell double-quote metacharacters
        for c in cmd.chars() {
            if r#"\"$`"#.contains(c) {
                escaped_cmd.push('\\');
            }
            escaped_cmd.push(c);
        }

        // Append comand
        self.cmd
            .args(["-c", format!(r#""{}""#, escaped_cmd).as_str()]);

        self
    }

    pub(crate) fn build(mut self) -> Command {
        self.cmd.arg(format!(r#""{}""#, self.database));
        self.cmd
    }
}
