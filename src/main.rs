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
mod domains;
mod repository;
use std::thread;
use crate::common::domain::AppContext;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_context = AppContext::new();

    let alerts_db = repository::database::Database::new();


    let app_context_copy = app_context.clone();
    actix_rt::spawn(async move {
        println!("{:?}", app_context_copy.sftp_cron_expression);

        let o: Session = get_tcp_stream_session();
        let sftp = o.sftp().unwrap();
        println!("Connected to SFTP");


        // Prepare the partners' directories
        for partner in &app_context_copy.sftp_client_partners {

            let mut tmp = app_context_copy.sftp_context.remote_base_dir.clone();

            tmp.push_str("/");
            tmp.push_str(partner);
            sftp.mkdir(Path::new(tmp.as_str()), 0o777).ok();

            let mut partner_base_in_dir = tmp.clone();
            partner_base_in_dir.push_str("/");
            partner_base_in_dir.push_str(&app_context_copy.sftp_context.partner_dir_structure.in_dir.clone());
            sftp.mkdir(Path::new(partner_base_in_dir.as_str()), 0o777).ok();


            let mut partner_base_in_dir_work = partner_base_in_dir.clone();
            partner_base_in_dir_work.push_str("/");
            partner_base_in_dir_work.push_str(&app_context_copy.sftp_context.partner_dir_structure.in_dir_work.clone());
            sftp.mkdir(Path::new(partner_base_in_dir_work.as_str()), 0o777).ok();

            let mut partner_base_in_dir_success = partner_base_in_dir.clone();
            partner_base_in_dir_success.push_str("/");
            partner_base_in_dir_success.push_str(&app_context_copy.sftp_context.partner_dir_structure.in_dir_success.clone());
            sftp.mkdir(Path::new(partner_base_in_dir_success.as_str()), 0o777).ok();


            let mut partner_base_in_dir_error = partner_base_in_dir.clone();
            partner_base_in_dir_error.push_str("/");
            partner_base_in_dir_error.push_str(&app_context_copy.sftp_context.partner_dir_structure.in_dir_error.clone());
            sftp.mkdir(Path::new(partner_base_in_dir_error.as_str()), 0o777).ok();


            let mut partner_base_out_dir = tmp.clone();
            partner_base_out_dir.push_str("/");
            partner_base_out_dir.push_str(&app_context_copy.sftp_context.partner_dir_structure.out_dir.clone());
            sftp.mkdir(Path::new(partner_base_out_dir.as_str()), 0o777).ok();

            println!("Configured directory structure for partner {:?}", partner);

        }
        o.disconnect(Some(DisconnectCode::ByApplication), "Done", Option::None).expect("TODO: panic message");
        println!("Disconnected from SFTP");


        println!("Starting polling");

        let expression = app_context_copy.sftp_cron_expression.as_str();
        let schedule = Schedule::from_str(expression).unwrap();
        let offset = Some(FixedOffset::east_opt(0).unwrap()).unwrap();

        loop {
            let mut upcoming = schedule.upcoming(offset).take(1);
            actix_rt::time::sleep(Duration::from_millis(500)).await;
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {

                    // let result = get_ips().await;
                    let result = "KARL";

                    // poll SFTP
                    println!("{:?}",result);

                    let sftp_poller: Session = get_tcp_stream_session();
                    let sftp = sftp_poller.sftp().unwrap();

                    println!("Connected to SFTP");

                    sftp_poller.disconnect(Some(DisconnectCode::ByApplication), "Done", Option::None).expect("TODO: panic message");
                    println!("Disconnected from SFTP");
                }
            }
        }
    });

    println!("{:?}", app_context.sftp_cron_expression);

    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
        .bind((app_context.app_server_host, app_context.app_server_port.parse::<u16>().unwrap()))?
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
