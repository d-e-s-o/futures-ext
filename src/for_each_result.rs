// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::result::Result;

use futures::Async;
use futures::future::Future;
use futures::future::IntoFuture;
use futures::Poll;
use futures::stream::Stream;


/// A stream combinator which executes a unit closure over each result
/// on a stream.
///
/// Like `futures::stream::Stream::for_each`, the implicit loop can be
/// short circuited by returning an error, but unlike there, the client
/// has full control over this fact.
/// This structure is returned by the `StreamExt::for_each_result`
/// method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct ForEachResult<S, F, R>
where
  R: IntoFuture,
{
  stream: S,
  f: F,
  fut: Option<R::Future>,
}

impl<S, F, R> ForEachResult<S, F, R>
where
  R: IntoFuture,
{
  pub fn new(s: S, f: F) -> Self
  where
    S: Stream,
    F: FnMut(Result<S::Item, S::Error>) -> R,
    R: IntoFuture<Item = (), Error = ()>,
  {
    Self {
      stream: s,
      f: f,
      fut: None,
    }
  }
}

impl<S, F, R> Future for ForEachResult<S, F, R>
where
  S: Stream,
  F: FnMut(Result<S::Item, S::Error>) -> R,
  R: IntoFuture<Item = (), Error = ()>,
{
  type Item = ();
  type Error = ();

  fn poll(&mut self) -> Poll<(), ()> {
    loop {
      if let Some(mut fut) = self.fut.take() {
        if fut.poll()?.is_not_ready() {
          self.fut = Some(fut);
          return Ok(Async::NotReady);
        }
      }

      let result = match self.stream.poll() {
        Ok(Async::Ready(None)) => return Ok(Async::Ready(())),
        Ok(Async::Ready(Some(t))) => Ok(t),
        Ok(Async::NotReady) => return Ok(Async::NotReady),
        Err(e) => Err(e),
      };

      self.fut = Some((self.f)(result).into_future());
    }
  }
}
