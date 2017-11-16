use hyper::Error as HyperError;
use serde_json::Error as SerdeError;
use postgres::Error as PostgresError;

#[derive(Debug)]
pub enum Error {
  Hyper(HyperError),
  Serde(SerdeError),
  Postgres(PostgresError)
}

impl Error {
  // return the error message in json format
  pub fn json(self) -> String {
    match self {
      Error::Hyper(_) => format!(r#"{{"error":{{"code":"error/hyper"}}}}"#),
      Error::Serde(_) => format!(r#"{{"error":{{"code":"error/serde-json"}}}}"#),
      Error::Postgres(_) => format!(r#"{{"error":{{"code":"error/postgres-database"}}}}"#),
    }
  }
}

macro_rules! impl_from {
  ($v:path, $t:ty) => {
    impl From<$t> for Error {
      fn from(err: $t) -> Self {
        $v(err)
      }
    }
  }
}

impl_from!(Error::Hyper, HyperError);
impl_from!(Error::Serde, SerdeError);
impl_from!(Error::Postgres, PostgresError);
