use std::io::Write;
use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;


fn main() {

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
