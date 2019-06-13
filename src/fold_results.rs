// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem::replace;

use futures::Async;
use futures::future::Future;
use futures::future::IntoFuture;
use futures::Poll;
use futures::stream::Stream;


#[derive(Debug)]
enum State<T, F>
where
  F: Future,
{
  /// Placeholder state when doing work.
  Empty,
  /// Ready to process the next stream item; current accumulator is the `T`.
  Ready(T),
  /// Working on a future to process the previous stream item.
  Processing(F),
}


/// A stream combinator which executes a unit closure over each result
/// on a stream.
///
/// Unlike `futures::stream::Stream::fold`, the stream is exhausted and not
/// short circuited on error.
/// This structure is returned by the `StreamExt::fold_results` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct FoldedResults<S, F, Fut, T, E>
where
  Fut: IntoFuture,
{
  stream: S,
  f: F,
  state: State<Result<T, E>, Fut::Future>,
}

impl<S, F, Fut, T, E> FoldedResults<S, F, Fut, T, E>
where
  Fut: IntoFuture,
{
  pub fn new(s: S, f: F, init: Result<T, E>) -> Self
  where
    S: Stream,
    F: FnMut(Result<T, E>, Result<S::Item, S::Error>) -> Fut,
    Fut: IntoFuture<Item = T, Error = E>,
  {
    Self {
      stream: s,
      f: f,
      state: State::Ready(init),
    }
  }
}

impl<S, F, Fut, T, E> Future for FoldedResults<S, F, Fut, T, E>
where
  S: Stream,
  F: FnMut(Result<T, E>, Result<S::Item, S::Error>) -> Fut,
  Fut: IntoFuture<Item = T, Error = E>,
{
  type Item = T;
  type Error = E;

  fn poll(&mut self) -> Poll<T, E> {
    loop {
      match replace(&mut self.state, State::Empty) {
        State::Empty => panic!("cannot poll FoldedResults twice"),
        State::Ready(state) => match self.stream.poll() {
          Ok(Async::Ready(Some(e))) => {
            let future = (self.f)(state, Ok(e));
            let future = future.into_future();
            self.state = State::Processing(future);
          }
          Ok(Async::Ready(None)) => return state.map(Async::Ready),
          Ok(Async::NotReady) => {
            self.state = State::Ready(state);
            return Ok(Async::NotReady);
          }
          Err(e) => {
            let future = (self.f)(state, Err(e));
            let future = future.into_future();
            self.state = State::Processing(future);
          }
        },
        State::Processing(mut fut) => match fut.poll()? {
          Async::Ready(state) => self.state = State::Ready(Ok(state)),
          Async::NotReady => {
            self.state = State::Processing(fut);
            return Ok(Async::NotReady);
          }
        },
      }
    }
  }
}
