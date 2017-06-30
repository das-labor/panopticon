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

use errors::*;
use panopticon_core::{ControlFlowTarget, Function};
use panopticon_graph_algos::GraphTrait;
use std::ffi::CString;
use std::path::Path;
use std::ptr;

#[repr(C)]
pub struct CSidebarItem {
    title: *const i8,
    subtitle: *const i8,
    uuid: *const i8,
}

impl CSidebarItem {
    pub fn new(func: &Function) -> Result<CSidebarItem> {

        let cfg = &func.cflow_graph;
        let entry = func.entry_point
            .and_then(|vx| cfg.vertex_label(vx))
            .and_then(
                |lb| if let &ControlFlowTarget::Resolved(ref bb) = lb {
                    Some(bb.area.start)
                } else {
                    None
                }
            );
        let str_entry = CString::new(entry.map(|x| format!("0x{:x}", x)).unwrap_or("".to_string()).into_bytes())?;
        let name = CString::new(func.name.to_string().into_bytes())?;
        let uuid = CString::new(func.uuid().to_string().into_bytes())?;

        Ok(
            CSidebarItem {
                title: name.into_raw(),
                subtitle: str_entry.into_raw(),
                uuid: uuid.into_raw(),
            }
        )
    }
}

impl Drop for CSidebarItem {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self.title as *mut i8);
            CString::from_raw(self.subtitle as *mut i8);
            CString::from_raw(self.uuid as *mut i8);
        }
    }
}

#[repr(C)]
pub struct CBasicBlockOperand {
    kind: *const i8,
    display: *const i8,
    alt: *const i8,
    data: *const i8,
}

impl CBasicBlockOperand {
    pub fn new(kind: String, display: String, alt: String, data: String) -> Result<CBasicBlockOperand> {
        let kind = CString::new(kind.into_bytes())?;
        let display = CString::new(display.into_bytes())?;
        let alt = CString::new(alt.into_bytes())?;
        let data = CString::new(data.into_bytes())?;

        Ok(
            CBasicBlockOperand {
                kind: kind.into_raw(),
                display: display.into_raw(),
                alt: alt.into_raw(),
                data: data.into_raw(),
            }
        )
    }
}

impl Drop for CBasicBlockOperand {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self.kind as *mut i8);
            CString::from_raw(self.display as *mut i8);
            CString::from_raw(self.alt as *mut i8);
            CString::from_raw(self.data as *mut i8);
        }
    }
}

#[repr(C)]
pub struct CBasicBlockLine {
    opcode: *const i8,
    region: *const i8,
    offset: u64,
    comment: *const i8,
    args: *const *const CBasicBlockOperand,
}

impl CBasicBlockLine {
    pub fn new(opcode: String, region: String, offset: u64, comment: String, args: Vec<CBasicBlockOperand>) -> Result<CBasicBlockLine> {
        let opcode = CString::new(opcode.into_bytes())?;
        let region = CString::new(region.into_bytes())?;
        let comment = CString::new(comment.into_bytes())?;
        let mut args: Vec<*const CBasicBlockOperand> = args.into_iter().map(|i| -> *const CBasicBlockOperand { Box::into_raw(Box::new(i)) }).collect();

        args.push(ptr::null());

        Ok(
            CBasicBlockLine {
                opcode: opcode.into_raw(),
                region: region.into_raw(),
                offset: offset,
                comment: comment.into_raw(),
                args: unsafe { (*Box::into_raw(Box::new(args))).as_ptr() },
            }
        )
    }
}

impl Drop for CBasicBlockLine {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self.opcode as *mut i8);
            CString::from_raw(self.region as *mut i8);
            CString::from_raw(self.comment as *mut i8);
            /*           let mut idx = 0;

            while !self.args.offset(idx).is_null() {
                Box::<*const CBasicBlockOperand>::from_raw(self.args.offset(idx) as *mut CBasicBlockOperand);
                idx += 1;
            }
            Box::from_raw(self.args as *mut *const CBasicBlockOperand);*/
        }
    }
}

#[repr(C)]
pub struct CRecentSession {
    title: *const i8,
    kind: *const i8,
    path: *const i8,
    timestamp: u32,
}

impl CRecentSession {
    pub fn new(title: String, kind: String, path: &Path, timestamp: u32) -> Result<CRecentSession> {
        let title = CString::new(title.into_bytes())?;
        let kind = CString::new(kind.into_bytes())?;
        let path = CString::new(format!("{}", path.display()).into_bytes())?;

        Ok(
            CRecentSession {
                title: title.into_raw(),
                kind: kind.into_raw(),
                path: path.into_raw(),
                timestamp: timestamp,
            }
        )
    }
}

impl Drop for CRecentSession {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self.kind as *mut i8);
            CString::from_raw(self.title as *mut i8);
            CString::from_raw(self.path as *mut i8);
        }
    }
}
