use oak_http_server::{Cookie, Server};
use std::time::{Duration, SystemTime};

fn main() {
    let hostname = "localhost";
    let port: u16 = 2300;

    let mut server = Server::new(hostname, port);
    server.on("/set-cookies", |request, mut response| {
        // Loop throught URL queries and set them as cookies
        for (name, value) in request.target.queries.iter() {
            response.set_cookie({
                let mut cookie = Cookie::new(name, value);
                cookie.set_expires(SystemTime::now() + Duration::from_secs(60 * 60));
                cookie
            });
        }

        // End request with an informative message
        response.end_with(format!(
            "Cookies \"{}\" set",
            request
                .target
                .queries
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        ))?;

        Ok(())
    });

    server.on("/get-cookies", |request, mut response| {
        response.send("Your cookies are:\n")?;

        // Loop through request cookies and send them to the user
        for (name, value) in request.cookies.iter() {
            response.send(format!("{}: {}\n", name, value))?;
        }

        response.end()?;

        Ok(())
    });

    server.start(|| {
        println!("Successfully initiated server");
    });
}
