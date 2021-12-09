/* SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only  */
/* Copyright (c) 2021 Gavin Henry <ghenry@sentrypeer.org> */
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

#ifndef SENTRYPEER_DATABASE_H
#define SENTRYPEER_DATABASE_H 1

#include <sqlite3.h>

#include "bad_actor.h"
#include "conf.h"

#define DEFAULT_DB_FILE_NAME "sentrypeer.db"

int db_insert_bad_actor(bad_actor const *bad_actor_event,
			sentrypeer_config const *config);

int db_select_bad_actor_by_ip(const char *bad_actor_ip_address,
			      sentrypeer_config const *config);

int db_select_bad_actor_by_uuid(const char *bad_actor_event_uuid,
				sentrypeer_config const *config);

int db_select_bad_actors(bad_actor *bad_actors,
			 sentrypeer_config const *config);

int db_set_error_log_callback(void);
void db_error_log_callback(int err_code, const char *msg);

#endif //SENTRYPEER_DATABASE_H
