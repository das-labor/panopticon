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

use std::ffi::{CString,CStr};
use singleton::{
    NodePosition,
    EdgePosition,
    PANOPTICON
};
use panopticon::{
    Function,
};
use std::ptr;
use std::path::{Path,PathBuf};
use errors::*;
use futures::{
    future,
    Future,
};
use futures_cpupool::CpuPool;
use parking_lot::Mutex;
use uuid::Uuid;

lazy_static! {
    pub static ref LAYOUT_TASK: Mutex<future::BoxFuture<(),Error>> = {
        Mutex::new(future::ok(()).boxed())
    };

    pub static ref THREAD_POOL: Mutex<CpuPool> = {
        Mutex::new(CpuPool::new_num_cpus())
    };
}

fn transform_nodes(only_entry: bool, nodes: Vec<NodePosition>) -> Vec<(usize,f32,f32,bool,Vec<CBasicBlockLine>)> {
    nodes.into_iter()
        .filter_map(|x| {
            if !only_entry || x.3 { Some(x) } else { None }
        })
    .map(|(id,x,y,is_entry,blk)| {
        let blk = blk.into_iter()
            .filter_map(|bbl| {
                let args = bbl.args.into_iter().filter_map(|x| {
                    CBasicBlockOperand::new(x.kind.to_string(),x.display,x.alt,x.data).ok()
                }).collect::<Vec<_>>();
                CBasicBlockLine::new(bbl.opcode,bbl.region,bbl.offset,bbl.comment,args).ok()
            })
        .collect::<Vec<_>>();
        (id,x,y,is_entry,blk)
    }).collect()
}

fn transform_edges(edges: Vec<EdgePosition>) -> (Vec<u32>,Vec<CString>,Vec<CString>,Vec<f32>,Vec<f32>,Vec<f32>,Vec<f32>,CString) {
    use std::f32;

    let edges = edges
        .into_iter()
        .map(|(id,kind,label,(head_x,head_y),(tail_x,tail_y),segs)| {
            let segs = segs.iter();
            let f = |&(x,y,_,_)| (x,y);
            let g = |&(_,_,x,y)| (x,y);
            let mut min_x = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut min_y = f32::INFINITY;
            let mut max_y = f32::NEG_INFINITY;
            let svg = if let Some(&(x,y,_,_)) = segs.clone().next() {
                let mut edge = format!("M {} {}",x,y);

                if min_x > x { min_x = x }
                if max_x < x + 1. { max_x = x + 1. }
                if min_y > y { min_y = y }
                if max_y < y + 1. { max_y = y + 1. }

                for (x,y) in segs.clone().take(1).map(&f).chain(segs.clone().map(&g)) {
                    edge = format!("{} L {} {}",edge,x,y);
                    if min_x > x { min_x = x }
                    if max_x < x + 1. { max_x = x + 1. }
                    if min_y > y { min_y = y }
                    if max_y < y + 1. { max_y = y + 1. }
                }

                let color = if kind == "fallthru" || kind == "fallthru-backedge" {
                    "red"
                } else if kind == "branch" || kind == "branch-backedge" {
                    "green"
                } else {
                    "black"
                };

                let arrow = if let Some(&(_,_,x,y)) = segs.clone().rev().next() {
                    let width = 12.;
                    let height = 8.;

                    format!("M {} {} L {} {} L {} {} L {} {} Z",
                            x,y,
                            x - width / 2.,y - height / 2.,
                            x,y + height,
                            x + width / 2.,y - height / 2.)
                } else {
                    "".to_string()
                };
                format!("
    <path style='fill:none; stroke:{}; stroke-width:2' d='{}'/>
    <path style='fill:{}; stroke-width:0' d='{}'/>\n",color,edge,color,arrow)
            } else {
                "".to_string()
            };

            let label = CString::new(label.as_bytes()).unwrap();
            let kind = CString::new(kind.as_bytes()).unwrap();

            (id as u32,label,kind,head_x,head_y,tail_x,tail_y,svg,min_x,max_x,min_y,max_y)
        });

    let mut head_xs = vec![];
    let mut head_ys = vec![];
    let mut tail_xs = vec![];
    let mut tail_ys = vec![];
    let mut ids = vec![];
    let mut labels = vec![];
    let mut kinds = vec![];
    let mut svg = "".to_string();
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for (id,label,kind,head_x,head_y,tail_x,tail_y,path,min_x_,max_x_,min_y_,max_y_) in edges {
        head_xs.push(head_x);
        head_ys.push(head_y);
        tail_xs.push(tail_x);
        tail_ys.push(tail_y);
        ids.push(id);
        labels.push(label);
        kinds.push(kind);
        svg += &path;

        if min_x > min_x_ { min_x = min_x_ }
        if max_x < max_x_ { max_x = max_x_ }
        if min_y > min_y_ { min_y = min_y_ }
        if max_y < max_y_ { max_y = max_y_ }
    }

    svg = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' viewBox='0 0 {} {}'>\n{}</svg>",
                  max_x + 10.,max_y + 10.,max_x + 10.,max_y + 10.,svg);

    (ids,labels,kinds,head_xs,head_ys,tail_xs,tail_ys,CString::new(svg.as_bytes()).unwrap())
}

fn transform_and_send_function(uuid: &Uuid, only_entry: bool, do_nodes: bool, do_edges: bool) -> future::BoxFuture<(),Error> {
    let uuid = uuid.clone();

    PANOPTICON
        .lock()
        .layout_function_async(&uuid)
        .and_then(move |(nodes,edges)| {
            let uuid = uuid;
            let uuid = CString::new(uuid.clone().to_string().as_bytes()).unwrap();

            if do_nodes {
                let nodes = transform_nodes(only_entry,nodes);

                for (id,x,y,is_entry,bbl) in nodes {
                    send_function_node(uuid.clone(),id,x,y,is_entry,bbl.as_slice()).unwrap();
                }
            }

            if do_edges {
                let (ids,labels,kinds,head_xs,head_ys,tail_xs,tail_ys,svg) = transform_edges(edges);
                send_function_edges(
                    uuid,ids.as_slice(),
                    labels.as_slice(),kinds.as_slice(),
                    head_xs.as_slice(),head_ys.as_slice(),
                    tail_xs.as_slice(),tail_ys.as_slice(),
                    svg).unwrap();
            }

            future::ok(())
        }).boxed()
}

pub extern "C" fn get_function(uuid_cstr: *const i8, only_entry: i8, do_nodes: i8, do_edges: i8) -> i32 {
    let uuid = unsafe { CStr::from_ptr(uuid_cstr) }.to_string_lossy().to_string();
    let uuid = match Uuid::parse_str(&uuid) {
        Ok(uuid) => uuid,
        Err(s) => { error!("get_function(): {}",s); return -1; }
    };

    unsafe { update_layout_task(uuid_cstr); }

    let task = transform_and_send_function(&uuid,only_entry != 0,do_nodes != 0,do_edges != 0)
        .then(|x| {
            let uuid = CString::new("".to_string().as_bytes()).unwrap();
            unsafe { update_layout_task(uuid.as_ptr()); }

            future::result(x)
        });
    let task = { THREAD_POOL.lock().spawn(task) };
    *LAYOUT_TASK.lock() = task.boxed();

    0
}

pub extern "C" fn open_program(path: *const i8) -> i32 {
    let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
    match PANOPTICON.lock().open_program(path) {
        Ok(()) => 0,
        Err(s) => { error!("open_program(): {}",s); -1 }
    }
}

pub extern "C" fn save_session(path: *const i8) -> i32 {
    let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
    match PANOPTICON.lock().save_session(path) {
        Ok(()) => 0,
        Err(s) => { error!("save_session(): {}",s); -1 }
    }
}

pub extern "C" fn comment_on(address: u64, comment: *const i8) -> i32 {
    let comment = unsafe { CStr::from_ptr(comment) }.to_string_lossy().to_string();
    match PANOPTICON.lock().comment_on(address,comment) {
        Ok(()) => 0,
        Err(s) => { error!("comment_on(): {}",s); -1 }
    }
}

pub extern "C" fn rename_function(uuid: *const i8, name: *const i8) -> i32 {
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy().to_string();
    let uuid = unsafe { CStr::from_ptr(uuid) }.to_string_lossy().to_string();
    match PANOPTICON.lock().rename_function(uuid,name) {
        Ok(()) => 0,
        Err(s) => { error!("rename_function(): {}",s); -1 }
    }
}

pub extern "C" fn set_value_for(uuid: *const i8, variable: *const i8, value: *const i8) -> i32 {
    let uuid = unsafe { CStr::from_ptr(uuid) }.to_string_lossy().to_string();
    let variable = unsafe { CStr::from_ptr(variable) }.to_string_lossy().to_string();
    let value = unsafe { CStr::from_ptr(value) }.to_string_lossy().to_string();
    match PANOPTICON.lock().set_value_for(uuid,variable,value) {
        Ok(()) => 0,
        Err(s) => { error!("set_value_for(): {}",s); -1 }
    }
}

pub extern "C" fn undo() -> i32 {
    match PANOPTICON.lock().undo() {
        Ok(()) => 0,
        Err(s) => { error!("undo(): {}",s); -1 }
    }
}

pub extern "C" fn redo() -> i32 {
    match PANOPTICON.lock().redo() {
        Ok(()) => 0,
        Err(s) => { error!("redo(): {}",s); -1 }
    }
}

#[repr(C)]
struct CSidebarItem {
    title: *const i8,
    subtitle: *const i8,
    uuid: *const i8,
}

impl CSidebarItem {
    pub fn new(func: &Function) -> Result<CSidebarItem> {
        use graph_algos::GraphTrait;
        use panopticon::ControlFlowTarget;

        let cfg = &func.cflow_graph;
        let entry = func.entry_point.
            and_then(|vx| cfg.vertex_label(vx)).
            and_then(|lb| {
                if let &ControlFlowTarget::Resolved(ref bb) = lb {
                    Some(bb.area.start)
                } else {
                    None
                }
            });
        let str_entry = CString::new(entry.map(|x| format!("0x{:x}",x)).unwrap_or("".to_string()).into_bytes())?;
        let name = CString::new(func.name.to_string().into_bytes())?;
        let uuid = CString::new(func.uuid.to_string().into_bytes())?;

        Ok(CSidebarItem{
            title: name.into_raw(),
            subtitle: str_entry.into_raw(),
            uuid: uuid.into_raw(),
        })
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

        Ok(CBasicBlockOperand{
            kind: kind.into_raw(),
            display: display.into_raw(),
            alt: alt.into_raw(),
            data: data.into_raw(),
        })
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

        Ok(CBasicBlockLine{
            opcode: opcode.into_raw(),
            region: region.into_raw(),
            offset: offset,
            comment: comment.into_raw(),
            args: unsafe { (*Box::into_raw(Box::new(args))).as_ptr() },
        })
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
        let path = CString::new(format!("{}",path.display()).into_bytes())?;

        Ok(CRecentSession{
            title: title.into_raw(),
            kind: kind.into_raw(),
            path: path.into_raw(),
            timestamp: timestamp,
        })
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
extern "C" {
    fn start_gui_loop(
        qml_dir: *const i8,
        inital_file: *const i8,
        recent_sessions: *const *const CRecentSession,
        get_function: extern "C" fn(*const i8,i8,i8,i8) -> i32,
        open_program: extern "C" fn(*const i8) -> i32,
        save_session: extern "C" fn(*const i8) -> i32,
        comment_on: extern "C" fn(u64, *const i8) -> i32,
        rename_function: extern "C" fn(*const i8,*const i8) -> i32,
        set_value_for: extern "C" fn(*const i8,*const i8,*const i8) -> i32,
        undo: extern "C" fn() -> i32,
        redo: extern "C" fn() -> i32);

    // thread-safe
    fn update_function_node(uuid: *const i8, id: u32, x: f32, y: f32, is_entry: i8, lines: *const *const CBasicBlockLine);

    // thread-safe
    fn update_function_edges(uuid: *const i8, ids: *const u32, labels: *const *const i8,
                             kinds: *const *const i8,
                             head_xs: *const f32, head_ys: *const f32,
                             tail_xs: *const f32, tail_ys: *const f32,svg: *const i8);

    // thread-safe
    fn update_sidebar_items(items: *const *const CSidebarItem);

    // thread-safe
    fn update_undo_redo(undo: i8, redo: i8);

    // thread-safe
    fn update_current_session(path: *const i8);

    // thread-safe
    fn update_layout_task(task: *const i8);
}

pub fn exec(qml_dir: &Path, initial_file: Option<String>, recent_sessions: Vec<(String,String,PathBuf,u32)>) -> Result<()> {
    let qml_dir = CString::new(format!("{}",qml_dir.display()).as_bytes())?;
    let initial_file = CString::new(initial_file.unwrap_or_default())?;
    let recent_sessions = recent_sessions.into_iter().filter_map(|x| CRecentSession::new(x.0,x.1,&x.2,x.3).ok()).collect::<Vec<_>>();
    let mut recent_sess_ptrs: Vec<*const CRecentSession> = recent_sessions.iter().map(|x| -> *const CRecentSession { x }).collect();

    recent_sess_ptrs.push(ptr::null());

    unsafe {
        start_gui_loop(
            qml_dir.as_ptr(),
            initial_file.as_ptr(),
            recent_sess_ptrs.as_ptr(),
            get_function,
            open_program,
            save_session,
            comment_on,
            rename_function,
            set_value_for,
            undo,
            redo);
    }

    Ok(())
}

pub fn update_sidebar(funcs: &[Function]) {
    let items = funcs.iter().filter_map(|f| CSidebarItem::new(f).ok()).collect::<Vec<_>>();
    let mut ptrs: Vec<*const CSidebarItem> = items.iter().map(|i| -> *const CSidebarItem { i }).collect();

    ptrs.push(ptr::null());
    unsafe { update_sidebar_items(ptrs.as_slice().as_ptr()); }
}

pub fn send_function_node(uuid: CString, id: usize, x: f32, y: f32, is_entry: bool, lines: &[CBasicBlockLine]) -> Result<()> {
    let mut ptrs: Vec<*const CBasicBlockLine> = lines.iter().map(|i| -> *const CBasicBlockLine { i }).collect();

    ptrs.push(ptr::null());
    unsafe {
        update_function_node(
            uuid.as_ptr(),
            id as u32,
            x,
            y,
            if is_entry { 1 } else { 0 },
            ptrs.as_slice().as_ptr());
    }

    Ok(())
}

pub fn send_function_edges(uuid: CString, ids: &[u32], labels: &[CString], kinds: &[CString],
                           head_xs: &[f32], head_ys: &[f32],
                           tail_xs: &[f32], tail_ys: &[f32], svg: CString) -> Result<()> {
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
            svg.as_ptr())
    }

    Ok(())
}

pub fn send_undo_redo_update(undo: bool, redo: bool) -> Result<()> {
    unsafe {
        update_undo_redo(
            if undo { 1 } else { 0 },
            if redo { 1 } else { 0 });
    }

    Ok(())
}

pub fn send_current_session(path: CString) -> Result<()> {
    unsafe {
        update_current_session(path.as_ptr());
    }

    Ok(())
}
