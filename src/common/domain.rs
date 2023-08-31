use serde_derive::{Deserialize, Serialize};
use crate::common::util::EnvAttributes;

/// Struct that represents the application's context
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppContext {
    pub sftp_cron_expression: String,
    pub sftp_client_partners_list: String,
    pub app_server_host: String,
    pub app_server_port: String,
    pub sftp_context: SftpContext
}

impl AppContext {
    pub fn new() -> AppContext {

        let envs = EnvAttributes::new();

        AppContext {
            sftp_cron_expression: envs.sftp_client_poll_cron_exp,
            sftp_client_partners_list: envs.sftp_client_partners_list,
            app_server_host: envs.app_server_host,
            app_server_port: envs.app_server_port,

            sftp_context: SftpContext {
                username: envs.sftp_server_username,
                password: envs.sftp_server_password,
                endpoint: envs.sftp_server_endpoint_url,
                remote_base_dir: envs.sftp_server_basedir,
                partner_dir_structure: SftpPartnerDirStructure {
                    in_dir: envs.sftp_server_in_dir,
                    in_dir_work: envs.sftp_server_in_dir_work,
                    in_dir_success: envs.sftp_server_in_dir_success,
                    out_dir: envs.sftp_server_out_dir,
                },
            },
        }
    }
}


/// Struct that represents an SFTP instance context container simple data
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SftpContext {
    pub username: String,
    pub password: String,
    pub endpoint: String,
    pub remote_base_dir: String,
    pub partner_dir_structure: SftpPartnerDirStructure
}

/// Struct that represents the directory structure all partners will have
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SftpPartnerDirStructure {
    pub in_dir: String,
    pub in_dir_work: String,
    pub in_dir_success: String,
    pub out_dir: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SftpPartner {
    pub name: String,
    pub enabled: bool
}

