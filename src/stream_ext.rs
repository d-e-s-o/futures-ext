// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::result::Result;

use futures::future::IntoFuture;
use futures::stream::Stream;

use crate::fold_results::FoldedResults;
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

  fn fold_results<F, T, E, Fut>(self, init: Result<T, E>, f: F) -> FoldedResults<Self, F, Fut, T, E>
  where
    Self: Sized + Stream,
    F: FnMut(Result<T, E>, Result<Self::Item, Self::Error>) -> Fut,
    Fut: IntoFuture<Item = T, Error = E>,
  {
    FoldedResults::new(self, f, init)
  }
}

impl<S> StreamExt for S where S: Stream {}
