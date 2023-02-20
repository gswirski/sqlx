use crate::any::AnyConnection;
use crate::connection::{ConnectOptions, LogSettings};
use crate::error::Error;
use futures_core::future::BoxFuture;
use log::LevelFilter;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

use crate::any::kind::AnyKind;

/// Opaque options for connecting to a database. These may only be constructed by parsing from
/// a connection url.
///
/// ```text
/// postgres://postgres:password@localhost/database
/// mysql://root:password@localhost/database
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AnyConnectOptions {
    pub database_url: Url,
    pub log_settings: LogSettings,
}
impl FromStr for AnyConnectOptions {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Ok(AnyConnectOptions {
            database_url: url
                .parse::<Url>()
                .map_err(|e| Error::Configuration(e.into()))?,
            log_settings: LogSettings::default(),
        })
    }
}

impl ConnectOptions for AnyConnectOptions {
    type Connection = AnyConnection;

    fn from_url(url: &Url) -> Result<Self, Error> {
        Ok(AnyConnectOptions {
            database_url: url.clone(),
            log_settings: LogSettings::default(),
        })
    }

    #[inline]
    fn connect(&self) -> BoxFuture<'_, Result<AnyConnection, Error>> {
        Box::pin(AnyConnection::establish(self))
    }

    fn log_statements(mut self, level: LevelFilter) -> Self {
        self.log_settings.statements_level = level;
        self
    }

    fn log_slow_statements(mut self, level: LevelFilter, duration: Duration) -> Self {
        self.log_settings.slow_statements_level = level;
        self.log_settings.slow_statements_duration = duration;
        self
    }

    fn pretty_print(&mut self, pretty_print: bool) -> &mut Self {
        match &mut self.0 {
            #[cfg(feature = "postgres")]
            AnyConnectOptionsKind::Postgres(o) => {
                o.pretty_print(pretty_print);
            }

            #[cfg(feature = "mysql")]
            AnyConnectOptionsKind::MySql(o) => {
                o.pretty_print(pretty_print);
            }

            #[cfg(feature = "sqlite")]
            AnyConnectOptionsKind::Sqlite(o) => {
                o.pretty_print(pretty_print);
            }

            #[cfg(feature = "mssql")]
            AnyConnectOptionsKind::Mssql(o) => {
                o.pretty_print(pretty_print);
            }
        };
        self
    }
}
