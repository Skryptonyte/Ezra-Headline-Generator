use std::*;
use std::fs::File;
use std::io::BufReader;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::prelude::*;
use rand::Rng;

struct Crimes
{
    crime_vec: Vec<String>,
    state_vec: Vec<String>
}
#[get("/")]
async fn test(crimes: web::Data<Crimes>) -> impl Responder
{
    let crimes_len = crimes.crime_vec.len();
  
    let mut crime_rng = rand::thread_rng().gen_range(0..crimes_len);
    let mut state_rng = rand::thread_rng().gen_range(0..50);
    return HttpResponse::Ok().body(
    format!("
        <html>
            <body>
            <h1> Breaking News: Flash star Ezra Miller has been charged with {} in the state of {}.</h1>
            </body>
        </html>
        ", crimes.crime_vec[crime_rng], crimes.state_vec[state_rng]));
}
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let mut crimes =Crimes {crime_vec: Vec::new(), state_vec: Vec::new()};

    let f = File::open("crimes.txt").unwrap();
    let buf = BufReader::new(f);

    for l in buf.lines()
    {
        crimes.crime_vec.push(l.unwrap());
    }

    let f2 = File::open("states.txt").unwrap();
    let buf = BufReader::new(f2);

    for l in buf.lines()
    {
        crimes.state_vec.push(l.unwrap());
    }

    let mut crime_web_data = web::Data::new(crimes);
    HttpServer::new(move ||{
        App::new()
        .app_data(crime_web_data.clone())
        .service(test)
    })
    .bind(("0.0.0.0",8000))?
    .run()
    .await
}
