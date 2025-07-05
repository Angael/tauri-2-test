use serde::{Deserialize, Deserializer};

// Helper function to deserialize a field, falling back to default on error.
pub fn deserialize_or_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    // We first deserialize into a generic serde_json::Value.
    // This allows us to handle any valid JSON value.
    let value = serde_json::Value::deserialize(deserializer)?;

    // Then, we attempt to deserialize the generic Value into our target type T.
    // If this fails, `unwrap_or_default()` will provide T::default().
    Ok(T::deserialize(value).unwrap_or_default())
}

pub fn deserialize_vec_skip_errors<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    // First, deserialize the entire array into a `Vec` of generic `Value`s.
    let values: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;

    // Iterate over the `Value`s, attempt to deserialize each into type `T`,
    // and collect only the successful ones.
    let results: Vec<T> = values
        .into_iter()
        .filter_map(|v| T::deserialize(v).ok()) // .ok() converts Result to Option, filter_map unwraps Some and discards None
        .collect();

    Ok(results)
}
