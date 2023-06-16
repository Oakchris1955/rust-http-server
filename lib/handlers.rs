use std::{fs, mem};

use crate::{Request, Response};

fn read_file(parent_dir: String, request: Request, mut response: Response) {
    match fs::read_to_string(
        parent_dir.chars().skip(1).collect::<String>() + &request.target.relative_path,
    ) {
        Ok(contents) => response.send(contents),
        Err(error) => {
            use crate::enums::Status;
            use std::io::ErrorKind;

            let status: Status = match error.kind() {
                ErrorKind::NotFound => Status::NotFound,
                _ => Status::InternalError,
            };

            let status_string = status.to_string();

            response.status(status);
            response.send(status_string);
        }
    }
}

/// Read a file from the same directory as the one specified during the handler's creation
///
/// # Example:
///
/// ```
/// use oak_http_server::{handlers::read_same_dir, Server};
///
/// fn main() {
///	    let hostname = "localhost";
///     let port: u16 = 2300;
///
///     let mut server = Server::new(hostname, port);
///		// If the server were to be started, any content the server would provide for the `/www` directory would be readen from the local `www` directory
///     server.on_directory("/www", read_same_dir);
/// }
/// ```
pub fn read_same_dir(request: Request, response: Response) {
    read_file(request.target.target_path.clone(), request, response)
}

/// Read a file from the directory different than the one specified during the handler's creation
///
/// # Example:
///
/// ```
/// use oak_http_server::{handlers::read_same_dir, Server};
///
/// fn main() {
///	    let hostname = "localhost";
///     let port: u16 = 2300;
///
///     let mut server = Server::new(hostname, port);
///		// If the server were to be started, any content the server would provide for the `/www` directory would be readen from the local `etc` directory
///     server.on_directory("/www", read_diff_dir("etc"));
/// }
/// ```
pub fn read_diff_dir(mut parent_dir: String) -> impl FnMut(Request, Response) {
    move |request: Request, response: Response| {
        read_file(mem::take(&mut parent_dir), request, response)
    }
}
