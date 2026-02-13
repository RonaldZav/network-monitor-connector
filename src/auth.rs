use actix_web::{HttpRequest, HttpResponse};
use ipnetwork::IpNetwork;
use std::net::IpAddr;
use crate::config::AppConfig;

pub fn check_auth(req: &HttpRequest, config: &AppConfig) -> Result<(), HttpResponse> {
    if config.whitelist.enabled {
        if let Some(peer_addr) = req.peer_addr() {
            let ip = peer_addr.ip();
            let allowed = config.whitelist.list.iter().any(|cidr| {
                if let Ok(network) = cidr.parse::<IpNetwork>() {
                    network.contains(ip)
                } else {
                    if let Ok(allowed_ip) = cidr.parse::<IpAddr>() {
                        allowed_ip == ip
                    } else {
                        false
                    }
                }
            });

            if !allowed {
                return Err(HttpResponse::Forbidden().body("Access denied"));
            }
        }
    }

    let auth_header = req.headers().get("Authorization");
    let mut token_match = false;

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {
            if header_str.starts_with("Bearer ") {
                if header_str[7..] == config.token {
                    token_match = true;
                }
            }
        }
    }

    if !token_match {
        let query_string = req.query_string();
        for pair in query_string.split('&') {
            let mut parts = pair.split('=');
            if let Some(key) = parts.next() {
                if key == "token" {
                    if let Some(value) = parts.next() {
                        if value == config.token {
                            token_match = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    if !token_match {
        return Err(HttpResponse::Unauthorized().body("Access denied"));
    }

    Ok(())
}
