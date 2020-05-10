use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Human {
    id: String,
    name: String,
    age: u8,
}

fn main() -> Result<(), serde_json::error::Error> {
    let name = "Ikari Shinji";
    let ikari_shinji = Human {
        id: "1".to_owned(),
        name: name.to_owned(),
        age: 14,
    };

    let ikari_shiniji_str = serde_json::to_string_pretty(&ikari_shinji)?;
    println!("{}", ikari_shiniji_str);
    Ok(())
}
