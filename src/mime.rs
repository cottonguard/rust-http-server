pub fn extension_to_mime(ext: &str) -> &str {
    match ext {
        "html" => "text/html",
        "js"   => "text/javascript",
        "css"  => "text/css",
        _ => "application/octet-stream"
    }
}
