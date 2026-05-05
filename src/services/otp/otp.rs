use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;
use rand::Rng;
use serde::{Serialize, Deserialize};



// Request body Struct

#[derive(Debug,Serialize,Deserialize)]
pub struct Otp{

    pub name: String,
    pub email: String,

}





pub async fn otp_code(name:&str,email:&str) -> Result<u32,Box<dyn std::error::Error>> {

    let mut otp_gen = rand::thread_rng();

    let otp: u32 = otp_gen.gen_range(100_000..999_999);

    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verification OTP")
        .header(ContentType::TEXT_HTML)
        .body(String::from(format!(r#"
        <!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>OTP Verification - Spotrider</title>
</head>
<body style="font-family:Arial,sans-serif;background:#f9f9f9;">
  <table align="center" width="100%" style="max-width:600px;background:#fff;border-radius:10px;padding:20px;">
    <tr>
      <td align="center">
        <h2 style="color:#1b73e8;">Your Spotrider Verification Code</h2>
        <p style="color:#555;">Use the OTP below to verify your account:</p>
        <h1 style="letter-spacing:4px;color:#1b73e8;">{}</h1>
        <p style="color:#777;font-size:12px;">This code expires in 5 minutes.</p>
      </td>
    </tr>
  </table>
</body>
</html>"#,otp)))
        .unwrap();



    get_mailer().send(email).await.ok();
    Ok(otp)
}