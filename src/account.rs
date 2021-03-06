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

use serde::{Deserialize, Serialize};
use std::fmt;

/**
 * Represent a Jami account, just here to store informations.
 **/
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Account {
    pub id: String,
    pub hash: String,
    pub alias: String,
    pub registered_name: String,
    pub enabled: bool,
}

// Used for println!
impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]: {} ({}) - Active: {}",
            self.id, self.hash, self.alias, self.enabled
        )
    }
}

impl Account {
    pub fn null() -> Account {
        Account {
            id: String::new(),
            hash: String::new(),
            alias: String::new(),
            registered_name: String::new(),
            enabled: false,
        }
    }

    pub fn get_display_name(&self) -> String {
        if !self.alias.is_empty() {
            return self.alias.clone();
        }
        if !self.registered_name.is_empty() {
            return self.registered_name.clone();
        }
        return self.hash.clone();
    }
}
