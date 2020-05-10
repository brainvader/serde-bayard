use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Episode {
    Jo,
    Ha,
    Q,
}

#[derive(Serialize, Deserialize)]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
}

fn main() {
    let ikari_shinji = Human {
        id: "1".to_owned(),
        name: "Ikari Shinji".to_owned(),
        appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
    };
}
