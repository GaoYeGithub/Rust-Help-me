// Copyright 2016-2018 Mateusz Sieczko and other GilRs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::time::SystemTime;

/// Like `(a: f32 / b).ceil()` but for integers.
pub fn ceil_div(a: u32, b: u32) -> u32 {
    if a == 0 {
        0
    } else {
        1 + ((a - 1) / b)
    }
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.max(min).min(max)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn time_now() -> SystemTime {
    SystemTime::now()
}

#[cfg(target_arch = "wasm32")]
pub fn time_now() -> SystemTime {
    use std::time::Duration;

    let epoch = SystemTime::UNIX_EPOCH;
    let time = stdweb::web::Date::new().get_time();
    let offset = Duration::from_millis(time as u64);
    epoch + offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_clamp() {
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp(2.0, 0.0, 1.0), 1.0);
    }
}
