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

use std::sync::{
    PoisonError,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
};
use std::collections::BTreeMap;
use std::path::{PathBuf,Path};
use std::iter::FromIterator;
use std::ops::DerefMut;
use std::fs::remove_file;
use std::error::Error;
use std::borrow::Cow;
use std::convert::Into;

#[cfg(windows)]
use std::env;

#[cfg(unix)]
use xdg::BaseDirectories;

use libc::c_int;
use qmlrs::{ffi,MetaObject,Variant,Object,ToQVariant,unpack_varlist};
use rustc_serialize::{json,Encodable};
use tempdir::TempDir;

use panopticon::project::Project;
use panopticon::result;
use panopticon::result::{
    Result,
};

use project;

extern "C" fn controller_slot(this: *mut ffi::QObject, id: c_int, a: *const ffi::QVariantList, ret: *mut ffi::QVariant) {
    let args = unpack_varlist(a);

    match (id as isize,args.len()) {
        // State transitions: NEW -> SYNC
        (CREATE_RAW_PROJECT,4) => project::create_raw_project(&args[0],&args[1],&args[2],&args[3]).to_qvariant(ret),
        (CREATE_ELF_PROJECT,1) => project::create_elf_project(&args[0]).to_qvariant(ret),
        (CREATE_PE_PROJECT,1) => project::create_pe_project(&args[0]).to_qvariant(ret),
        (OPEN_PROJECT,1) => project::open_project(&args[0]).to_qvariant(ret),

        // State transition: DIRTY -> SYNC
        (SNAPSHOT_PROJECT,1) => project::snapshot_project(&args[0]).to_qvariant(ret),

        // Getter in SYNC & DIRTY state
        (FUNCTION_INFO,1) => ::function::metainfo(&args[0]).to_qvariant(ret),
        (FUNCTION_CFG,1) => ::function::control_flow_graph(&args[0]).to_qvariant(ret),
        (FUNCTION_APPROX,1) => ::function::approximate(&args[0]).to_qvariant(ret),
        (SUGIYAMA_LAYOUT,5) => ::function::layout(&args[0],&args[1],&args[2],&args[3],&args[4]).to_qvariant(ret),

        // Stateless getter
        (ALL_TARGETS,0) => ::function::targets().to_qvariant(ret),
        (READ_DIRECTORY,1) => ::function::read_directory(&args[0]).to_qvariant(ret),
        (FILE_DETAILS,1) => ::function::file_details(&args[0]).to_qvariant(ret),

        // State transitions: SYNC -> DIRTY or DIRTY -> DIRTY
        (SET_COMMENT,3) => ::function::comment(&args[0],&args[1],&args[2]).to_qvariant(ret),
        (SET_NAME,2) => ::function::rename(&args[0],&args[1]).to_qvariant(ret),

        _ => panic!("Unknown controller call id '{}' with {} arguments.",id,args.len())
    }
}

const STATE_CHANGED: isize = 0;
const PATH_CHANGED: isize = 1;

pub const DISCOVERED_FUNCTION: isize = 2;
pub const STARTED_FUNCTION: isize = 3;
pub const FINISHED_FUNCTION: isize = 4;

pub const LAYOUTED_FUNCTION: isize = 5;
pub const CHANGED_FUNCTION: isize = 6;

pub const CREATE_RAW_PROJECT: isize = 7;
pub const CREATE_ELF_PROJECT: isize = 8;
pub const CREATE_PE_PROJECT: isize = 9;

pub const OPEN_PROJECT: isize = 10;

pub const SET_COMMENT: isize = 11;
pub const SET_NAME: isize = 12;

pub const SNAPSHOT_PROJECT: isize = 13;

pub const FUNCTION_INFO: isize = 14;
pub const FUNCTION_CFG: isize = 15;
pub const FUNCTION_APPROX: isize = 16;

pub const ALL_TARGETS: isize = 17;
pub const READ_DIRECTORY: isize = 18;
pub const FILE_DETAILS: isize = 19;

pub const SUGIYAMA_LAYOUT: isize = 20;


pub extern "C" fn create_singleton(_: *mut ffi::QQmlEngine, _: *mut ffi::QJSEngine) -> *mut ffi::QObject {
    let mut metaobj = MetaObject::new("Panopticon",controller_slot);

    // universial signals
    assert_eq!(metaobj.add_signal("stateChanged()"),STATE_CHANGED);
    metaobj.add_property("state","QString",Some("stateChanged()"));

    assert_eq!(metaobj.add_signal("savePathChanged()"),PATH_CHANGED);
    metaobj.add_property("savePath","QString",Some("savePathChanged()"));

    // WORKING signals
    assert_eq!(metaobj.add_signal("discoveredFunction(QString)"),DISCOVERED_FUNCTION);
    assert_eq!(metaobj.add_signal("startedFunction(QString)"),STARTED_FUNCTION);
    assert_eq!(metaobj.add_signal("finishedFunction(QString)"),FINISHED_FUNCTION);

    // WORKING and DONE signals
    assert_eq!(metaobj.add_signal("layoutedFunction(QString)"),LAYOUTED_FUNCTION);
    assert_eq!(metaobj.add_signal("changedFunction(QString)"),CHANGED_FUNCTION);

    // state = NEW -> READY, dirty = -> true
    assert_eq!(metaobj.add_method("createRawProject(QString,QString,int,int)","QString"),CREATE_RAW_PROJECT);
    assert_eq!(metaobj.add_method("createElfProject(QString)","QString"),CREATE_ELF_PROJECT);
    assert_eq!(metaobj.add_method("createPeProject(QString)","QString"),CREATE_PE_PROJECT);
    assert_eq!(metaobj.add_method("openProject(QString)","QString"),OPEN_PROJECT);

    // state = (WORKING,DONE), dirty = -> true
    assert_eq!(metaobj.add_method("setComment(QString,int,QString)","QString"),SET_COMMENT);
    assert_eq!(metaobj.add_method("setName(QString,QString)","QString"),SET_NAME);

    // state = (WORKING,DONE), dirty = -> false
    assert_eq!(metaobj.add_method("snapshotProject(QString)","QString"),SNAPSHOT_PROJECT);

    // getter
    assert_eq!(metaobj.add_method("functionInfo(QString)","QString"),FUNCTION_INFO);
    assert_eq!(metaobj.add_method("functionCfg(QString)","QString"),FUNCTION_CFG);
    assert_eq!(metaobj.add_method("functionApproximate(QString)","QString"),FUNCTION_APPROX);

    assert_eq!(metaobj.add_method("allTargets()","QString"),ALL_TARGETS);
    assert_eq!(metaobj.add_method("readDirectory(QString)","QString"),READ_DIRECTORY);
    assert_eq!(metaobj.add_method("fileDetails(QString)","QString"),FILE_DETAILS);

    // setter
    assert_eq!(metaobj.add_method("sugiyamaLayout(QString,QString,int,int,int)","QString"),SUGIYAMA_LAYOUT);


    let mut obj = metaobj.instantiate();

    obj.set_property("state",Variant::String("NEW".to_string()));
    obj.emit(STATE_CHANGED,&[]);
    obj.set_property("savePath",Variant::String("".to_string()));
    obj.emit(PATH_CHANGED,&[]);

    assert!(Controller::instantiate_singleton(metaobj,Object::from_ptr(obj.as_ptr())).is_ok());

    obj.as_ptr()
}

#[derive(RustcEncodable)]
pub struct Return<T: Encodable> {
    status: String,
    payload: T,
}

pub fn return_json<T: Encodable>(r: Result<T>) -> String {
    match r {
        Ok(t) => json::encode(&Return::<T>{ status: "ok".to_string(), payload: t }),
        Err(e) => json::encode(&BTreeMap::from_iter(vec![("status".to_string(),"err".to_string()),
                                                        ("error".to_string(),e.description().to_string())])),
    }.unwrap_or(format!("{{ \"status\": \"err\", \"error\": \"Failed to render JSON response\"}}"))
}

lazy_static! {
    pub static ref CONTROLLER: RwLock<Controller> = RwLock::new(Controller::Empty);
}

enum Backing {
    Unnamed(PathBuf),
    Named(PathBuf),
}

impl Backing {
    pub fn path<'a>(&'a self) -> &'a Path {
        match self {
            &Backing::Unnamed(ref p) => p.as_path(),
            &Backing::Named(ref p) => p.as_path(),
        }
    }
}

pub enum Controller {
    Empty,
    New{
        //metaObject: MetaObject,
        singletonObject: Object,
    },
    Set{
        //metaObject: MetaObject,
        singletonObject: Object,
        project: Project,
        backing_file: Backing,
        is_dirty: bool,
    },
}

pub trait ToVariant {
    fn to_variant(self) -> Variant;
}

impl ToVariant for i64 {
    fn to_variant(self) -> Variant {
        Variant::I64(self)
    }
}

impl ToVariant for String {
    fn to_variant(self) -> Variant {
        Variant::String(self)
    }
}

#[cfg(unix)]
fn session_directory() -> Result<PathBuf> {
    match BaseDirectories::with_prefix("panopticon") {
        Ok(dirs) => Ok(dirs.get_data_home().join("sessions")),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(windows)]
fn session_directory(p: &Path) -> Result<PathBuf> {
    match env::current_exe() {
        Ok(path) => Ok(Some(path.join("AppData/Local/Panopticon/Panopticon/sessions").join(p))),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

impl Controller {
    pub fn instantiate_singleton(_: MetaObject, s: Object) -> Result<()> {
        {
            let mut guard = try!(CONTROLLER.write());
            let is_empty = if let &Controller::Empty = &*guard { true } else { false };

            if is_empty {
                *guard = Controller::New{
                    //metaObject: m,
                    singletonObject: s,
                };

                Ok(())
            } else {
                Err("Tried to instantiate another singleton".into())
            }
        }.and_then(|_| {
            Controller::update_state()
        })
    }

    pub fn read<A,F: FnOnce(&Project) -> A>(f: F) -> Result<A> {
        let guard = try!(CONTROLLER.read());
        if let &Controller::Set{ ref project,.. } = &*guard {
            Ok(f(project))
        } else {
            Err("Controller in wrong state (read)".into())
        }
    }

    pub fn modify<A,F: FnOnce(&mut Project) -> A>(f: F) -> Result<A> {
        {
            let mut guard = try!(CONTROLLER.write());
            if let &mut Controller::Set{ ref mut project, ref mut is_dirty,.. } = &mut *guard {
                let ret: Result<A> = Ok(f(project));

                *is_dirty = true;
                ret
            } else {
                Err("Controller in wrong state (modify)".into())
            }
        }.and_then(|a| {
            try!(Controller::update_state());
            Ok(a)
        })
    }

    pub fn sync() -> Result<()> {
        {
            let mut guard = try!(CONTROLLER.write());
            if let &mut Controller::Set{ ref mut project, ref mut is_dirty, ref backing_file,.. } = &mut *guard {
                try!(project.snapshot(&backing_file.path()));
                *is_dirty = false;
                Ok(())
            } else {
                Err("Controller in wrong state (sync)".into())
            }
        }.and_then(|_| {
            Controller::update_state()
        })
    }

    pub fn replace(p: Project,q: Option<&Path>) -> Result<()> {
        {
            let mut obj = try!(Controller::instance());
            let mut guard = try!(CONTROLLER.write());

            *guard = Controller::New{ singletonObject: obj };
            Ok(())
        }.and_then(|_| {
            Controller::update_state()
        }).and_then(|_| {
            let mut guard = try!(CONTROLLER.write());
            let bf = if let Some(p) = q {
                Backing::Named(p.to_path_buf())
            } else {
                Backing::Unnamed(try!(TempDir::new_in(try!(session_directory()),"panop-backing")).path().to_path_buf())
            };

            match &mut *guard {
                &mut Controller::Set{ ref mut project, ref mut is_dirty, ref mut backing_file,.. } => {
                    *project = p;
                    *is_dirty = false;
                    *backing_file = bf;
                    Ok(())
                },
                ctrl@&mut Controller::New{ ..} => {
                    let so = if let &mut Controller::New{ /*ref metaObject,*/ ref mut singletonObject,.. } = ctrl {
                        singletonObject.as_ptr()
                    } else {
                        unreachable!()
                    };

                    *ctrl = Controller::Set{
                        //metaObject: metaObject,
                        singletonObject: Object::from_ptr(so),
                        project: p,
                        is_dirty: false,
                        backing_file: bf,
                    };
                    Ok(())
                },
                &mut Controller::Empty => {
                    Err("Controller is in empty state".into())
                }
            }
        }).and_then(|_| {
            Controller::update_state()
        })
    }

    pub fn set_backing(p: &Path) -> Result<()> {
        {
            let mut guard = try!(CONTROLLER.write());
            if let &mut Controller::Set{ ref mut is_dirty, ref mut backing_file,.. } = &mut *guard {
                if let &mut Backing::Unnamed(ref p) = backing_file {
                    remove_file(p);
                }
                *backing_file = Backing::Named(p.to_path_buf());
                *is_dirty = true;
                Ok(())
            } else {
                Err("Controller is in empty state".into())
            }
        }.and_then(|_| {
            Controller::update_state()
        })
    }

    pub fn emit0(s: isize) -> Result<()> {
        let guard = try!(CONTROLLER.read());

        match &*guard {
            &Controller::New{ ref singletonObject } => singletonObject.emit(s,&[]),
            &Controller::Set{ ref singletonObject,.. } => singletonObject.emit(s,&[]),
            &Controller::Empty => return Err("Controller is in empty state".into()),
        }

        Ok(())
    }

    pub fn emit1<A: ToVariant + Clone>(s: isize, a: &A) -> Result<()> {
        let guard = try!(CONTROLLER.read());

        match &*guard {
            &Controller::New{ ref singletonObject } => singletonObject.emit(s,&[a.clone().to_variant()]),
            &Controller::Set{ ref singletonObject,.. } => singletonObject.emit(s,&[a.clone().to_variant()]),
            &Controller::Empty => return Err("Controller is in empty state".into()),
        }

        Ok(())
    }

    fn instance() -> Result<Object> {
        let mut guard = try!(CONTROLLER.write());
        match &mut *guard {
            &mut Controller::Empty => Err("Controller is in empty state".into()),
            &mut Controller::New{ ref mut singletonObject,.. } => Ok(Object::from_ptr(singletonObject.as_ptr())),
            &mut Controller::Set{ ref mut singletonObject,.. } => Ok(Object::from_ptr(singletonObject.as_ptr())),
        }
    }

    fn update_state() -> Result<()> {
        let mut obj = try!(Controller::instance());
        let mut guard = try!(CONTROLLER.write());
        let (nstate,nback) = match &mut *guard {
            &mut Controller::Empty => ("".to_string(),"".to_string()),
            &mut Controller::New{ .. } => ("NEW".to_string(),"".to_string()),
            &mut Controller::Set{ is_dirty: true, ref backing_file,.. } =>
                ("DIRTY".to_string(),backing_file.path().to_str().unwrap_or("").to_string()),
            &mut Controller::Set{ is_dirty: false, ref backing_file,.. } =>
                ("SYNC".to_string(),backing_file.path().to_str().unwrap_or("").to_string()),
        };
        let state_changed = if let Variant::String(ref s) = obj.get_property("state") {
            *s != nstate
        } else {
            true
        };
        let back_changed = if let Variant::String(ref s) = obj.get_property("savePath") {
            *s != nback
        } else {
            true
        };

        obj.set_property("state",Variant::String(nstate));
        if state_changed {
            obj.emit(STATE_CHANGED,&[]);
        }

        obj.set_property("savePath",Variant::String(nback));
        if back_changed {
            obj.emit(PATH_CHANGED,&[]);
        }

        Ok(())
    }
}
