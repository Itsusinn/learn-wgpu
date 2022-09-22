use std::fmt::Debug;

use color_eyre::eyre::{ErrReport, Result};

pub trait EyreExt<E> {
  fn to_eyre(self) -> ErrReport;
}

impl<E> EyreExt<E> for E
where
  E: Into<ErrReport> + Debug + Sync + Send,
{
  fn to_eyre(self) -> ErrReport {
    color_eyre::eyre::eyre!(self)
  }
}

pub trait ResultExt<T, E> {
  fn ignore(self) -> Option<T>;
  fn log(self) -> Option<T>;
  fn eyre_log(self) -> Option<T>;
}
impl<T, E> ResultExt<T, E> for Result<T, E>
where
  E: Into<ErrReport> + Debug + Sync + Send,
{
  #[inline]
  fn ignore(self) -> Option<T> {
    match self {
      Ok(v) => Some(v),
      Err(_) => None,
    }
  }

  #[inline]
  fn log(self) -> Option<T> {
    match self {
      Ok(v) => Some(v),
      Err(e) => {
        error!("{:?}", e);
        None
      }
    }
  }

  #[inline]
  fn eyre_log(self) -> Option<T> {
    match self {
      Ok(v) => Some(v),
      Err(e) => {
        error!("{:?}", color_eyre::eyre::eyre!(e));
        None
      }
    }
  }
}
