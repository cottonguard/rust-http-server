use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use response::*;
use request::*;
use mime::{extension_to_mime};

const USE_INDEX_HTML: bool = true;

pub fn serve(req: &Request, res: &mut Response) {

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
        res.set_status_code(404);
        res.set_status_message("Not Found");
        let body = b"<h1>Not Found</h1>";
        res.write(body);
        res.set_header("content-length", &body.len().to_string());
        // let _ = res.end();
        return;
    }

    read_file(&local_path, |d| {
        match d {
            Ok(data) => {
                res.write(data);
                res.set_header("content-type",
                    extension_to_mime(local_path.extension()
                                      .map_or("", |oss| oss.to_str().unwrap_or(""))));
                res.set_header("content-length", &data.len().to_string());
            }
            _ => {
                res.set_status_code(404);
                res.set_status_message("Not Found");
            }
        }
        // let _ = res.end();
    });
}

fn read_file(path: impl AsRef<Path>, f: impl FnOnce(io::Result<&[u8]>)) {
    match File::open(path) {
        Ok(mut file) => {
            let mut data = Vec::<u8>::new();
            let _ = file.read_to_end(&mut data);
            f(Ok(&data));
        }
        Err(e) => {
            f(Err(e));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found() {
        read_file("n/o/t/f/o/u/n/d", |result| {
            assert!(result.is_err());
        });
    }
}
