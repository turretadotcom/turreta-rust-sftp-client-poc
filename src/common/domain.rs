use serde_derive::{Deserialize, Serialize};
use crate::common::util::EnvAttributes;
use regex::Regex;

/// Struct that represents the application's context
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppContext {
    pub sftp_cron_expression: String,
    pub sftp_client_partners_list: String,
    pub sftp_client_partners: Vec<String>,
    pub app_server_host: String,
    pub app_server_port: String,
    pub sftp_context: SftpContext
}

impl AppContext {
    pub fn new() -> AppContext {

        let envs = EnvAttributes::new();

        let mut final_partner_list = Vec::new();

        let tmp_partner_list = &envs.sftp_client_partners_list;

        let regex_result =Regex::new(r"[,\s]");
        let regex = regex_result.unwrap();
        let regex_partner_list = regex.split(tmp_partner_list.as_str());
        let y:Vec<_> = regex_partner_list.into_iter().collect();

        for part in y {
            final_partner_list.push(part.to_string());
        }

        AppContext {
            sftp_cron_expression: envs.sftp_client_poll_cron_exp,
            sftp_client_partners_list: envs.sftp_client_partners_list,
            sftp_client_partners: final_partner_list.clone(),
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
                    in_dir_error: envs.sftp_server_in_dir_error,
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
    pub in_dir_error: String,
    pub in_dir_success: String,
    pub out_dir: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SftpPartner {
    pub name: String,
    pub enabled: bool
}

pub trait DataTransformer {
    fn transform(&self, in_data: String) -> String;
}