mod auth_credentials;
mod services;

use services::{
    delivery::delivery::{confirmed_delivery, Delivery},
    login::login::{login as other_login, Login},
    marketing::marketing::{marketing_updates, Marketing},
    otp::otp::{otp_code, Otp},
    payment::payment::{payment_deposited, PaymentDeposited},
    registration::registration::{registration as other_registration, Registration},
    resetpassword::resetpassword::{resetpassword as other_resetpassword, ResetPassword},
    updates::update::{feature_updates, FeatureUpdate}

};
use derive_more::{Display, Error};
use log::info;
use ntex::web;
use std::env;
use num_cpus;
use std::error::Error;
use ntex::web::types::Json;


// delivery endpoint
#[web::post("/delivery")]
async fn delivery(req_body:Json<Delivery>) -> impl web::Responder {
    confirmed_delivery(&req_body.name,&req_body.email,&req_body.delivery_id).await.ok();
    web::HttpResponse::Ok()
     
}

// login endpoint
#[web::post("/login")]
async fn login(req_body: Json<Login>) -> impl web::Responder {
    other_login(&req_body.name, &req_body.email, &req_body.device, &req_body.location, &req_body.time).await.ok();
    web::HttpResponse::Ok()
}

// marketingendpoint 
#[web::post("/marketing")]
async fn marketing(req_body: Json<Marketing>) -> impl web::Responder {
    marketing_updates(&req_body.name, &req_body.email, &req_body.topic, &req_body.body, &req_body.link).await.ok();
    web::HttpResponse::Ok()
}

// otp endpoint
#[web::post("/otp")]
async fn otp(req_body: Json<Otp>) -> impl web::Responder {
    
    let otp_request = otp_code(&req_body.name, &req_body.email).await.ok();

    let req_response = format!("{{\"OTP\":\"{}\"}}",otp_request.unwrap());
    


     web::HttpResponse::Ok().body(req_response)
    
}

// payment endpoint
#[web::post("/payment")]
async fn payment(req_body: Json<PaymentDeposited>) -> impl web::Responder {
    payment_deposited(&req_body.name, &req_body.email, &req_body.sender_name, &req_body.amount, &req_body.order_id).await.ok();
    web::HttpResponse::Ok()
}

// registration endpoint
#[web::post("/registration")]
async fn registration(req_body: Json<Registration>) -> impl web::Responder {
    other_registration(&req_body.name, &req_body.email).await.ok();
    web::HttpResponse::Ok()
}


// resetpassword endpoint
#[web::post("/resetpassword")]
async fn resetpassword(req_body: Json<ResetPassword>) -> impl web::Responder {
    let resetpassword = other_resetpassword(&req_body.name, &req_body.email).await.ok();
    let req_response = format!("{{\"tempoary_password\":\"{}\"}}",resetpassword.unwrap());
    web::HttpResponse::Ok().body(req_response)
}

// project updates endpoint
#[web::post("/updates")]
async fn updates(req_body: Json<FeatureUpdate>) -> impl web::Responder {
    feature_updates(&req_body.name, &req_body.email, &req_body.feature_topic, &req_body.body, &req_body.site_link).await.ok();
    web::HttpResponse::Ok()
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
