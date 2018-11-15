pub fn extension_to_mime(ext: &str) -> &str {
    match ext {
        "txt"  => "text/plain",
        "text" => "text/plain",
        "htm"  => "text/html",
        "html" => "text/html",
        "js"   => "text/javascript",
        "css"  => "text/css",
        "json" => "application/json",
        "xml"  => "application/xml",
        "png"  => "image/png",
        "jpg"  => "image/jpeg",
        "jpeg" => "image/jpeg",
        "svg"  => "image/svg+xml",
        "ico"  => "image/x-icon",
        _ => "application/octet-stream"
    }
}
