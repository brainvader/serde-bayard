use serde::Serialize;

#[derive(Serialize)]
enum Character {
    Human { id: String, name: String, age: u8 },
    Evangelion { unit: String, pilot: String },
    Angel { id: String, name: String },
}

fn main() -> Result<(), serde_json::error::Error> {
    let name = "Ikari Shinji";
    let ikari_shinji = Character::Human {
        id: "1".to_owned(),
        name: name.to_owned(),
        age: 14,
    };

    let evangelion_01 = Character::Evangelion {
        unit: "01".to_owned(),
        pilot: name.to_owned(),
    };

    let sachiel = Character::Angel {
        id: "4".to_owned(),
        name: "Sachiel".to_owned(),
    };

    let ikari_shiniji_str = serde_json::to_string_pretty(&ikari_shinji)?;
    println!("{}", ikari_shiniji_str);
    Ok(())
}
