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

        fn build_marketing_html(name: &str, topic: &str, body: &str, cta_text: &str, cta_link: &str) -> String {
                format!(r#"<!DOCTYPE html>
<html lang=\"en\"> 
<head>
    <meta charset=\"utf-8\"> 
    <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"> 
    <title>{}</title>
    <style>
        /* Mobile-first responsive styles */
        body {{ margin:0; padding:0; background:#f5f7fb; font-family:Inter, Roboto, Helvetica, Arial, sans-serif; color:#333; }}
        .container {{ max-width:700px; margin:24px auto; background:#ffffff; border-radius:12px; overflow:hidden; box-shadow:0 6px 24px rgba(18,38,63,0.08); }}
        .header {{ padding:28px 32px; background:linear-gradient(90deg,#1b73e8 0%,#5ac8fa 100%); color:#fff; text-align:left; }}
        .logo {{ font-weight:700; font-size:20px; letter-spacing:0.3px; }}
        .hero {{ padding:32px; text-align:left; }}
        .title {{ font-size:20px; margin:0 0 8px 0; color:#102a43; }}
        .pre {{ color:#475569; margin:0 0 16px 0; line-height:1.45; }}
        .body {{ color:#334155; font-size:15px; line-height:1.6; }}
        .cta {{ display:inline-block; margin-top:18px; background:#1b73e8; color:#fff; padding:12px 20px; border-radius:8px; text-decoration:none; font-weight:600; }}
        .footer {{ padding:20px 32px; color:#64748b; font-size:13px; background:#fbfdff; text-align:center; }}
        @media (max-width:480px) {{ .header {{ padding:20px; }} .hero {{ padding:20px; }} .container {{ margin:12px; }} }}
    </style>
</head>
<body>
    <div class=\"container\">
        <div class=\"header\">
            <div class=\"logo\">Stridespot</div>
        </div>
        <div class=\"hero\">
            <h1 class=\"title\">{}</h1>
            <p class=\"pre\">Hi {},</p>
            <div class=\"body\">{}</div>
            <a href=\"{}\" class=\"cta\">{}</a>
        </div>
        <div class=\"footer\">
            <div>You're receiving this email because you're subscribed to Stridespot updates.</div>
            <div style=\"margin-top:8px;\">If you'd like to unsubscribe, <a href=\"{{unsubscribe_link}}\" style=\"color:#1b73e8; text-decoration:none;\">click here</a>.</div>
        </div>
    </div>
</body>
</html>"#, topic, topic, name, body, cta_link, cta_text)
        }

        let html = build_marketing_html(name, topic, body, "Learn more", "https://stridespot.example/learn");

        let email = Message::builder()
                .from(get_email().parse().unwrap())
                .to(email.parse().unwrap())
                .subject(topic)
                .header(ContentType::TEXT_HTML)
                .body(html)
                .unwrap();


        get_mailer().send(email).await?;
        Ok(())

}