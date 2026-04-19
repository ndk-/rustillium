/// Type alias for cached secret data: the list of key/value pairs, or an error string.
pub type CachedSecretsResult = Result<Vec<(String, String)>, String>;
