use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use response::*;
use request::*;
use mime::{extension_to_mime};

pub fn serve(mut req: Request, mut res: Response) {
    let url_path = Path::new(req.url());
    let local_path = Path::new("public")
        .join(url_path.strip_prefix("/").unwrap());

    println!("local_path: {:?}", local_path);

    match File::open(local_path) {
        Ok(mut file) => {
            let mut data = Vec::<u8>::new();
            let _ = file.read_to_end(&mut data);
            res.write(&data);

            res.set_header("content-type", 
                           extension_to_mime(req.url().rsplit(".").next().unwrap_or("")));
        }
        Err(_) => {
            res.set_status_code(404);
            res.set_status_message("Not Found");
        }
    }
    res.end();
}
