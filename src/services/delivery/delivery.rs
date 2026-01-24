use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;
use serde::{Serialize, Deserialize};



// Request body Struct

#[derive(Debug,Serialize,Deserialize)]
pub struct Delivery{

    pub name: String,
    pub email: String,
    pub delivery_id: String

}


pub async fn confirmed_delivery(name:&str,email:&str,delivery_id:&str) -> Result<(),Box<dyn std::error::Error>>{

    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Delivery Confirmed")
        .header(ContentType::TEXT_HTML)
        .body(String::from(format!(r#"<html> <br>Dear Ekom,<br/>
        !</html>"#)))
        .unwrap();



    get_mailer().send(email).await?;

    Ok(())
      
}