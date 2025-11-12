use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;
use rand::{distributions::Alphanumeric, Rng};


// const CHARSET64: &[u8, 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\ abcdefghijklmnopqrstuvwxyz\ 01234567890-_!@#$%^&*()+";

pub async fn resetpassword(name:&str,email:&str) -> Result<String,Box<dyn std::error::Error>>{

    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    
    
    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Password Reset")
        .header(ContentType::TEXT_HTML)
        .body(String::from(format!(r#" <!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Change Password - Spotrider</title>
</head>
<body style="font-family:Arial,sans-serif;background:#f9f9f9;padding:0;margin:0;">
  <table align="center" width="100%" style="max-width:600px;background:#ffffff;border-radius:10px;padding:20px;">
    <tr>
      <td align="center">
        <h2 style="color:#1b73e8;">Spotrider</h2>
        <h3>Change Your Password</h3>
        <p style="color:#555;">We received a request to change your password. Below is your tempoary password that will expire in 30 minutes. Kindly do well to change your password in the app when you login.</p>
        <h3 href="{{reset_link}}" style="background:#1b73e8;color:#fff;text-decoration:none;padding:10px 20px;border-radius:6px;display:inline-block;">{}</h3>
        <p style="color:#777;font-size:12px;margin-top:20px;">If you didn’t request this change, you can safely ignore this email.</p>
      </td>
    </tr>
  </table>
</body>
</html>
"#, password)))
        .unwrap();


    get_mailer().send(email).await.ok();
    Ok(password)

}