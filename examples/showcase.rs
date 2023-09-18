use oak_http_server::{Server, Status};

fn main() {
    let hostname = "localhost";
    let port: u16 = 2300;

    let mut server = Server::new(hostname, port);
    server.on("/test", |request, response| {
        response.end_with(format!(
            "Your current query options are:\n{}",
            request
                .target
                .queries
                .iter()
                .map(|(name, value)| format!("{}: {}\n", name, value))
                .collect::<String>()
        ))?;

        Ok(())
    });

    server.on_get("/add", |request, mut response| {
        // Initialize variables to store integers
        let mut first: usize = 0;
        let mut second: usize = 0;

        let mut success = true;

        // Create a slice and a function to correctly parse query arguments to the variables
        let variables_slice = (&["first", "second"], &mut [&mut first, &mut second]);

        // For each query we are looking for, check if it exists and attempt to parse it into a usize
        // In case an error occurs, immediately break the loop and execute fail code
        for (&name, reference) in variables_slice.0.iter().zip(variables_slice.1.iter_mut()) {
            if let Some(raw_value) = request.target.queries.get(name) {
                if let Ok(parsed_value) = raw_value.parse::<usize>() {
                    **reference = parsed_value
                } else {
                    success = false;
                    break;
                }
            } else {
                success = false;
                break;
            }
        }

        // If there was an error parsing or finding the query parameters, respond with a 400 status code and return
        if !success {
            response.status(Status::BadRequest);
            response.end_with("Error while parsing query arguments \"first\" and \"second\"")?;
            return Ok(());
        }

        // Add both variables together and return them
        response.end_with((first + second).to_string())?;

        Ok(())
    });

    server.start(|| {
        println!("Successfully initiated server");
    });
}
