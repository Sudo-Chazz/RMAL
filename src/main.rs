use reqwest;

pub struct Args {
    pub mac_address: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            mac_address: get_nth_arg(1),
        }
    }
}

fn main() {
    let args = Args::new();
    let vendor = match request_and_response(args.mac_address) {
        Ok(response) => response,
        Err(e) =>  match e {
            e if e.is_status() => String::from("Something isn't right? Check that you input the MAC address correctly and try again!"),
            e if e.is_timeout() => String::from("It appears the site https://api.macvendors.com/ may be unreachable currently. 
            Try running your previous command again if this error presists wait a little while."),
            e if !e.is_connect() => String::from("We couldn't connect to https://api.macvendors.com/. the site may be down or you may be disconnected from the internet."),
            _ => String::from("An unhandled error has been encoutered please report this at [add github repo] and include what you carried out to reach this point."),
        },
    };
    println!("Vendor: {:#?}", vendor);
}

fn get_nth_arg(n: usize) -> String {
    let args = match std::env::args().nth(n) {
        Some(input) => input,
        None => help(),
    };

    args
}

fn help() -> String {
    format!(
        "\nThis is a CLI tool to quickly look up the vendor associated with the provided MAC Address.\n
            [*] appropriate delimiter formatting of MAC address: '-' '.' ':' or no delimiter at all.
            [*] Examples: 
               
                00-11-22-33-44-55
                00:11:22:33:44:55
                00.11.22.33.44.55
                001122334455
                0011.2233.4455
 "
    )
}

/*
The method below is not the most efficient for multiple requests (a feature to be implemented in the future)
    * reqwests recommends creating a client as to avoid initiating a new connection each request
    * correcting my implementation to use this method would allow a more performant execution when sending lots of requests
    * More research needs to be done on my part on how to implement this
*/
#[tokio::main]
async fn request_and_response(directory: String) -> Result<String, reqwest::Error> {
    let url = format!("{}{}", " https://api.macvendors.com/", directory);
    let response = reqwest::get(url)
    .await?
    .text()
    .await?;

    Ok(response)
}