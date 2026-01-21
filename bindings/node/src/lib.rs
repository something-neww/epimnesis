#![deny(clippy::all)]

use engine::plus_100;
use napi_derive::napi;

#[napi(js_name = "plus100")]
pub fn plus100(input: u32) -> u32 {
  plus_100(input)
}
