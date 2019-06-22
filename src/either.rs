// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


macro_rules! EitherDef {
  ( $name:ident, $variant:ident, $($variants:ident),* ) => {
    #[derive(Debug)]
    pub enum $name<$variant,$($variants,)*> {
      $variant($variant),
      $(
        $variants($variants),
      )*
    }

    impl<$variant,$($variants,)*> futures::future::Future for $name<$variant,$($variants,)*>
    where
      $variant: futures::future::Future,
      $(
        $variants: futures::future::Future<Item = $variant::Item, Error = $variant::Error>,
      )*
    {
      type Item = $variant::Item;
      type Error = $variant::Error;

      fn poll(&mut self) -> futures::Poll<$variant::Item, $variant::Error> {
        match *self {
          $name::$variant(ref mut x) => x.poll(),
          $(
            $name::$variants(ref mut x) => x.poll(),
          )*
        }
      }
    }
  };
}


EitherDef! { Either3,  A, B, C }
EitherDef! { Either4,  A, B, C, D }
EitherDef! { Either5,  A, B, C, D, E }
EitherDef! { Either6,  A, B, C, D, E, F }
EitherDef! { Either7,  A, B, C, D, E, F, G }
EitherDef! { Either8,  A, B, C, D, E, F, G, H }
EitherDef! { Either9,  A, B, C, D, E, F, G, H, I }
EitherDef! { Either10, A, B, C, D, E, F, G, H, I, J }
EitherDef! { Either11, A, B, C, D, E, F, G, H, I, J, K }
EitherDef! { Either12, A, B, C, D, E, F, G, H, I, J, K, L }
EitherDef! { Either13, A, B, C, D, E, F, G, H, I, J, K, L, M }
EitherDef! { Either14, A, B, C, D, E, F, G, H, I, J, K, L, M, N }
EitherDef! { Either15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O }
EitherDef! { Either16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P }
EitherDef! { Either17, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q }
EitherDef! { Either18, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R }
EitherDef! { Either19, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S }
EitherDef! { Either20, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T }
