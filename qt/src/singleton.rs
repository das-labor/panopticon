/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use qml::{
    QObject,
    QVariant,
    QMetaType,
    QMetaTypable,
    QObjectMacro,
    QListModel,
    emit_signal,
};
use paths::{
    session_directory
};
use std::fs;
use panopticon::{
    Project
};
use errors::*;

Q_LISTMODEL! {
    pub QRecentSessions {
        timestamp: i32,
        title: String,
        typ: String,
        path: String
    }
}

pub struct Panopticon {
    // QML
    pub recentSessions: QRecentSessions,
}

impl Panopticon {
    fn read_recent_sessions() -> Result<QRecentSessions> {
        let path = session_directory()?;
        let mut ret = QRecentSessions::new();

        if let Ok(dir) = fs::read_dir(path) {

            for ent in dir.filter_map(|x| x.ok()) {
                if let Ok(ref project) = Project::open(&ent.path()) {
                    if let Ok(ref md) = ent.metadata() {
                        let mtime = md.modified()?.duration_since(::std::time::UNIX_EPOCH)?.as_secs() as i32;
                        let fname = ent.path().file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or("(error)".to_string());
                        ret.append_row(mtime,project.name.clone(),"".to_string(),fname);
                    }
                }
            }
        }
        Ok(ret)
    }
}

impl Default for Panopticon {
    fn default() -> Panopticon {
        Panopticon{
            recentSessions: Self::read_recent_sessions().unwrap_or_else(|_| QRecentSessions::new()),
        }
    }
}

Q_OBJECT!(
pub Panopticon as QPanopticon {
    signals:
    slots:
    properties:
        // recent sessions
        recentSessions: QVariant; read: get_recent_sessions, write: set_recent_sessions, notify: recent_sessions_changed;
        haveRecentSessions: bool; read: get_have_recent_sessions, write: set_have_recent_sessions, notify: have_recent_sessions_changed;
});
