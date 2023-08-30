use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppContext {
    pub sftp_cron_expression: String,
    pub sftp_context: SftpContext
}

impl AppContext {
    pub fn new() -> AppContext {

        let sftp_poll_cron_expr = std::env::var("SFTP_CLIENT_POLL_CRON_EXPR").expect("SFTP_CLIENT_POLL_CRON_EXPR must be set.");
        
        AppContext {
            sftp_cron_expression: sftp_poll_cron_expr,
            sftp_context: SftpContext {
                username: "".to_string(),
                password: "".to_string(),
                endpoint: "".to_string(),
                remote_base_dir: "".to_string(),
                partner_dir_structure: SftpPartnerDirStructure {
                    in_dir: "".to_string(),
                    in_dir_work: "".to_string(),
                    in_dir_success: "".to_string(),
                    out_dir: "".to_string(),
                },
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SftpContext {
    pub username: String,
    pub password: String,
    pub endpoint: String,
    pub remote_base_dir: String,
    pub partner_dir_structure: SftpPartnerDirStructure
}

#[derive(Debug, Serialize, Deserialize)]
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

