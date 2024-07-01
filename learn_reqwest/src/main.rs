use error_chain::error_chain;
use std::io::Read;
use std::time::Instant;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn make_blocking_request() -> Result<()> {
    let block_start = Instant::now();
    let mut res = reqwest::blocking::get("https://httpbin.org/get").expect("Unable to get response");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Unable to parse response body");

    println!("Status {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    println!("{:?}", block_start.elapsed());

    Ok(())
}

async fn make_async_request() -> Result<()> {
    let async_start = Instant::now();
    let res = reqwest::get("http://httpbin.org/get").await.expect("Unable to get response"); 

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await.expect("Unable to read the response body");
    println!("Body:\n{}", body);
    println!("{:?}", async_start.elapsed());

    Ok(())
}

fn main() -> Result<()> {
    if let Err(ref e) = make_blocking_request() {
        eprintln!("Error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("Caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("Backtrace: {:?}", backtrace);
        }
    }

    Ok(())
}

//#[tokio::main]
//async fn main() {
    // making a blocking request
    //if let Err(ref e) = make_blocking_request() {
    //    eprintln!("Error: {}", e);
    //
    //    for e in e.iter().skip(1) {
    //        eprintln!("Caused by: {}", e);
    //    }
    //
    //    if let Some(backtrace) = e.backtrace() {
    //        eprintln!("Backtrace: {:?}", backtrace);
    //    }
    //}

    // making a non-blocking(async) request
//    if let Err(ref e) = make_async_request().await {
//        eprintln!("Error: {}", e);       
//    }
//}
