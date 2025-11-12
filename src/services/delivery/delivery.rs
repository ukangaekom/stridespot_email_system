use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;




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