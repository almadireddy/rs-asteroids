extern crate actix_web;
extern crate actix_files;
extern crate listenfd;
extern crate serde_json;
extern crate serde;
extern crate rand;
use chrono::prelude::*;

#[macro_use]
extern crate log;

use actix_files as fs;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, middleware, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use listenfd::ListenFd;
use byteorder::{ByteOrder, LittleEndian};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use json::JsonValue;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use chrono::prelude::*;

use asteroids::{Player, motion::{
    Placement
}, geometry::{
    Point
}, Game};
use std::sync::Mutex;
use std::collections::HashMap;

struct ServerPlayer {
    player: Player,
}

impl Actor for ServerPlayer {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ServerPlayer {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => {
                let x = LittleEndian::read_f64(&bin.slice(0..8));
                println!("Received x {:?}", x);
                let y = LittleEndian::read_f64(&bin.slice(8..16));
                println!("Received y {:?}", y);
                let r = LittleEndian::read_f64(&bin.slice(16..24));
                println!("Received r {:?}", r);

                self.player.placement.position = Point::new(x, y);
                self.player.placement.rotation = r;
                ctx.text("Got coordinates")
            } ,
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(ServerPlayer { player: Player::new(Point::new(0.0, 0.0)) }, &req, stream);
    println!("{:?}", resp);
    resp
}

#[derive(Debug, Serialize, Deserialize)]
struct NewGameResponse {
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Room {
    code: String,
    created_at: chrono::DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentRooms {
    games: Mutex<std::collections::HashMap<String, Room>>
}

async fn new_game(req: HttpRequest, data: web::Data<CurrentRooms>) -> Result<HttpResponse, Error> {
    let mut rng = thread_rng();
    let chars: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(4)
        .collect();
    let chars: String = chars.to_uppercase();
    let mut games = data.games.lock().unwrap();

    let new_room = Room{code: chars.clone(), created_at: Utc::now() };
    let resp = HttpResponse::Ok().json(&new_room);
    games.insert(chars.clone(), new_room);

    println!("current rooms: {:?}", games);

    Ok(resp)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Watching on 127.0.0.1:8088");
    let map: HashMap<String, Room> = HashMap::new();
    let rooms = web::Data::new(CurrentRooms {
        games: Mutex::new(map)
    });

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(rooms.clone())
            .wrap(middleware::Logger::new("%a | %s %r | %Dms"))
            .route("/game/", web::get().to(index))
            .route("/game/new", web::post().to(new_game))
            .service(fs::Files::new("/", "./app/www/").index_file("index.html"))
    });
    
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
