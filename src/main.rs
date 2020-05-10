use serde::{de, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Episode {
    #[serde(alias = "/episode/Jo")]
    Jo,
    #[serde(alias = "/episode/Ha")]
    Ha,
    #[serde(alias = "/episode/Q")]
    Q,
}

#[derive(Debug, Serialize, Deserialize)]
struct Human {
    #[serde(deserialize_with = "deserialize_from_array")]
    #[serde(alias = "_id")]
    id: String,
    #[serde(deserialize_with = "deserialize_from_array")]
    name: String,
    appears_in: Vec<Episode>,
}

fn deserialize_from_array<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: de::Deserializer<'de>,
{
    struct ActualDataVisitor<T>(std::marker::PhantomData<fn() -> T>);

    impl<'de, T> de::Visitor<'de> for ActualDataVisitor<T>
    where
        T: Deserialize<'de>,
    {
        // Deserialize into
        type Value = T;

        // For error message
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a nonempty sequence of data")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            // You can use IgnoredAny to skip over the first nth elements.
            let actual_data = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("no values in seq when looking for maximum"))?;
            Ok(actual_data)
        }
    }
    let visitor = ActualDataVisitor(std::marker::PhantomData);
    deserializer.deserialize_seq(visitor)
}

fn main() {
    let ikari_shinji = Human {
        id: "1".to_owned(),
        name: "Ikari Shinji".to_owned(),
        appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
    };
    let ikari_shinji_str =
        serde_json::to_string(&ikari_shinji).expect("cant't convert into string");

    println!("Ikari Shinji \n {}", ikari_shinji_str);

    let ayanami_rei_doc = "{\"_id\":[\"0\"],\"appears_in\":[\"/episode/Jo\",\"/episode/Ha\",\"/episode/Q\"],\"name\":[\"Ayanami Rei\"]}";
    let anayami_rei: Human = serde_json::from_str(&ayanami_rei_doc).unwrap();
    println!("{:?}", anayami_rei);
}
