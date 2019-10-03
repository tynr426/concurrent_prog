
mod channel_learn;
use channel_learn::channel_test;
mod mutex_learn;
use mutex_learn::mutex_test;
fn main() {
    channel_test();
    mutex_test();
}
