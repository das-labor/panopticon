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

use crate::errors::*;
use crate::ffi::{start_gui_loop, update_current_session, update_function_edges, update_function_node, update_layout_task, update_sidebar_items, update_undo_redo};
use panopticon_core::Function;
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::ptr;
use crate::types::{CBasicBlockLine, CRecentSession, CSidebarItem};

use uuid::Uuid;

pub trait Glue {
    fn get_function(uuid: &Uuid, only_entry: bool, do_nodes: bool, do_edges: bool) -> Result<()>;
    fn subscribe_to(uuid: &Uuid, state: bool) -> Result<()>;
    fn open_program(path: &str) -> Result<()>;
    fn save_session(path: &str) -> Result<()>;
    fn comment_on(address: u64, comment: &str) -> Result<()>;
    fn rename_function(uuid: &Uuid, name: &str) -> Result<()>;
    fn set_value_for(uuid: &Uuid, variable: &str, value: &str) -> Result<()>;
    fn undo() -> Result<()>;
    fn redo() -> Result<()>;

    fn exec(qml_dir: &Path, initial_file: Option<String>, recent_sessions: Vec<(String, String, PathBuf, u32)>) -> Result<()> {
        let qml_dir = CString::new(format!("{}", qml_dir.display()).as_bytes()).unwrap();
        let initial_file = CString::new(initial_file.unwrap_or_default()).unwrap();
        let recent_sessions = recent_sessions.into_iter().filter_map(|x| CRecentSession::new(x.0, x.1, &x.2, x.3).ok()).collect::<Vec<_>>();
        let mut recent_sess_ptrs: Vec<*const CRecentSession> = recent_sessions.iter().map(|x| -> *const CRecentSession { x }).collect();

        recent_sess_ptrs.push(ptr::null());

        unsafe {
            start_gui_loop(
                qml_dir.as_ptr(),
                initial_file.as_ptr(),
                recent_sess_ptrs.as_ptr(),
                Self::get_function_plumbing,
                Self::subscribe_to_plumbing,
                Self::open_program_plumbing,
                Self::save_session_plumbing,
                Self::comment_on_plumbing,
                Self::rename_function_plumbing,
                Self::set_value_for_plumbing,
                Self::undo_plumbing,
                Self::redo_plumbing,
            );
        }

        Ok(())
    }

    fn update_sidebar(funcs: &[Function]) {
        let items = funcs.iter().filter_map(|f| CSidebarItem::new(f).ok()).collect::<Vec<_>>();
        let mut ptrs: Vec<*const CSidebarItem> = items.iter().map(|i| -> *const CSidebarItem { i }).collect();

        ptrs.push(ptr::null());
        unsafe {
            update_sidebar_items(ptrs.as_slice().as_ptr());
        }
    }

    fn send_function_node(uuid: CString, id: usize, x: f32, y: f32, is_entry: bool, lines: &[CBasicBlockLine]) -> Result<()> {
        let mut ptrs: Vec<*const CBasicBlockLine> = lines.iter().map(|i| -> *const CBasicBlockLine { i }).collect();

        ptrs.push(ptr::null());
        unsafe {
            update_function_node(
                uuid.as_ptr(),
                id as u32,
                x,
                y,
                if is_entry { 1 } else { 0 },
                ptrs.as_slice().as_ptr(),
            );
        }

        Ok(())
    }

    fn send_function_edges(
        uuid: CString,
        ids: &[u32],
        labels: &[CString],
        kinds: &[CString],
        head_xs: &[f32],
        head_ys: &[f32],
        tail_xs: &[f32],
        tail_ys: &[f32],
        svg: CString,
    ) -> Result<()> {
        let mut label_ptrs: Vec<*const i8> = labels.iter().map(|i| -> *const i8 { i.as_ptr() }).collect();
        let mut kind_ptrs: Vec<*const i8> = kinds.iter().map(|i| -> *const i8 { i.as_ptr() }).collect();

        label_ptrs.push(ptr::null());
        kind_ptrs.push(ptr::null());

        unsafe {
            update_function_edges(
                uuid.as_ptr(),
                ids.as_ptr(),
                label_ptrs.as_slice().as_ptr(),
                kind_ptrs.as_slice().as_ptr(),
                head_xs.as_ptr(),
                head_ys.as_ptr(),
                tail_xs.as_ptr(),
                tail_ys.as_ptr(),
                svg.as_ptr(),
            )
        }

        Ok(())
    }

    fn send_undo_redo_update(undo: bool, redo: bool) -> Result<()> {
        unsafe {
            update_undo_redo(if undo { 1 } else { 0 }, if redo { 1 } else { 0 });
        }

        Ok(())
    }

    fn send_current_session(path: CString) -> Result<()> {
        unsafe {
            update_current_session(path.as_ptr());
        }

        Ok(())
    }

    fn send_layout_task(t: &CString) -> Result<()> {
        unsafe {
            update_layout_task(t.as_ptr());
        }

        Ok(())
    }

    extern "C" fn get_function_plumbing(uuid_cstr: *const i8, only_entry: i8, do_nodes: i8, do_edges: i8) -> i32 {
        let uuid = unsafe { CStr::from_ptr(uuid_cstr) }.to_string_lossy().to_string();
        let uuid = match Uuid::parse_str(&uuid) {
            Ok(uuid) => uuid,
            Err(s) => {
                error!("get_function(): {}", s);
                return -1;
            }
        };

        match Self::get_function(&uuid, only_entry != 0, do_nodes != 0, do_edges != 0) {
            Ok(()) => 0,
            Err(s) => {
                error!("get_function(): {}", s);
                -1
            }
        }
    }

    extern "C" fn subscribe_to_plumbing(uuid_cstr: *const i8, state: i8) -> i32 {
        let uuid = unsafe { CStr::from_ptr(uuid_cstr) }.to_string_lossy().to_string();
        let uuid = match Uuid::parse_str(&uuid) {
            Ok(uuid) => uuid,
            Err(s) => {
                error!("subscribe_to(): {}", s);
                return -1;
            }
        };

        match Self::subscribe_to(&uuid, state != 0) {
            Ok(()) => 0,
            Err(s) => {
                error!("subcribe_to(): {}", s);
                -1
            }
        }
    }

    extern "C" fn open_program_plumbing(path: *const i8) -> i32 {
        let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
        match Self::open_program(&path) {
            Ok(()) => 0,
            Err(s) => {
                error!("open_program(): {}", s);
                -1
            }
        }
    }

    extern "C" fn save_session_plumbing(path: *const i8) -> i32 {
        let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
        match Self::save_session(&path) {
            Ok(()) => 0,
            Err(s) => {
                error!("save_session(): {}", s);
                -1
            }
        }
    }

    extern "C" fn comment_on_plumbing(address: u64, comment: *const i8) -> i32 {
        let comment = unsafe { CStr::from_ptr(comment) }.to_string_lossy().to_string();
        match Self::comment_on(address, &comment) {
            Ok(()) => 0,
            Err(s) => {
                error!("comment_on(): {}", s);
                -1
            }
        }
    }

    extern "C" fn rename_function_plumbing(uuid: *const i8, name: *const i8) -> i32 {
        let name = unsafe { CStr::from_ptr(name) }.to_string_lossy().to_string();
        let uuid = unsafe { CStr::from_ptr(uuid) }.to_string_lossy().to_string();
        let uuid = match Uuid::parse_str(&uuid) {
            Ok(uuid) => uuid,
            Err(s) => {
                error!("rename_function(): {}", s);
                return -1;
            }
        };

        match Self::rename_function(&uuid, &name) {
            Ok(()) => 0,
            Err(s) => {
                error!("rename_function(): {}", s);
                -1
            }
        }
    }

    extern "C" fn set_value_for_plumbing(uuid: *const i8, variable: *const i8, value: *const i8) -> i32 {
        let uuid = unsafe { CStr::from_ptr(uuid) }.to_string_lossy().to_string();
        let variable = unsafe { CStr::from_ptr(variable) }.to_string_lossy().to_string();
        let value = unsafe { CStr::from_ptr(value) }.to_string_lossy().to_string();
        let uuid = match Uuid::parse_str(&uuid) {
            Ok(uuid) => uuid,
            Err(s) => {
                error!("set_value_for(): {}", s);
                return -1;
            }
        };

        match Self::set_value_for(&uuid, &variable, &value) {
            Ok(()) => 0,
            Err(s) => {
                error!("set_value_for(): {}", s);
                -1
            }
        }
    }

    extern "C" fn undo_plumbing() -> i32 {
        match Self::undo() {
            Ok(()) => 0,
            Err(s) => {
                error!("undo(): {}", s);
                -1
            }
        }
    }

    extern "C" fn redo_plumbing() -> i32 {
        match Self::redo() {
            Ok(()) => 0,
            Err(s) => {
                error!("redo(): {}", s);
                -1
            }
        }
    }
}
