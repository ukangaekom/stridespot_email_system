use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;
use serde::{Serialize, Deserialize};



// Request body Struct

#[derive(Debug,Serialize,Deserialize)]
pub struct Marketing{

    pub name: String,
    pub email: String,
    pub topic: String,
    pub body: String,

}


// let body = format!(r#"<head>
//   <meta charset="UTF-8">
//   <title>Exclusive Offer - Spotrider</title>
// </head>
// <body style="font-family:Arial,sans-serif;background:#f9f9f9;">
//   <table align="center" width="100%" style="max-width:600px;background:#fff;border-radius:10px;padding:20px;">
//     <tr>
//       <td align="center">
//         <h2 style="color:#1b73e8;">🎉 Special Offer Just for You!</h2>
//         <p style="color:#555;">Get <strong>20% off</strong> your next delivery with Spotrider. Use the code below at checkout.</p>
//         <h3 style="background:#1b73e8;color:#fff;padding:10px 20px;border-radius:6px;display:inline-block;">STRIDE20</h3>
//         <a href="{{promo_link}}" style="background:#1b73e8;color:#fff;padding:10px 20px;border-radius:6px;text-decoration:none;display:inline-block;margin-top:10px;">Use Offer</a>
//         <p style="color:#777;font-size:12px;margin-top:20px;">Offer valid until {{expiry_date}}.</p>
//       </td>
//     </tr>
//   </table>
// </body>"#);


pub async fn marketing_updates(name:&str,email:&str,topic:&str,body:&str) -> Result<(),Box<dyn std::error::Error>>{

    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject(topic)
        .header(ContentType::TEXT_HTML)
        .body(String::from(format!(r#"
        <!DOCTYPE html>
<html>
{}
</html>
"#,body)))
        .unwrap();



    get_mailer().send(email).await?;
    Ok(())


}