// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use futures::future::err;
use futures::future::Future;
use futures::future::ok;

use futures_ext::future::Either4;


#[test]
fn either() {
  // We only "test" a single EitherX variant. They are generated
  // programmatically for the most part and should not differ.
  fn test(x: u64) -> Result<u64, ()> {
    let future = match x {
      1 => Either4::A(ok(3)),
      2 => Either4::B(ok(7)),
      5 => Either4::C(ok(11)),
      _ => Either4::D(err(())),
    };

    future.wait()
  }

  assert_eq!(test(1), Ok(3));
  assert_eq!(test(2), Ok(7));
  assert_eq!(test(5), Ok(11));
  assert_eq!(test(8), Err(()));
}
