use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvAttributes {
    pub sftp_client_poll_cron_exp: String,
    pub sftp_server_endpoint_url: String,
    pub sftp_server_basedir: String,
    pub sftp_server_in_dir: String,
    pub sftp_server_in_dir_work: String,
    pub sftp_server_in_dir_error: String,
    pub sftp_server_in_dir_success: String,
    pub sftp_server_out_dir: String,
    pub sftp_server_username: String,
    pub sftp_server_password: String,
    pub sftp_client_partners_list: String
}

impl EnvAttributes {
    pub fn new() -> EnvAttributes {

        let sftp_poll_cron_expr = std::env::var("SFTP_CLIENT_POLL_CRON_EXPR").expect("SFTP_CLIENT_POLL_CRON_EXPR must be set.");


        EnvAttributes {
            sftp_client_poll_cron_exp: sftp_poll_cron_expr.clone(),
            sftp_server_endpoint_url: "".to_string(),
            sftp_server_basedir: "".to_string(),
            sftp_server_in_dir: "".to_string(),
            sftp_server_in_dir_work: "".to_string(),
            sftp_server_in_dir_error: "".to_string(),
            sftp_server_in_dir_success: "".to_string(),
            sftp_server_out_dir: "".to_string(),
            sftp_server_username: "".to_string(),
            sftp_server_password: "".to_string(),
            sftp_client_partners_list: "".to_string(),
        }
    }
}