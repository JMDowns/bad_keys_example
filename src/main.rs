use bad_keys_example::run;

fn main() {
    pollster::block_on(run());
}
