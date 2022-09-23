const VERSION_GLUE: &str = ".";
const VARIANT_GLUE: &str = "-";

fn generate_left(segments: &[String], glue: &str) -> Vec<String> {
    (1..=segments.len())
        .map(|i| {
            segments
                .iter()
                .take(i)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(glue)
        })
        .collect()
}

fn generate_right(segments: &[String], glue: &str) -> Vec<String> {
    (0..segments.len())
        .map(|i| {
            segments
                .iter()
                .skip(i)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(glue)
        })
        .collect()
}

pub struct Version {
    segments: Vec<String>,
}

impl Version {
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    pub fn generate(&self) -> Vec<String> {
        generate_left(&self.segments, VERSION_GLUE)
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        Self::new(s.split(VERSION_GLUE).map(|s| s.to_string()).collect())
    }
}

impl From<String> for Version {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

pub fn tagen(version: impl Into<Version>, variants: &[&str]) -> Vec<String> {
    version
        .into()
        .generate()
        .into_iter()
        .flat_map(|version| match variants.len() {
            0 => vec![version],
            _ => generate_right(
                &variants.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                VARIANT_GLUE,
            )
            .into_iter()
            .map(move |variant| format!("{}{}{}", &version, VARIANT_GLUE, variant))
            .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_variants() {
        assert_eq!(vec!["1", "1.64", "1.64.0"], tagen("1.64.0", &vec![]),);
    }

    #[test]
    fn test_with_variants() {
        assert_eq!(
            vec![
                "1-bullseye-slim",
                "1-slim",
                "1.64-bullseye-slim",
                "1.64-slim",
                "1.64.0-bullseye-slim",
                "1.64.0-slim",
            ],
            tagen("1.64.0", &vec!["bullseye", "slim"]),
        );
    }
}
