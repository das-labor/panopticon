/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
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

use libc::c_int;
use std::sync::RwLock;

use panopticon::project::Project;
use qmlrs::{ffi,MetaObject,Variant,Object,ToQVariant,unpack_varlist};

lazy_static! {
    pub static ref PROJECT: RwLock<Option<Project>> = RwLock::new(None);
}

extern "C" fn controller_slot(this: *mut ffi::QObject, id: c_int, a: *const ffi::QVariantList, ret: *mut ffi::QVariant) {
    let mut obj = Object::from_ptr(this);
    let args = unpack_varlist(a);

    match (id as isize,args.len()) {
        // State transitions
        (CREATE_AVR_SESSION,1) => ::state::create_avr_session(&args[0],&mut obj).to_qvariant(ret),
        (CREATE_RAW_SESSION,1) => ::state::create_raw_session(&args[0],&mut obj).to_qvariant(ret),
        (OPEN_SESSION,1) => ::state::open_session(&args[0],&mut obj).to_qvariant(ret),
        (START,0) => ::state::start(&mut obj).to_qvariant(ret),
        (DONE,0) => ::state::done(&mut obj).to_qvariant(ret),

        // Stateless getter
        (FUNCTION_INFO,1) => ::function::metainfo(&args[0]).to_qvariant(ret),
        (FUNCTION_CFG,1) => ::function::control_flow_graph(&args[0]).to_qvariant(ret),

        // Self-contained functions
        (SUGIYAMA_LAYOUT,4) => ::function::layout(&args[0],&args[1],&args[2],&args[3],&mut obj).to_qvariant(ret),
        (DIJKSTRA_ROUTE,1) => ::function::route(&args[0],&mut obj).to_qvariant(ret),

        // Setter
        (SET_COMMENT,3) => ::function::comment(&args[0],&args[1],&args[2],&mut obj).to_qvariant(ret),
        (SET_NAME,2) => ::function::rename(&args[0],&args[1],&mut obj).to_qvariant(ret),

        _ => panic!("Unknown controller call id '{}' with {} arguments.",id,args.len())
    }
}

pub const STATE_CHANGED: isize = 0;

pub const DISCOVERED_FUNCTION: isize = 1;
pub const STARTED_FUNCTION: isize = 2;
pub const FINISHED_FUNCTION: isize = 3;
pub const LAYOUTED_FUNCTION: isize = 4;
pub const ROUTED_FUNCTION: isize = 5;
pub const CHANGED_FUNCTION: isize = 6;

pub const CREATE_AVR_SESSION: isize = 7;
pub const CREATE_RAW_SESSION: isize = 8;
pub const OPEN_SESSION: isize = 9;

pub const START: isize = 10;
pub const DONE: isize = 11;

pub const FUNCTION_INFO: isize = 12;
pub const FUNCTION_CFG: isize = 13;

pub const SUGIYAMA_LAYOUT: isize = 14;
pub const DIJKSTRA_ROUTE: isize = 15;
pub const SET_COMMENT: isize = 16;
pub const SET_NAME: isize = 17;

pub extern "C" fn create_singleton(_: *mut ffi::QQmlEngine, _: *mut ffi::QJSEngine) -> *mut ffi::QObject {
    let mut metaobj = MetaObject::new("Panopticon",controller_slot);

    // universial signals
    assert_eq!(metaobj.add_signal("stateChanged()"),STATE_CHANGED);
    metaobj.add_property("state","QString",Some("stateChanged()"));

    // WORKING signals
    assert_eq!(metaobj.add_signal("discoveredFunction(QString)"),DISCOVERED_FUNCTION);
    assert_eq!(metaobj.add_signal("startedFunction(QString)"),STARTED_FUNCTION);
    assert_eq!(metaobj.add_signal("finishedFunction(QString)"),FINISHED_FUNCTION);

    // WORKING and DONE signals
    assert_eq!(metaobj.add_signal("layoutedFunction(QString)"),LAYOUTED_FUNCTION);
    assert_eq!(metaobj.add_signal("routedFunction(QString)"),ROUTED_FUNCTION);
    assert_eq!(metaobj.add_signal("changedFunction(QString)"),CHANGED_FUNCTION);

    // state = NEW -> READY
    assert_eq!(metaobj.add_method("createAvrSession(QString)","bool"),CREATE_AVR_SESSION);
    assert_eq!(metaobj.add_method("createRawSession(QString)","bool"),CREATE_RAW_SESSION);
    assert_eq!(metaobj.add_method("openSession(QString)","bool"),OPEN_SESSION);

    // state = READY -> WORKING
    assert_eq!(metaobj.add_method("start()","bool"),START);

    // state = WORKING -> DONE
    assert_eq!(metaobj.add_method("done()","void"),DONE);

    // getter
    assert_eq!(metaobj.add_method("functionInfo(QString)","QString"),FUNCTION_INFO);
    assert_eq!(metaobj.add_method("functionCfg(QString)","QString"),FUNCTION_CFG);

    // setter
    assert_eq!(metaobj.add_method("sugiyamaLayout(QString,QString,int,int)","QString"),SUGIYAMA_LAYOUT);
    assert_eq!(metaobj.add_method("dijkstraRoute(QString)","QString"),DIJKSTRA_ROUTE);
    assert_eq!(metaobj.add_method("setComment(QString,int,QString)","QString"),SET_COMMENT);
    assert_eq!(metaobj.add_method("setName(QString,QString)","QString"),SET_NAME);

    let mut obj = metaobj.instantiate();

    obj.set_property("state",Variant::String("NEW".to_string()));
    obj.emit(0,&[]);
    obj.as_ptr()
}

