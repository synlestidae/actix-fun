use lettre::smtp::authentication::Credentials;
use lettre::smtp::extension::ClientId;
use lettre::{ClientSecurity, ClientTlsParameters, Envelope, SendableEmail, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use crate::TxMessage;
use crate::alert_analyser::StateChange;
use log::debug;
use native_tls::TlsConnector;

pub struct AlertEmailer {
    pub from: String,
    pub to: String,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String
}

impl actix::Actor for AlertEmailer {
    type Context = actix::Context<Self>;
}

impl actix::Handler<TxMessage<StateChange>> for AlertEmailer {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<StateChange>, _ctx: &mut Self::Context) -> Self::Result {
        let message_id = format!(
            "alert-email-test:{}",
            msg.id
        );

        debug!("Email parameter (message_id={})", message_id);
        debug!("Email parameter (from={})", self.from);

        let mut email_body = String::new();
        email_body.push_str(&format!("There was a status change. You might want to look into this!
            
            Here is the current status: 
            {}

            This was the previous status:
            {}
        ", msg.msg.previous_status, msg.msg.current_status));

        let mut email_builder = EmailBuilder::new()
            .from(self.from)
            .subject("Something changed!")
            .body(email_body);

        email_builder = email_builder.to(self.to); // TODO This may fail if formatted badly

        let connector = match TlsConnector::new() {
            Ok(c) => c,
            Err(err) => {
                error!("Error with TLS connector: {}", err);
                return;
            }
        };
        let client = SmtpClient::new(
            self.hostname,
            ClientSecurity::Required(ClientTlsParameters {
                connector,
                domain: self.host.clone(),
            }),
        );

        debug!("Using credentials (username={})", self.username);

        let creds = Credentials::new(self.username, self.password);

        let mut transport = client
            .credentials(creds)
            .hello_name(ClientId::Domain(host))
            .transport();

        debug!("Sending email");

        let email = match email_builder.build() {
            Ok(e) => e,
            Err(err) => {
                error!("Error building email: {}", err);
                return Err(Error::from(err));
            }
        };

        match transport.send(email.into()) {
            Ok(_) => info!("Email sent successfully!"),
            Err(e) => {
                error!("Failed to send email: {:?}", e);
                return Err(Error::from(e));
            }
        };

        Ok(())
    }
}

