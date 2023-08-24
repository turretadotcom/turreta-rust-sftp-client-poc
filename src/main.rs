use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;
use chrono::{Local, FixedOffset};
use actix_web::{web, App, HttpServer};
use std::{collections::HashMap, time::Duration};
use std::str::FromStr;
use cron::Schedule;
use dotenv::dotenv;
use regex::Regex;
use std::thread;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    actix_rt::spawn(async move {

        let sftp_poll_cron_expr = std::env::var("SFTP_POLL_CRON_EXPR").expect("SFTP_POLL_CRON_EXPR must be set.");
        let partners_list = std::env::var("PARTNERS_LIST").expect("PARTNERS_LIST must be set.");

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
        }
    });
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
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
