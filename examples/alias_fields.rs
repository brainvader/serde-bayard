use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Episode {
    #[serde(alias = "/episode/Jo")]
    Jo,
    #[serde(alias = "/episode/Ha")]
    Ha,
    #[serde(alias = "/episode/Q")]
    Q,
}

#[derive(Serialize, Deserialize, Debug)]
struct Human {
    #[serde(alias = "_id")]
    id: String,
    name: String,
    appears_in: Vec<Episode>,
}

fn main() -> Result<(), serde_json::error::Error> {
    let ikari_shinji_str = "{\"_id\":\"1\",\"appears_in\":[\"/episode/Jo\",\"/episode/Ha\",\"/episode/Q\"],\"name\":\"Ikari Shinji\"}";

    let ikari_shinji: Human = serde_json::from_str(&ikari_shinji_str)?;
    println!("{:?}", ikari_shinji);

    let ikari_shinji_re = serde_json::to_string_pretty(&ikari_shinji)?;
    println!("{}", ikari_shinji_re);

    Ok(())
}
