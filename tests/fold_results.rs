// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use futures::future::Future;
use futures::stream::empty;
use futures::stream::iter_result;

use futures_ext::StreamExt;


#[test]
fn empty_stream() {
  let init = Ok(());
  let fut = empty::<_, ()>().fold_results(init, |acc, res| acc.and(res));

  let result = fut.wait();
  assert_eq!(result, Ok(()));
}

#[test]
fn sequence_results() {
  fn test(seq: &[Result<u64, &str>], expected: Result<u64, ()>) {
    let stream = iter_result(seq.iter().cloned());
    let init = Ok(0);
    let future = stream.fold_results(init, |acc, res| {
      acc.and_then(|acc| match res {
        Ok(val) => Ok(acc + val),
        Err(_) => Err(()),
      })
    });

    let result = future.wait();
    assert_eq!(result, expected);
  }

  test(&[], Ok(0));
  test(&[Ok(1337)], Ok(1337));
  test(&[Err("hello")], Err(()));
  test(&[Ok(42), Ok(3)], Ok(45));
  test(&[Ok(13), Ok(7), Err("err")], Err(()));
}
