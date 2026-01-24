mod auth_credentials;
mod services;

use services::{
    delivery::delivery::{confirmed_delivery, Delivery},
    login::login::login as other_login,
    marketing::marketing::marketing_updates,
    otp::otp::otp_code,
    payment::payment::payment_deposited,
    registration::registration::registration as other_registration,
    resetpassword::resetpassword::resetpassword as other_resetpassword,
    updates::update::feature_updates

};
use derive_more::{Display, Error};
use log::info;
use ntex::web;
use std::env;
use num_cpus;
use std::error::Error;
use ntex::web::types::Json;
use serde::Deserialize;


// delivery endpoint
#[web::post("/delivery")]
async fn delivery(req_body:Json<Delivery>) -> impl web::Responder {
    confirmed_delivery(&req_body.name,&req_body.email,&req_body.delivery_id).await.ok();
    web::HttpResponse::Ok()
     
}

// login endpoint
#[web::post("/login")]
async fn login(req_body: String) -> impl web::Responder {
    other_login("Ekomabasi","ekomabasiuk@gmail.com","Android","Uyo","12:00 UTC").await.ok();
    web::HttpResponse::Ok().body(req_body)
}

// marketingendpoint 
#[web::post("/marketing")]
async fn marketing(req_body: String) -> impl web::Responder {
    marketing_updates("Ekomabasi","ekomabasiuk@gmail.com","Bonanza","We Updated the APP-> Body",).await.ok();
    web::HttpResponse::Ok().body(req_body)
}

// otp endpoint
#[web::post("/otp")]
async fn otp(req_body: String) -> impl web::Responder {
    
    let otp_request = otp_code("Ekomabasi","ekomabasiuk@gmail.com").await.ok();

    let req_response = format!("{{\"OTP\":\"{}\"}}",otp_request.unwrap());
    


     web::HttpResponse::Ok().body(req_response)
    
}

// payment endpoint
#[web::post("/payment")]
async fn payment(req_body: String) -> impl web::Responder {
    payment_deposited("Ekomabasi","ekomabasiuk@gmail.com","sender_name","121333","#DL829230").await.ok();
    web::HttpResponse::Ok().body(req_body)
}

// registration endpoint
#[web::post("/registration")]
async fn registration(req_body: String) -> impl web::Responder {
    other_registration("Ekomabasi","ekomabasiuk@gmail.com").await.ok();
    web::HttpResponse::Ok().body(req_body)
}


// resetpassword endpoint
#[web::post("/resetpassword")]
async fn resetpassword(req_body: String) -> impl web::Responder {
    let resetpassword = other_resetpassword("Ekomabasi","ekomabasiuk@gmail.com",).await.ok();
    let req_response = format!("{{\"tempoary_password\":\"{}\"}}",resetpassword.unwrap());
    web::HttpResponse::Ok().body(req_response)
}

// project updates endpoint
#[web::post("/updates")]
async fn updates(req_body: String) -> impl web::Responder {
    feature_updates("Ekomabasi","ekomabasiuk@gmail.com","Update Topic", "Update Message","https://github.com/ukangaekom").await.ok();
    web::HttpResponse::Ok().body(req_body)
}



// JWT Middleware



#[rustfmt::skip]
#[ntex::main]
async fn main() -> std::io::Result<()> {

    dotenv_flow::dotenv_flow().ok();
    

    web::HttpServer::new(|| {
        web::App::new()
            .service(delivery)
            .service(login)
            .service(marketing)
            .service(otp)
            .service(payment)
            .service(registration)
            .service(resetpassword)
            .service(updates)
    })
    .workers(num_cpus::get())
    .backlog(4096)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
