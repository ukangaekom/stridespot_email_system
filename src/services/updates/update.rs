use crate::auth_credentials::credentials::{get_mailer, get_email};
use lettre::message::{
    header::ContentType, Message
};
use lettre::AsyncTransport;



use serde::{Serialize, Deserialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct FeatureUpdate{

    pub name: String,
    pub email: String,
    pub feature_topic:String,
    pub body: String,
    pub site_link: String

}



// let body:&str = "<ul style=\"text-align:left;color:#555;\">
//           <li>🚀 Faster delivery tracking</li>
//           <li>🔒 Enhanced security</li>
//           <li>🗺️ Real-time driver map view</li>
//         </ul>"


pub async fn feature_updates(name:&str,email:&str, feature_topic:&str, body:&str, site_link:&str) -> Result<(),Box<dyn std::error::Error>>{

    let email = Message::builder()
        .from(get_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject(feature_topic)
        .header(ContentType::TEXT_HTML)
        .body(format!(r#"
        <!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Spotrider Updates</title>
</head>
<body style="font-family:Arial,sans-serif;background:#f9f9f9;">
  <table align="center" width="100%" style="max-width:600px;background:#fff;border-radius:10px;padding:20px;">
    <tr>
      <td align="center">
        <h2 style="color:#1b73e8;">New Features & Updates!</h2>
        <p style="color:#555;">We’ve made some exciting improvements to Spotrider on {}</p>

        {}
        
        <a href="{}" style="background:#1b73e8;color:#fff;padding:10px 20px;border-radius:6px;text-decoration:none;">Update</a>
        <p style="color:#777;font-size:12px;margin-top:20px;">Keep your app updated to enjoy these features.</p>
      </td>
    </tr>
  </table>
</body>
</html>

        "#,feature_topic, body, site_link))
        .unwrap();


    get_mailer().send(email).await?;
    Ok(())
}