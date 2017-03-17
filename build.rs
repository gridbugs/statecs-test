use std::path::Path;

extern crate statecs;

fn main() {
    let config_a = statecs::Config::new();
    let config_b = statecs::Config { single_component_bitfield: false, .. config_a };
    let config_c = statecs::Config { combine_flag_set: false, .. config_a };
    let config_d = statecs::Config { component_bookkeeping: true, .. config_a };

    statecs::generate("ecs_0.toml", Path::new("src")
                      .join("generated/generated_0_a.rs"), config_a);
    statecs::generate("ecs_0.toml", Path::new("src")
                      .join("generated/generated_0_b.rs"), config_b);
    statecs::generate("ecs_0.toml", Path::new("src")
                      .join("generated/generated_0_c.rs"), config_c);
    statecs::generate("ecs_0.toml", Path::new("src")
                      .join("generated/generated_0_d.rs"), config_d);
}
