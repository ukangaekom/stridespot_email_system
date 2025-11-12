use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;



pub async fn payment_deposited(name:&str,email:&str,sender_name:&str, amount: &str,order_id:&str) -> Result<(),Box<dyn std::error::Error>>{

    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Deposit Confirmation")
        .header(ContentType::TEXT_HTML)
        .body(String::from(format!(r#"
        <!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Payment Successful - Spotrider</title>
</head>
<body style="font-family:Arial,sans-serif;background:#f9f9f9;">
  <table align="center" width="100%" style="max-width:600px;background:#fff;border-radius:10px;padding:20px;">
    <tr>
      <td align="center">
        <h2 style="color:#1b73e8;">Payment Successful</h2>
        <p style="color:#555;">Hi {}, your payment of <strong>{}</strong> has been successfully processed for the deliery id <strong>#{}</strong>.</p>
        <a href="{{order_link}}" style="background:#1b73e8;color:#fff;padding:10px 20px;border-radius:6px;text-decoration:none;">View Order</a>
        <p style="color:#777;font-size:12px;margin-top:20px;">Thank you for choosing Stridespot!</p>
      </td>
    </tr>
  </table>
</body>
</html>
"#,name,amount,order_id)))
        .unwrap();


    get_mailer().send(email).await?;
    Ok(())

} 


pub async fn payment_withdraw(name:&str,email:&str,amount: &str) -> Result<(),Box<dyn std::error::Error>>{
    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Withdrawal Confirmation")
        .body(String::from(format!(r#"<html>This is a test email! Please don't reply!</html>"#)))
        .unwrap();

    
    get_mailer().send(email).await?;
    Ok(())

}