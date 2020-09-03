use rust_patterns::common::hellosayer::HelloSayer;

use rust_patterns::choice::dyn_obj;
use rust_patterns::choice::enum_with_impl;
use rust_patterns::choice::enum_with_trait;

fn main() {
    enum_with_impl::choose('a').say_hello();
    enum_with_impl::choose('b').say_hello();

    enum_with_trait::choose('a').say_hello();
    enum_with_trait::choose('b').say_hello();

    dyn_obj::choose('a').say_hello();
    dyn_obj::choose('b').say_hello();
}
