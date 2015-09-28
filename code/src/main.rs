extern crate time;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_time;

use std::fmt::Display;
use time::Duration;
use carboxyl::{ Stream };
use carboxyl_time::{ every };


fn watch<T>(stream: Stream<T>)
    where T: Display + Clone + Send + Sync + 'static
{
    for event in stream.events() {
        println!("{}", event);
    }
}


fn counter() -> Stream<i32> {
    let pulse = every(Duration::milliseconds(200));
    pulse
        .scan(0, |a, _| a + 1)
        .snapshot(&pulse, |a, _| a)
}


fn main() {
    let count: Stream<i32> = counter();

    watch(count);
}
