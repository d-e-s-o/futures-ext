// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use futures::future::err;
use futures::future::Future;
use futures::future::ok;
use futures::stream::empty;
use futures::stream::iter_result;

use futures_ext::StreamExt;


#[test]
fn empty_stream() {
  let future = empty::<bool, ()>().for_each_result(|_| ok(()));

  let result = future.wait();
  assert_eq!(result, Ok(()));
}

#[test]
fn empty_stream_with_error() {
  let future = empty::<bool, ()>().for_each_result(|_| err(()));

  let result = future.wait();
  assert_eq!(result, Ok(()));
}

#[test]
fn sequences_ok() {
  fn test(seq: &[Result<u64, &str>]) {
    let mut iter = seq.iter();
    let stream = iter_result(iter.clone().cloned());

    let future = stream.for_each_result(|result| {
      assert_eq!(&result, iter.next().unwrap());
      ok(())
    });

    let result = future.wait();
    assert!(iter.next().is_none());
    assert_eq!(result, Ok(()));
  }

  test(&[]);
  test(&[Ok(20)]);
  test(&[Err("foo?")]);
  test(&[Err("bar!"), Ok(3)]);
  test(&[Ok(1), Ok(42), Err("error"), Ok(23)]);
}

#[test]
fn sequence_results() {
  fn test(seq: &[Result<u64, &str>], expected: Result<(), ()>) {
    let stream = iter_result(seq.iter().cloned());
    let future = stream.for_each_result(|result| result.map(|_| ()).map_err(|_| ()));

    let result = future.wait();
    assert_eq!(result, expected);
  }

  test(&[], Ok(()));
  test(&[Ok(1337)], Ok(()));
  test(&[Err("hello")], Err(()));
}

#[test]
fn short_circuit_on_error() {
  let results = [Ok(9), Err("test"), Ok(10)];
  let mut iter = results.iter();
  let stream = iter_result(iter.clone().cloned());

  let future = stream.for_each_result(|result| {
    let _ = iter.next();
    result.map(|_| ()).map_err(|_| ())
  });

  let result = future.wait();
  assert_eq!(result, Err(()));
  assert_eq!(iter.next(), Some(&Ok(10)));
}
