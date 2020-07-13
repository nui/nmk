use serde::Serialize;

#[derive(Serialize)]
struct Info {
    cargo: Cargo,
}

#[derive(Serialize)]
struct Cargo {
    target: &'static str,
}

pub fn display_info() {
    let info = Info {
        cargo: Cargo {
            target: env!("CARGO_TARGET"),
        },
    };
    if let Ok(json) = serde_json::to_string_pretty(&info) {
        println!("{}", json);
    }
}
