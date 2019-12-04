pub(crate) fn onclick_event(t: &str, k: &str, v: &str) -> String {
  format!(
    "solitaire.on_event('{}', '{}', {});return false;",
    t,
    k,
    if v.is_empty() { "''" } else { v }
  )
}
