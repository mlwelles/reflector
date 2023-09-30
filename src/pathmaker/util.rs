use super::PathMakerError;

fn atoi(s: &str, err: PathMakerError) -> Result<i32, PathMakerError> {
    match s.parse() {
        Ok(x) => Ok(x),
        Err(_) => Err(err),
    }
}
