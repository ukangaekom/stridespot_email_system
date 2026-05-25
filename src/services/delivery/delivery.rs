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
        .body(format!(r#"
        <!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Delivery Confirmed</title>
</head>
<body style="font-family:Arial,sans-serif;background:#f9f9f9;">
  <table align="center" width="100%" style="max-width:600px;background:#fff;border-radius:10px;padding:20px;">
    <tr>
      <td align="center">
        <h2 style="color:#1b73e8;">Delivery Confirmed</h2>
        <p style="color:#555;">Hi {},</p>
        <p style="color:#555;">Your delivery has been confirmed and is on its way.</p>
        <p style="color:#555;">Delivery ID: <strong>{}</strong></p>
        <p style="color:#777;font-size:14px;">Thank you for choosing Stridespot.</p>
      </td>
    </tr>
  </table>
</body>
</html>
        "#, name, delivery_id))
        .unwrap();



    get_mailer().send(email).await?;

    Ok(())
      
}