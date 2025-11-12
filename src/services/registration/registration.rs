use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;




pub async fn registration(name:&str,email:&str) -> Result<(),Box<dyn std::error::Error>>{

    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Successful Registration")
        .header(ContentType::TEXT_HTML)
        .body(String::from(format!(r#"
        <!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Welcome to Spotrider</title>
</head>
<body style="font-family:Arial,sans-serif;background:#f9f9f9;">
  <table align="center" width="100%" style="max-width:600px;background:#fff;border-radius:10px;padding:20px;">
    <tr>
      <td align="center">
        <h2 style="color:#1b73e8;">Welcome to Spotrider!</h2>
        <p style="color:#555;">Hi {}, your account has been successfully created. Start scheduling and tracking your deliveries today.</p>
        <a href="{{login_link}}" style="background:#1b73e8;color:#fff;padding:10px 20px;border-radius:6px;text-decoration:none;">Go to Dashboard</a>
        <p style="color:#777;font-size:12px;margin-top:20px;">Need help? <a href="{{support_link}}" style="color:#1b73e8;">Contact Support</a></p>
      </td>
    </tr>
  </table>
</body>
</html>

        "#,name)))
        .unwrap();



   get_mailer().send(email).await?;
   Ok(())

}