/* SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only  */
/* Copyright (c) 2021 Gavin Henry <ghenry@sentrypeer.org> */

#include <stdio.h>
#include <stdlib.h>
#include <syslog.h>

// Produced by autoconf and cmake (manually by me)
#include "config.h"

#include "conf.h"
#include "sip_daemon.h"
#include "http_daemon.h"

int main(int argc, char **argv)
{
	struct sentrypeer_config config;

	if (process_cli(&config, argc, argv) != EXIT_SUCCESS) {
		exit(EXIT_FAILURE);
	}

	if (config.syslog_mode) {
		openlog(PACKAGE_NAME, LOG_PID, LOG_USER);
	}

	if (config.debug_mode || config.verbose_mode) {
		char starting_fmt[] = "Starting %s...\n";
		fprintf(stderr, starting_fmt, PACKAGE_NAME);
		if (config.syslog_mode) {
			syslog(LOG_ERR, starting_fmt, PACKAGE_NAME);
		}
	}

	// Threaded, so start the HTTP daemon first
	char failed[] = "Failed to start %s server on port %d\n";
	if (http_daemon_init(&config) == EXIT_FAILURE) {
		fprintf(stderr, failed, "HTTP", HTTP_DAEMON_PORT);
		perror("http_daemon_init");
		if (config.syslog_mode) {
			syslog(LOG_ERR, failed, "HTTP");
		}
		exit(EXIT_FAILURE);
	}

	// Blocking, so start the SIP daemon last
	if (sip_daemon_init(&config) == EXIT_FAILURE) {
		fprintf(stderr, failed, "SIP", SIP_DAEMON_PORT);
		perror("sip_daemon_init");
		if (config.syslog_mode) {
			syslog(LOG_ERR, failed, "SIP");
		}
		exit(EXIT_FAILURE);
	}

	return EXIT_SUCCESS;
}