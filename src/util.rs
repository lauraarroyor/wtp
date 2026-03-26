use serde_json::Value;

/// Returns the string capitalized.
pub fn capitalized(s: &str) -> String {
    let Some(c) = s.chars().nth(0) else {
        return String::new();
    };

    format!("{}{}", c.to_uppercase(), &s[c.len_utf8()..])
}

/// Returns the PokéAPI JSON for the given Pokémon index.
pub async fn get_poke_json(i: usize) -> Result<Value, String> {
    use reqwest::Client;

    let url = format!("https://pokeapi.co/api/v2/pokemon/{i}");

    Client::new()
        .get(url)
        .send()
        .await
        .map_err(|_| "Failed to contact PokéAPI".to_string())?
        .json()
        .await
        .map_err(|_| "Got bad JSON from PokéAPI".to_string())
}
