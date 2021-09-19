use tide::Request;
use tide::Response;
use sqlite::Connection;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/status").get(status);
    app.at("/status/:location").get(status);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn status(req: Request<()>) -> tide::Result{
    let param = req.param("location");
    match param {
        Ok(location) => {
            if let Some(url) = req.host(){
                Connection::open("./ips.db")
                .expect("Cannot open database")
                .execute(
                    format!(
                    "CREATE TABLE IF NOT EXISTS addresses(location TEXT NOT NULL, url TEXT NOT NULL);
                    INSERT INTO addresses VALUES('{}','{}');
                    
                    ",location, url))
                    .expect("Failed to write values to table");
            }
        }
        Err(_e) => return Ok(Response::new(tide::StatusCode::Ok))
    }
    Ok(Response::new(tide::StatusCode::Ok))

}