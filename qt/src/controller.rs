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
        (CREATE_RAW_PROJECT,2) => ::state::create_raw_project(&args[0],&args[1],&mut obj).to_qvariant(ret),
        (CREATE_ELF_PROJECT,1) => ::state::create_elf_project(&args[0],&mut obj).to_qvariant(ret),
        (OPEN_PROJECT,1) => ::state::open_project(&args[0],&mut obj).to_qvariant(ret),
        (SNAPSHOT_PROJECT,1) => ::state::snapshot_project(&args[0],&mut obj).to_qvariant(ret),
        (START,0) => ::state::start(&mut obj).to_qvariant(ret),
        (DONE,0) => ::state::done(&mut obj).to_qvariant(ret),

        // Stateless getter
        (FUNCTION_INFO,1) => ::function::metainfo(&args[0]).to_qvariant(ret),
        (FUNCTION_CFG,1) => ::function::control_flow_graph(&args[0]).to_qvariant(ret),
        (ALL_TARGETS,0) => ::function::targets().to_qvariant(ret),

        // Self-contained functions
        (SUGIYAMA_LAYOUT,5) => ::function::layout(&args[0],&args[1],&args[2],&args[3],&args[4],&mut obj).to_qvariant(ret),

        // Setter
        (SET_COMMENT,3) => ::function::comment(&args[0],&args[1],&args[2],&mut obj).to_qvariant(ret),
        (SET_NAME,2) => ::function::rename(&args[0],&args[1],&mut obj).to_qvariant(ret),

        _ => panic!("Unknown controller call id '{}' with {} arguments.",id,args.len())
    }
}

pub const STATE_CHANGED: isize = 0;
pub const DIRTY_CHANGED: isize = 1;

pub const DISCOVERED_FUNCTION: isize = 2;
pub const STARTED_FUNCTION: isize = 3;
pub const FINISHED_FUNCTION: isize = 4;

pub const LAYOUTED_FUNCTION: isize = 5;
pub const CHANGED_FUNCTION: isize = 6;

pub const CREATE_RAW_PROJECT: isize = 7;
pub const CREATE_ELF_PROJECT: isize = 8;

pub const OPEN_PROJECT: isize = 9;

pub const START: isize = 10;
pub const DONE: isize = 11;

pub const SET_COMMENT: isize = 12;
pub const SET_NAME: isize = 13;

pub const SNAPSHOT_PROJECT: isize = 14;

pub const FUNCTION_INFO: isize = 15;
pub const FUNCTION_CFG: isize = 16;
pub const ALL_TARGETS: isize = 17;

pub const SUGIYAMA_LAYOUT: isize = 18;

pub extern "C" fn create_singleton(_: *mut ffi::QQmlEngine, _: *mut ffi::QJSEngine) -> *mut ffi::QObject {
    let mut metaobj = MetaObject::new("Panopticon",controller_slot);

    // universial signals
    assert_eq!(metaobj.add_signal("stateChanged()"),STATE_CHANGED);
    metaobj.add_property("state","QString",Some("stateChanged()"));

    assert_eq!(metaobj.add_signal("dirtyChanged()"),DIRTY_CHANGED);
    metaobj.add_property("dirty","int",Some("dirtyChanged()"));

    // WORKING signals
    assert_eq!(metaobj.add_signal("discoveredFunction(QString)"),DISCOVERED_FUNCTION);
    assert_eq!(metaobj.add_signal("startedFunction(QString)"),STARTED_FUNCTION);
    assert_eq!(metaobj.add_signal("finishedFunction(QString)"),FINISHED_FUNCTION);

    // WORKING and DONE signals
    assert_eq!(metaobj.add_signal("layoutedFunction(QString)"),LAYOUTED_FUNCTION);
    assert_eq!(metaobj.add_signal("changedFunction(QString)"),CHANGED_FUNCTION);

    // state = NEW -> READY, dirty = -> true
    assert_eq!(metaobj.add_method("createRawProject(QString,QString)","bool"),CREATE_RAW_PROJECT);
    assert_eq!(metaobj.add_method("createElfProject(QString)","bool"),CREATE_ELF_PROJECT);
    assert_eq!(metaobj.add_method("openProject(QString)","bool"),OPEN_PROJECT);

    // state = READY -> WORKING
    assert_eq!(metaobj.add_method("start()","bool"),START);

    // state = WORKING -> DONE
    assert_eq!(metaobj.add_method("done()","void"),DONE);

    // state = (WORKING,DONE), dirty = -> true
    assert_eq!(metaobj.add_method("setComment(QString,int,QString)","QString"),SET_COMMENT);
    assert_eq!(metaobj.add_method("setName(QString,QString)","QString"),SET_NAME);

    // state = (WORKING,DONE), dirty = -> false
    assert_eq!(metaobj.add_method("snapshotProject(QString)","bool"),SNAPSHOT_PROJECT);

    // getter
    assert_eq!(metaobj.add_method("functionInfo(QString)","QString"),FUNCTION_INFO);
    assert_eq!(metaobj.add_method("functionCfg(QString)","QString"),FUNCTION_CFG);
    assert_eq!(metaobj.add_method("allTargets()","QString"),ALL_TARGETS);

    // setter
    assert_eq!(metaobj.add_method("sugiyamaLayout(QString,QString,int,int,int)","QString"),SUGIYAMA_LAYOUT);


    let mut obj = metaobj.instantiate();

    obj.set_property("state",Variant::String("NEW".to_string()));
    obj.emit(STATE_CHANGED,&[]);
    obj.set_property("dirty",Variant::I64(0));
    obj.emit(DIRTY_CHANGED,&[]);
    obj.as_ptr()
}

