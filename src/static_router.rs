use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use response::*;
use request::*;

pub fn serve(mut req: Request, mut res: Response) {
    let local_path = Path::new("public")
        .join(Path::new(&req.url).strip_prefix("/").unwrap());

    println!("local_path: {:?}", local_path);

    match File::open(local_path) {
        Ok(mut file) => {
            let mut data = Vec::<u8>::new();
            let _ = file.read_to_end(&mut data);
            res.write(&data);
        }
        Err(_) => {
            res.status_code = 404;
            res.status_message = String::from("Not Found");
        }
    }
    res.end();
}
