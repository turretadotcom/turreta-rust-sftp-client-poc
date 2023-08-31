use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvAttributes {

    pub app_server_host: String,
    pub app_server_port: String,

    pub sftp_client_poll_cron_exp: String,
    pub sftp_client_partners_list: String,

    pub sftp_server_endpoint_url: String,
    pub sftp_server_basedir: String,
    pub sftp_server_in_dir: String,
    pub sftp_server_in_dir_work: String,
    pub sftp_server_in_dir_error: String,
    pub sftp_server_in_dir_success: String,
    pub sftp_server_out_dir: String,
    pub sftp_server_username: String,
    pub sftp_server_password: String
}

impl EnvAttributes {
    pub fn new() -> EnvAttributes {

        let sftp_client_poll_cron_exp = std::env::var("SFTP_CLIENT_POLL_CRON_EXPR").expect("SFTP_CLIENT_POLL_CRON_EXPR must be set.");
        let sftp_client_partners_list = std::env::var("SFTP_CLIENT_PARTNERS_LIST").expect("SFTP_CLIENT_PARTNERS_LIST must be set.");

        let sftp_server_endpoint_url = std::env::var("SFTP_SERVER_ENDPOINT_URL").expect("SFTP_SERVER_ENDPOINT_URL must be set.");

        let sftp_server_basedir = std::env::var("SFTP_SERVER_BASEDIR").expect("SFTP_SERVER_BASEDIR must be set.");
        let sftp_server_in_dir = std::env::var("SFTP_SERVER_IN_DIR").expect("SFTP_SERVER_IN_DIR must be set.");
        let sftp_server_in_dir_work = std::env::var("SFTP_SERVER_IN_WIP_DIR").expect("SFTP_SERVER_IN_WIP_DIR must be set.");
        let sftp_server_in_dir_error = std::env::var("SFTP_SERVER_IN_ERR_DIR").expect("SFTP_SERVER_IN_ERR_DIR must be set.");
        let sftp_server_in_dir_success = std::env::var("SFTP_SERVER_IN_SUCCESS_DIR").expect("SFTP_SERVER_IN_SUCCESS_DIR must be set.");
        let sftp_server_out_dir = std::env::var("SFTP_SERVER_OUT_DIR").expect("SFTP_SERVER_OUT_DIR must be set.");

        let sftp_server_username = std::env::var("SFTP_SERVER_USERNAME").expect("SFTP_SERVER_USERNAME must be set.");
        let sftp_server_password = std::env::var("SFTP_SERVER_USERPWD").expect("SFTP_SERVER_USERPWD must be set.");

        let app_server_host = std::env::var("APP_SERVER_HOST").expect("APP_SERVER_HOST must be set.");
        let app_server_port = std::env::var("APP_SERVER_PORT").expect("APP_SERVER_PORT must be set.");


        EnvAttributes {
            app_server_host,
            app_server_port,
            sftp_client_poll_cron_exp,
            sftp_server_endpoint_url,
            sftp_server_basedir,
            sftp_server_in_dir,
            sftp_server_in_dir_work,
            sftp_server_in_dir_error,
            sftp_server_in_dir_success,
            sftp_server_out_dir,
            sftp_server_username,
            sftp_server_password,
            sftp_client_partners_list,
        }
    }
}