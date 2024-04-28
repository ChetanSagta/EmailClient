use std::net::TcpStream;

const GOOGLE_SMTP_URL: &str = "stmp.gmail.com:587";
const GOOGLE_IMAP_URL: &str = "imap.gmail.com:993";

enum Commands {
    AUTH,
    DATA,
    RCPT,
}

trait Auth {
    fn authenticate(username: &str, password: &str);
}

struct StateTable{}

struct Lines{

    data: String,
    end:String //"CRLF"
}

struct Envelope {
    //series of SMTP protocol units
    originator_address: String,
    recipients: Vec<String>,

}

struct Content {
    header: String,
    body: String,
}

struct Mail {
    envelope: Envelope,
    content: Content,
}

struct SMTPClient{
}

struct IMAPClient{
   tcp_stream:TcpStream
}

impl IMAPClient{

    fn initialise(){

    }

}

enum ErrorCode{
    SmtpError= 554,
    Ok= 200
}

enum IMAPStates{
    NotConnected,
    ConnectionEstablished, 
    NotAuthenticated,
    Authenticated,
    Selected,
    Logout
}

fn main() {
    let client = IMAPClient{tcp_stream: TcpStream::connect(GOOGLE_IMAP_URL).unwrap()};

}
