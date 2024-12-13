/* SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only  */
/* Copyright (c) 2021 - 2024 Gavin Henry <ghenry@sentrypeer.org> */
/*
   _____            _              _____
  / ____|          | |            |  __ \
 | (___   ___ _ __ | |_ _ __ _   _| |__) |__  ___ _ __
  \___ \ / _ \ '_ \| __| '__| | | |  ___/ _ \/ _ \ '__|
  ____) |  __/ | | | |_| |  | |_| | |  |  __/  __/ |
 |_____/ \___|_| |_|\__|_|   \__, |_|   \___|\___|_|
                              __/ |
                             |___/
*/
use crate::config::SentryPeerConfig;
use crate::sip::{gen_sip_reply, log_sip_packet};
use std::future::Future;
use std::net::SocketAddr;
use tokio::io::{split, AsyncReadExt};
use tokio::net::TcpStream;

pub fn handle_tcp_connection(
    stream: TcpStream,
    sentrypeer_config: SentryPeerConfig,
    peer_addr: std::net::SocketAddr,
    addr: SocketAddr,
) -> impl Future<Output = i32> {
    async move {
        let mut buf = [0; 1024];
        let (mut reader, writer) = split(stream);
        let bytes_read = reader.read(&mut buf).await.unwrap();

        let debug_mode = (unsafe { *sentrypeer_config.p }).debug_mode;
        let verbose_mode = (unsafe { *sentrypeer_config.p }).verbose_mode;
        let sip_responsive_mode = (unsafe { *sentrypeer_config.p }).sip_responsive_mode;

        if log_sip_packet(
            sentrypeer_config,
            buf.to_vec(),
            bytes_read,
            peer_addr,
            addr,
            "TCP",
        ) != libc::EXIT_SUCCESS
        {
            eprintln!("Failed to log SIP packet");
        }

        if debug_mode || verbose_mode {
            eprintln!(
                "Received: {:?}",
                String::from_utf8_lossy(&buf[..bytes_read])
            );
        }

        if sip_responsive_mode {
            gen_sip_reply(writer).await;
        }

        libc::EXIT_SUCCESS
    }
}