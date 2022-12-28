use lettre::message::{header::ContentType, Attachment, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::{env, fs, path::Path};

struct Config {
    from_address: String,
    from_name: String,
    recipients: String,
    subject: String,
    body: String,
    body_is_html: String,
    user_name: String,
    password: String,
    host: String,
    port: String,
    use_starttls: String,
    attachment_path: String,
    attachment_type: String,
}

fn main() {
    // Parse environment variables
    let config = Config {
        from_address: env::var("MAILER_FROM_ADDRESS").unwrap_or(String::from("")),
        from_name: env::var("MAILER_FROM_NAME").unwrap_or(String::from("")),
        recipients: env::var("MAILER_RECIPIENTS").unwrap_or(String::from("")),
        subject: env::var("MAILER_SUBJECT").unwrap_or(String::from("")),
        body: env::var("MAILER_BODY").unwrap_or(String::from("")),
        body_is_html: env::var("MAILER_BODY_IS_HTML").unwrap_or(String::from("false")),
        attachment_path: env::var("MAILER_ATTACHMENT_PATH").unwrap_or(String::from("")),
        attachment_type: env::var("MAILER_ATTACHMENT_TYPE").unwrap_or(String::from("text/plain")),
        user_name: env::var("MAILER_USER_NAME").unwrap_or(String::from("")),
        password: env::var("MAILER_PASSWORD").unwrap_or(String::from("")),
        host: env::var("MAILER_HOST").unwrap_or(String::from("")),
        port: env::var("MAILER_PORT").unwrap_or(String::from("465")),
        use_starttls: env::var("MAILER_USE_STARTTLS").unwrap_or(String::from("false")),
    };

    // Initialize the builder
    let mut msg_builder = Message::builder()
        .from(
            format!("{} <{}>", config.from_name, config.from_address)
                .parse()
                .unwrap(),
        )
        .subject(config.subject);
    for recipient in config.recipients.split(',') {
        msg_builder = msg_builder.to(format!("<{}>", recipient).parse().unwrap());
    }
    // Add body
    let mut body: MultiPart;
    if config.body_is_html == "true" {
        body = MultiPart::mixed().singlepart(SinglePart::html(
            config.body.replace("\\n", "\n").replace("\\t", "\t"),
        ));
    } else {
        body = MultiPart::mixed().singlepart(SinglePart::plain(
            config.body.replace("\\n", "\n").replace("\\t", "\t"),
        ));
    }
    // Attach files
    if !config.attachment_path.is_empty() {
        body = body.singlepart(
            Attachment::new(String::from(
                Path::new(&config.attachment_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ))
            .body(
                fs::read(config.attachment_path).unwrap(),
                ContentType::parse(config.attachment_type.as_str()).unwrap(),
            ),
        );
    }
    // Build the message
    let msg = msg_builder.multipart(body).unwrap();

    // Send
    if config.use_starttls == "true" {
        match SmtpTransport::starttls_relay(config.host.as_str())
            .unwrap()
            .credentials(Credentials::new(config.user_name, config.password))
            .port(config.port.parse::<u16>().unwrap())
            .build()
            .send(&msg)
        {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    } else {
        match SmtpTransport::relay(config.host.as_str())
            .unwrap()
            .credentials(Credentials::new(config.user_name, config.password))
            .port(config.port.parse::<u16>().unwrap())
            .build()
            .send(&msg)
        {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
