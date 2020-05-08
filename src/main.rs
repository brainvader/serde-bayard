use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Episode {
    Jo,
    Ha,
    Q,
}

#[derive(Debug, Serialize, Deserialize)]
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
    let ikari_shinji_str =
        serde_json::to_string(&ikari_shinji).expect("cant't convert into string");

    println!("result \n {}", ikari_shinji_str);

    let ayanami_rei_str = "
    {
        \"id\": \"2\",
        \"name\": \"Ayanami Rei\",
        \"appears_in\": [\"/episode/Jo\", \"/episode/Ha\", \"/episode/Q\"]
    }";
    let anayami_rei: Human = serde_json::from_str(&ayanami_rei_str).unwrap();
    println!("{:?}", anayami_rei);
}
