mod fibonacci;
mod user;

use fibonacci::loop_dyn_fib;
use user::*;

fn main() {
    let num = get_loop_range();
    loop_dyn_fib(num);
    end();
}
