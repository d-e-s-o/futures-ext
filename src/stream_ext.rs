// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::result::Result;

use futures::future::IntoFuture;
use futures::stream::Stream;

use crate::for_each_result::ForEachResult;


pub trait StreamExt {
  fn for_each_result<F, R>(self, f: F) -> ForEachResult<Self, F, R>
  where
    Self: Sized + Stream,
    F: FnMut(Result<Self::Item, Self::Error>) -> R,
    R: IntoFuture<Item = (), Error = ()>,
  {
    ForEachResult::new(self, f)
  }
}

impl<S> StreamExt for S where S: Stream {}
