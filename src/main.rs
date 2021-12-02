#[macro_use] extern crate rocket;

use rocket::fs::relative;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::fs::FileServer;
// use rocket::serde::{Deserialize, json::Json};

#[get("/asset-manifest.json")]
async fn asset_manifest() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/asset-manifest.json")).await.ok()
}
#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/favicon.ico")).await.ok()
}
#[get("/logo192.png")]
async fn logo192() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/logo192.png")).await.ok()
}
#[get("/logo512.png")]
async fn logo512() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/logo512.png")).await.ok()
}
#[get("/manifest.json")]
async fn manifest() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/manifest.json")).await.ok()
}
#[get("/robots.txt")]
async fn robots() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/robots.txt")).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/index.html")).await.ok()
}

#[get("/<_>")]
async fn any_index() -> Option<NamedFile> {
    return NamedFile::open(Path::new("./Cello/build/index.html")).await.ok()
}

//TODO
#[get("/boards/<board_id>")]
async fn get_board(board_id: &str) -> &'static str {
    "get board"
}

//TODO
// #[derive(Deserialize)]
struct Action<'r> {
    action: &'r str,
    payload: bool //TODO fix to any object (maybe?)
}

//TODO
// #[post("/boards/<board_id>", data="<action>")]
// async fn append_board(board_id: &str, action: Json<Action<'_>>) -> &'static str {
    // "post board";
// }


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, any_index, asset_manifest, favicon, logo192, logo512, manifest, robots])
        .mount("/api", routes![get_board])
        .mount("/static", FileServer::from(relative!("./Cello/build/static")))
}


//for all routes, serve /Cello/build/index.html
//EXCEPT:
// /asset-manifest.json    -------------
// /favicon.ico            -------------
// /logo192.png            -------------
// /logo512.png            -------------
// /manifest.json          -------------
// /static/*               -------------
// /robots.txt             -------------
//Which are all forwarded to the file system. 
//
//POST /api/boards/[board_id]
// - If there isn't a file, create it. (enforce max file size & name safety constraints)
// - If there is a file, open it.
// - Append whatever you find to the file, recording current timestamp
//GET: /api/boards/[board_id]?since=[last sync timestamp]
// - If there isn't a file, return an empty JSON array. 
// - If there is a file, open it.
// - If there isn't a since time, set it to unix epoch.
// - Seek forward to the first timestamp after the since timestamp.
// - Collect every JSON object (could be none), package it into an array, send back.