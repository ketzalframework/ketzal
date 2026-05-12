pub fn normalize_path(base: &str, route: &str) -> String {
    let base = base.trim_end_matches('/');
    let route = route.trim_start_matches('/');

    match route.is_empty() {
        true => base.to_string(),
        false => format!("{}/{}", base, route),
    }
}
