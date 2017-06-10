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

use types::{CBasicBlockLine, CRecentSession, CSidebarItem};

extern "C" {
    pub fn start_gui_loop(
        qml_dir: *const i8,
        inital_file: *const i8,
        recent_sessions: *const *const CRecentSession,
        get_function: extern "C" fn(*const i8, i8, i8, i8) -> i32,
        subscribe_to: extern "C" fn(*const i8, i8) -> i32,
        open_program: extern "C" fn(*const i8) -> i32,
        save_session: extern "C" fn(*const i8) -> i32,
        comment_on: extern "C" fn(u64, *const i8) -> i32,
        rename_function: extern "C" fn(*const i8, *const i8) -> i32,
        set_value_for: extern "C" fn(*const i8, *const i8, *const i8) -> i32,
        undo: extern "C" fn() -> i32,
        redo: extern "C" fn() -> i32,
    );

    // thread-safe
    pub fn update_function_node(uuid: *const i8, id: u32, x: f32, y: f32, is_entry: i8, lines: *const *const CBasicBlockLine);

    // thread-safe
    pub fn update_function_edges(
        uuid: *const i8,
        ids: *const u32,
        labels: *const *const i8,
        kinds: *const *const i8,
        head_xs: *const f32,
        head_ys: *const f32,
        tail_xs: *const f32,
        tail_ys: *const f32,
        svg: *const i8,
    );

    // thread-safe
    pub fn update_sidebar_items(items: *const *const CSidebarItem);

    // thread-safe
    pub fn update_undo_redo(undo: i8, redo: i8);

    // thread-safe
    pub fn update_current_session(path: *const i8);

    // thread-safe
    pub fn update_layout_task(task: *const i8);
}
