use oak_http_server::{HttpServer, HttpStatus};

fn main() {
    let hostname = "localhost";
    let port: u16 = 2300;

    let mut server = HttpServer::new(hostname, port);
    server.on("/test", |request, response| {
        response.send(format!(
            "Your current query options are:\n{}",
            request
                .target
                .queries
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
        ))
    });

    server.on_get("/add", |request, mut response| {
        // Initialize variables to store integers
        let mut first: Option<usize> = None;
        let mut second: Option<usize> = None;

        // Create a slice and a function to correctly parse query arguments to the variables
        let variables_slice = (&["first", "second"], &mut [&mut first, &mut second]);

        fn convert_to_usize(variable: &mut Option<usize>, num_string: String) {
            match num_string.parse::<usize>() {
                Ok(number) => *variable = Some(number),
                _ => (),
            }
        }

        // Loop through each query and if a valid argument is found, put its value into a variable
        for query in request.target.queries {
            match query.name.as_str() {
                "first" | "second" => convert_to_usize(
                    variables_slice.1[variables_slice
                        .0
                        .iter()
                        .position(|x| &query.name == x)
                        .unwrap()],
                    query.value,
                ),
                _ => (),
            }
        }

        // If both variables have some value, add them together and return them
        if let (Some(first_num), Some(second_num)) = (first, second) {
            response.send((first_num + second_num).to_string());

        // Otherwise, respond with a Bad Request (400) HTTP code
        } else {
            response.status(HttpStatus::BadRequest);
            response.send("Error while parsing query arguments \"first\" and \"second\"");
        }
    });

    server.start(|| {
        println!("Successfully initiated server");
    });
}
