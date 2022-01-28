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
        let subject = "test_email";
        let message = "fun_email";
        let email_body = vec![
          "Content-Type: text/plain\r\n",
          "MIME-Version: 1.0\r\n",
          &format!("From: {}\r\n", self.from),
          &format!("To: {}\r\n", self.to),
          &format!("Subject: {}\r\n\r\n", subject),
          &format!("{}\r\n\r\n", message),
        ];
        /*
         *
        *  'Content-Type: multipart/mixed; boundary="foo_bar_baz"\r\n',
  'MIME-Version: 1.0\r\n',
  'From: sender@gmail.com\r\n',
  'To: receiver@gmail.com\r\n',
  'Subject: Subject Text\r\n\r\n',

  '--foo_bar_baz\r\n',
  'Content-Type: text/plain; charset="UTF-8"\r\n',
  'MIME-Version: 1.0\r\n',
  'Content-Transfer-Encoding: 7bit\r\n\r\n',

  'The actual message text goes here\r\n\r\n',

  '--foo_bar_baz\r\n',
  'Content-Type: image/png\r\n',
  'MIME-Version: 1.0\r\n',
  'Content-Transfer-Encoding: base64\r\n',
  'Content-Disposition: attachment; filename="example.png"\r\n\r\n',

   pngData, '\r\n\r\n',

   '--foo_bar_baz--' 
         var response = UrlFetchApp.fetch(
        "https://www.googleapis.com/upload/gmail/v1/users/me/messages/send?uploadType=media", {
            method: "POST",
            headers: {
                "Authorization": "Bearer " + ScriptApp.getOAuthToken(),
                "Content-Type": "message/rfc822",
            },
            muteHttpExceptions: true,
            payload: mail
         * */

        Ok(())
    }
}

