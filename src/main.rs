use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
