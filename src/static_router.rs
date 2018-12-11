use std::fs::*;
use std::io;
use std::io::*;
use std::path::Path;
use std::str::pattern::Pattern;

use response::*;
use request::*;
use mime::{extension_to_mime};

const USE_INDEX_HTML: bool = true;
const DEFAULT_CHARSET: &str = "utf-8";

pub fn serve(req: &Request, res: &mut Response) -> io::Result<()> {
    let mut local_path = Path::new("public")
        .join(Path::new(req.url()).strip_prefix("/").unwrap());

    println!("local_path: {:?}", local_path);
    println!("{:?}", local_path.metadata());

    if USE_INDEX_HTML && local_path.is_dir() {
        local_path = local_path.join(Path::new("index.html"));
        println!("local_path: {:?}", local_path);
        println!("{:?}", local_path.metadata());
    }

    if !local_path.is_file() {
        not_found(res);
        return Ok(());
    }

    match File::open(&local_path) {
        Ok(file) => {
            let metadata = file.metadata()?;
            let len = metadata.len();
            let mime_type = extension_to_mime(local_path.extension()
                            .map_or("", |oss| oss.to_str().unwrap_or("")));
            let content_type = if "text".is_prefix_of(mime_type) {
                [mime_type, ";charset=", DEFAULT_CHARSET].join("")
            } else {
                mime_type.to_string()
            };
            res.set_header("content-type", &content_type);
            res.set_header("content-length", &len.to_string());

            let mut br = BufReader::new(file);
            loop {
                let len = {
                    let buf = br.fill_buf()?;
                    if buf.len() == 0 {
                        break;
                    }
                    res.write(buf);
                    buf.len()
                };
                br.consume(len);
            }
        },
        _ => {
            not_found(res);    
        }
    };
    Ok(())
}

fn not_found(res: &mut Response) {
    res.set_status_code(404);
    res.set_status_message("Not Found");
    let body = b"<h1>Not Found</h1>";
    res.write(body);
    res.set_header("content-length", &body.len().to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
}
