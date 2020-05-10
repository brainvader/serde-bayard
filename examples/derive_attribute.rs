use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Episode {
    Jo,
    Ha,
    Q,
}

#[derive(Serialize, Deserialize, Debug)]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
}

fn to_string(human: &Human) -> serde_json::error::Result<String> {
    serde_json::to_string_pretty(human)
}

fn from_string(human_str: &str) -> serde_json::error::Result<Human> {
    serde_json::from_str(human_str)
}

fn main() -> Result<(), serde_json::error::Error> {
    let ikari_shinji = Human {
        id: "1".to_owned(),
        name: "Ikari Shinji".to_owned(),
        appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
    };

    let ikari_shinji_str = to_string(&ikari_shinji)?;
    println!("{}", ikari_shinji_str);

    let ikari_shinji_org = from_string(&ikari_shinji_str)?;

    println!("{:?}", ikari_shinji_org);
    Ok(())
}
