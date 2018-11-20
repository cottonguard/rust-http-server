use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use response::*;
use request::*;
use mime::{extension_to_mime};
/*
pub fn serve(mut req: Request, mut res: Response) {
    let local_path = Path::new("public")
        .join(Path::new(req.url()).strip_prefix("/").unwrap());

    println!("local_path: {:?}", local_path);

    if !local_path.is_file() {
        res.set_status_code(404);
        res.set_status_message("Not Found");
        let _ = res.end();
        return;
    }

    read_file(local_path, move |d| {
        match d {
            Ok(data) => {
                res.write(data);
                res.set_header("content-type",
                    extension_to_mime(req.url().rsplit(".").next().unwrap_or("")));
                res.set_header("content-length", &data.len().to_string());
            }
            _ => {
                res.set_status_code(404);
                res.set_status_message("Not Found");
            }
        }
        let _ = res.end();
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
}*/
