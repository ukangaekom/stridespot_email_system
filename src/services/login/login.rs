use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;




pub async fn login(name:&str,email:&str,device:&str,location:&str,time:&str) -> Result<(),Box<dyn std::error::Error>>{

    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Logged In")
        .header(ContentType::TEXT_HTML)
        .body(String::from(format!(r#"
        <!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Login Alert - Spotrider</title>
</head>
<body style="font-family:Arial,sans-serif;background:#f9f9f9;">
  <table align="center" width="100%" style="max-width:600px;background:#fff;border-radius:10px;padding:20px;">
    <tr>
      <td align="center">
        <h2 style="color:#1b73e8;">New Login Detected</h2>
        <p style="color:#555;">Hi {}, your account was just logged into from a new device:</p>
        <p style="background:#f1f1f1;padding:10px;border-radius:6px;display:inline-block;">
          Device: {}<br>
          Location: {}<br>
          Time: {}
        </p>
        <p>If this wasn’t you, please <a href="{{reset_link}}" style="color:#1b73e8;">reset your password</a> immediately.</p>
      </td>
    </tr>
  </table>
</body>
</html>  
        "#,name,device,location,time)))
        .unwrap();



    get_mailer().send(email).await?;
    Ok(())
}