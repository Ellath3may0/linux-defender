//! The configuration paths a full backup covers.

/// Concatenate arrays at compile time for const reference
macro_rules! concat {
    ($($s:expr),+ $(,)?) => {{
        const LEN: usize = 0 $(+ $s.len())+;
        const OUT: [&str; LEN] = {
            let mut out = [""; LEN];
            let mut n = 0;
            $(
                let mut i = 0;
                while i < $s.len() {
                    out[n] = $s[i];
                    n += 1;
                    i += 1;
                }
            )+
            out
        };
        &OUT
    }};
}

/// A combined list of config files and directories worth preserving
pub const ALL_PATHS: &[&str] = concat!(
    SYSTEM_PATHS,
    APACHE_HTTPD,
    NGINX,
    OPENCART,
    POSTFIX,
    DOVECOT,
    SPLUNK
);

/// A combined list of typical system config files and directories
pub const SYSTEM_PATHS: &[&str] = concat!(SSH, DNS_HOSTS, CRON, PAM);

/// A combined list of typical service & system configuration files for an ecomm server
pub const ECOMM_PATHS: &[&str] = concat!(SYSTEM_PATHS, APACHE_HTTPD, NGINX, OPENCART);

/// A combined list of typical service & system configuration files for a mail server
pub const MAIL_PATHS: &[&str] = concat!(SYSTEM_PATHS, POSTFIX, DOVECOT);

/// A combined list of typical service & system configuration files for a splunk server
pub const SPLUNK_PATHS: &[&str] = concat!(SYSTEM_PATHS, SPLUNK);

/// SSH config files and keys
pub const SSH: &[&str] = &[
    "/etc/ssh/sshd_config",
    "/etc/ssh/ssh_config",
    "/etc/ssh/ssh_host_rsa_key.pub",
    "/etc/ssh/ssh_host_ecdsa_key.pub",
    "/etc/ssh/ssh_host_ed25519_key.pub",
];

/// DNS configuration and fixed hostnames
pub const DNS_HOSTS: &[&str] = &["/etc/hostname", "/etc/hosts", "/etc/resolv.conf"];

/// System and user crontabs
pub const CRON: &[&str] = &["/etc/crontab", "/var/spool/cron/crontabs/"];

/// PAM configuration and directory
pub const PAM: &[&str] = &["/etc/pam.conf", "/etc/pam.d"];

/// Apache/httpd configurations and directories
pub const APACHE_HTTPD: &[&str] = &[
    "/etc/apache2",
    "/etc/httpd",
    "/etc/apache2/apache2.conf",
    "/etc/apache2/ports.conf",
    "/etc/apache2/sites-available",
    "/etc/apache2/sites-enabled",
    "/etc/apache2/mods-available",
    "/etc/apache2/mods-enabled",
    "/etc/apache2/conf-available/",
    "/etc/apache2/conf-enabled/",
    "/etc/httpd/conf/httpd.conf",
    "/etc/httpd/conf.d",
];

/// Nginx configurations and directories
pub const NGINX: &[&str] = &[
    "/etc/nginx/nginx.conf",
    "/etc/nginx/sites-available/",
    "/etc/nginx/sites-enabled/",
    "/etc/nginx/conf.d/",
];

/// Opencart configurations
pub const OPENCART: &[&str] = &[
    "/var/www/html/opencart/config.php",
    "/var/www/html/opencart/admin/config.php",
    "/var/www/opencart/config.php",
    "/var/www/opencart/admin/config.php",
    "/var/www/html/config.php",
    "/var/www/html/admin/config.php",
];

/// Postfix config files and directories
pub const POSTFIX: &[&str] = &["/etc/postfix", "/etc/aliases"];

/// Dovecot config files and directories
pub const DOVECOT: &[&str] = &["/etc/dovecot", "/etc/dovecot/conf.d"];

/// Splunk service configuration and directories
pub const SPLUNK: &[&str] = &[
    "/opt/splunk/",
    "/opt/splunk/etc/",
    "/opt/splunk/etc/system/local/",
    "/opt/splunk/etc/system/default/",
    "/opt/splunk/etc/apps/",
    "/opt/splunk/etc/users/",
    "/opt/splunk/etc/deployment-apps/",
    "/opt/splunk/etc/master-apps/",
    "/opt/splunk/etc/shcluster/",
    "/etc/systemd/system/Splunkd.service",
    "/etc/init.d/splunk",
    "/etc/sysconfig/splunk",
    "/opt/splunk/etc/splunk-launch.conf",
];
