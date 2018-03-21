#![feature(exhaustive_patterns)]
#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate tokio_core;
extern crate tokio_io;
extern crate futures_await as futures;
extern crate future_utils;
#[macro_use]
extern crate unwrap;
#[macro_use]
extern crate net_literals;

mod priv_prelude;

use priv_prelude::*;

mod igd;

#[derive(Debug)]
pub struct Results {
    igd: igd::IgdResults,
}

#[async]
fn test(handle: Handle) -> Result<Results, !> {
    let Ok(igd) = await!(igd::check_igd(handle));
    Ok(Results {
        igd,
    })
}

fn main() {
    let mut core = unwrap!(Core::new());
    let handle = core.handle();
    let Ok(results) = core.run(test(handle));

    println!("got results: {:?}", results);
}

