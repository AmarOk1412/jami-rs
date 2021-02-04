/**
 * Copyright (c) 2018-2021, Sébastien Blin <sebastien.blin@enconn.fr>
 * All rights reserved.
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright
 *  notice, this list of conditions and the following disclaimer.
 * * Redistributions in binary form must reproduce the above copyright
 *  notice, this list of conditions and the following disclaimer in the
 *  documentation and/or other materials provided with the distribution.
 * * Neither the name of the University of California, Berkeley nor the
 *  names of its contributors may be used to endorse or promote products
 *  derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE REGENTS AND CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 **/
use app_dirs::{get_app_dir, AppDataType, AppInfo};
use serde::{Deserialize, Serialize};

/**
 * A class used to store transfers per account per conversation
 **/
#[derive(Serialize, Deserialize)]
pub struct TransferManager {
    path: String,
}

impl TransferManager {
    /**
     * Generate a new TransferManager
     * @return the new manager
     */
    pub fn new() -> Self {
        let db_path = get_app_dir(
            AppDataType::UserData,
            &AppInfo {
                name: "jami",
                author: "SFL",
            },
            "jami-cli.db",
        );

        let path = db_path.unwrap().into_os_string().into_string().unwrap();
        let conn = rusqlite::Connection::open(&*path).unwrap();
        let version: i32 = conn
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap_or(0);
        let do_migration = version != 1;
        if do_migration {
            conn.execute("CREATE TABLE IF NOT EXISTS transfers (
                id               INTEGER PRIMARY KEY,
                account_id       TEXT,
                conversation_id  TEXT,
                tid              TEXT,
                path             TEXT
                )", rusqlite::NO_PARAMS).unwrap();
            conn.pragma_update(None, "user_version", &1).unwrap();
        }

        Self {
            path
        }
    }

    pub fn path(&mut self, account_id: String, conv_id: String, tid: String) -> Option<String> {
        let conn = rusqlite::Connection::open(&*self.path).unwrap();
        let mut stmt = conn.prepare("SELECT path FROM transfers WHERE account_id=:account_id AND conversation_id=:conversation_id AND tid=:tid").unwrap();
        let mut rows = stmt.query_named(&[(":account_id", &account_id), (":conversation_id", &conv_id), (":tid", &tid)]).unwrap();
        if let Ok(Some(row)) = rows.next() {
            return match row.get(0) {
                Ok(r) => Some(r),
                _ => None,
            };
        }
        None
    }

    pub fn set_file_path(&mut self, account_id: String, conv_id: String, tid: String, path: String) -> Option<i32> {
        let conn = rusqlite::Connection::open(&*self.path).unwrap();
        // Else insert!
        let mut conn = conn.prepare("INSERT INTO transfers (account_id, conversation_id, tid, path)
                                     VALUES (:account_id, :conversation_id, :tid, :path)").unwrap();
        match conn.execute_named(&[(":account_id", &account_id),
                                   (":conversation_id", &conv_id),
                                   (":tid", &tid),
                                   (":path", &path)]) {
            Ok(id) => {
                return Some(id as i32);
            }
            Err(_e) => {
                return None;
            }
        }
    }

}
