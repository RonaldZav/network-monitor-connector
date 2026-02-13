mod config;
mod models;
mod system_info;
mod auth;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use sysinfo::System;
use colored::*;
use crate::config::{AppConfig, load_or_create_config};
use crate::models::MonitorInfo;
use crate::system_info::{get_cpu_info, get_ram_info, refresh_cpu, refresh_ram, refresh_all};
use crate::auth::check_auth;

struct AppState {
    sys: Mutex<System>,
    config: AppConfig,
}

#[get("/v1/monitor/cpu")]
async fn monitor_cpu(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    if let Err(response) = check_auth(&req, &data.config) {
        return response;
    }

    let mut sys = data.sys.lock().unwrap();
    refresh_cpu(&mut sys);
    let info = get_cpu_info(&mut sys);
    HttpResponse::Ok().json(info)
}

#[get("/v1/monitor/ram")]
async fn monitor_ram(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    if let Err(response) = check_auth(&req, &data.config) {
        return response;
    }

    let mut sys = data.sys.lock().unwrap();
    refresh_ram(&mut sys);
    let info = get_ram_info(&mut sys);
    HttpResponse::Ok().json(info)
}

#[get("/v1/monitor")]
async fn monitor_all(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    if let Err(response) = check_auth(&req, &data.config) {
        return response;
    }

    let mut sys = data.sys.lock().unwrap();
    refresh_all(&mut sys);

    let cpu_info = get_cpu_info(&mut sys);
    let ram_info = get_ram_info(&mut sys);

    let info = MonitorInfo {
        cpu: cpu_info,
        ram: ram_info,
    };

    HttpResponse::Ok().json(info)
}

fn print_banner(host: &str, port: u16, token: &str) {
    println!("{}", "========================================".cyan().bold());
    println!("{}", "           NETWORK MONITOR              ".green().bold());
    println!("{}", "            by RonaldZav                ".yellow().italic());
    println!("{}", "========================================".cyan().bold());
    println!("   {}: {}", "Host".bold(), host);
    println!("   {}: {}", "Port".bold(), port);
    println!("   {}: {}", "Token".bold(), token);
    println!("{}", "========================================".cyan().bold());
    println!("{}", "Server is running...".blue());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_or_create_config();
    let bind_address = format!("{}:{}", config.host, config.port);

    print_banner(&config.host, config.port, &config.token);

    let sys = System::new_all();
    let app_state = web::Data::new(AppState {
        sys: Mutex::new(sys),
        config: config.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(monitor_cpu)
            .service(monitor_ram)
            .service(monitor_all)
    })
    .bind(bind_address)?
    .run()
    .await
}
