use rand::{seq::SliceRandom, thread_rng};

const ADJECTIVES: &[&'static str] = &[
    "Coole",
    "Nice",
    "Mausige",
    "Tolle",
    "Kleine",
    "Große",
    "Slayende",
    "Musikalische",
    "Smarte",
    "Achtsame",
    "Fleißige",
    "Großartige",
    "Lässige",
    "Fabelhafte",
    "Famose",
    "Charmante",
    "Tiefsinnige",
    "Fesche",
    "Knorke",
    "Schnatternde",
    "Urige",
    "Fetzige",
    "Rücksichtsvolle",
    "Funkelnde",
    "Glitzernde",
    "Flauschige",
    "Solidarische",
    "Demokratische",
    "Autonome",
    "Knuddelige",
    "Hochbegabte",
    "Emsige",
    "Anmutige",
    "Empathische",
    "Rücksichtsvolle",
    "Geduldige",
];

const NOUNS: &[&'static str] = &[
    "Mäuse",
    "Tapire",
    "Ameisenbären",
    "Dackel",
    "Otter",
    "Seehunde",
    "Löwen",
    "Pandas",
    "Igel",
    "Hasen",
    "Koalas",
    "Meerschweinchen",
    "Frösche",
    "Pinguine",
    "Wombats",
    "Kängurus",
    "Regenwürmer",
    "Giraffen",
    "Enten",
    "Elefanten",
    "Hummeln",
    "Schnecken",
    "Flamingos",
    "Delphine",
    "Kiwis",
    "Numbats",
    "Quokkas",
    "Fledermäuse",
    "Schwane",
    "Ohrenkneifer",
    "Blutegel",
    "Hippos",
    "Ameisen",
    "Biber",
    "Katzen",
    "Belugawale",
    "Hammerhaie",
];

pub fn random_name() -> String {
    let mut rng = thread_rng();
    format!(
        "{} {}",
        ADJECTIVES.choose(&mut rng).unwrap(),
        NOUNS.choose(&mut rng).unwrap()
    )
}
