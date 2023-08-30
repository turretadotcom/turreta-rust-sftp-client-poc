use std::net::TcpStream;
use ssh2::{DisconnectCode, Session, Sftp};
use std::path::Path;
use chrono::{Local, FixedOffset};
use actix_web::{web, App, HttpServer};
use std::{collections::HashMap, time::Duration};
use std::str::FromStr;
use cron::Schedule;
use dotenv::dotenv;
use regex::Regex;
pub mod common;
use std::thread;
use crate::common::domain::AppContext;
use crate::common::util::EnvAttributes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let envs = EnvAttributes::new();

    let envs2 = envs.clone();
    actix_rt::spawn(async move {

        println!("{:?}", envs2.sftp_client_poll_cron_exp);

        let sftp_poll_cron_expr = std::env::var("SFTP_CLIENT_POLL_CRON_EXPR").expect("SFTP_CLIENT_POLL_CRON_EXPR must be set.");
        let partners_list = std::env::var("SFTP_CLIENT_PARTNERS_LIST").expect("SFTP_CLIENT_PARTNERS_LIST must be set.");


        let o = AppContext::new();

        let p =Regex::new(r"[,\s]");
        let q = p.unwrap();
        let o = q.split(partners_list.as_str());
        let y:Vec<_> = o.into_iter().collect();



        print!("{:?}", y);
        // Pred partners SFTP dirs



        /*

        /base-dir/
            - P1
              - from_client
                - in
                - error
                - wip
                - done
              - to_client
                - file1
            - P2
              - from_client
                - in
                - error
                - wip
                - done
              - to_client
                - file1
            - P3
              - from_client
                - in
                - error
                - wip
                - done
              - to_client
                - file1
        */

        let expression = sftp_poll_cron_expr.as_str();
        let schedule = Schedule::from_str(expression).unwrap();
        let offset = Some(FixedOffset::east_opt(0).unwrap()).unwrap();

        loop {

            let o: Session = get_tcp_stream_session();
            let sftp = o.sftp().unwrap();

            println!("Connected to SFTP");

            let mut upcoming = schedule.upcoming(offset).take(1);
            actix_rt::time::sleep(Duration::from_millis(500)).await;
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {

                    // let result = get_ips().await;
                    let result = "KARL";

                    // poll SFTP
                    println!("{:?}",result);
                }
            }

            o.disconnect(Some(DisconnectCode::ByApplication), "Done", Option::None).expect("TODO: panic message");
            println!("Disconnected from SFTP");
        }
    });

    println!("{:?}", envs.sftp_client_poll_cron_exp);

    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn get_tcp_stream_session() -> Session {

    let sftp_endpoint_url = std::env::var("SFTP_SERVER_ENDPOINT_URL").expect("SFTP_SERVER_ENDPOINT_URL must be set.");
    let sftp_username = std::env::var("SFTP_SERVER_USERNAME").expect("SFTP_SERVER_USERNAME must be set.");
    let sftp_user_password = std::env::var("SFTP_SERVER_USERPWD").expect("SFTP_SERVER_USERPWD must be set.");

    let tcp = TcpStream::connect(sftp_endpoint_url.as_str()).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(sftp_username.as_str(), sftp_user_password.as_str()).unwrap();

    sess
}


fn prepare_dir_structure_when_required() -> Session {

    let sftp_endpoint_url = std::env::var("SFTP_SERVER_ENDPOINT_URL").expect("SFTP_SERVER_ENDPOINT_URL must be set.");
    let sftp_username = std::env::var("SFTP_SERVER_USERNAME").expect("SFTP_SERVER_USERNAME must be set.");
    let sftp_user_password = std::env::var("SFTP_SERVER_USERPWD").expect("SFTP_SERVER_USERPWD must be set.");

    let tcp = TcpStream::connect(sftp_endpoint_url.as_str()).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(sftp_username.as_str(), sftp_user_password.as_str()).unwrap();

    sess
}

fn get_sftp_client_instance() -> Sftp {

    let sftp_endpoint_url = std::env::var("SFTP_SERVER_ENDPOINT_URL").expect("SFTP_SERVER_ENDPOINT_URL must be set.");
    let sftp_username = std::env::var("SFTP_SERVER_USERNAME").expect("SFTP_SERVER_USERNAME must be set.");
    let sftp_user_password = std::env::var("SFTP_SERVER_USERPWD").expect("SFTP_SERVER_USERPWD must be set.");

    let tcp = TcpStream::connect(sftp_endpoint_url.as_str()).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(sftp_username.as_str(), sftp_user_password.as_str()).unwrap();

    let sftp = sess.sftp().unwrap();

    sftp
}
fn main_temp() {

    let tcp = TcpStream::connect("127.0.0.1:9922").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("foo", "pass").unwrap();

    let sftp = sess.sftp().unwrap();

    sftp.mkdir(Path::new("/foo-home"), 0o777).ok();
    // sftp.create(&Path::new("/foo-home/file.json"))
    //     .unwrap()
    //     .write_all("text to be written to file".as_bytes())
    //     .unwrap();

    let p = Path::new("/foo-home");

    let pp = sftp.readdir(p).unwrap();

    for i in &pp {

        let (a, b) = i;
        let pppp = a.display().to_string();

        let ff = Path::new(&pppp);

        let ff2 = Path::new("/foo-home/file2.json");
        sftp.rename(ff, ff2, None).unwrap();
    }


}
