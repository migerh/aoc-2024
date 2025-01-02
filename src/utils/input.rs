use anyhow::Result;

pub fn read_by_line<T>(input: &str, reader: fn(&str) -> Result<T>) -> Result<Vec<T>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(reader)
        .collect::<Result<Vec<_>>>()
}
