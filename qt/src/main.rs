extern crate panopticon;
extern crate qmlrs;
extern crate libc;
extern crate graph_algos;

#[macro_use]
extern crate lazy_static;

use libc::c_int;
use std::sync::RwLock;
use std::path::Path;
use std::thread;

use panopticon::project::Project;
use panopticon::region::Region;
use panopticon::program::DisassembleEvent;
use panopticon::avr;

use graph_algos::traits::Graph;
use qmlrs::{ffi,MetaObject,Variant,Object,ToQVariant};

/*
 * --> Virgin --[ create/open Session ]--> Ready --[ start() ]--> Working --> Done -.
 *        ^-----------------------[ close() ]---------------------------------------'
 * Ready: emit discoverFunction, startFunction, doneFunction
 */

lazy_static! {
    static ref PROJECT: RwLock<Option<Project>> = RwLock::new(None);
}

extern "C" fn panopticon_slot(this: *mut ffi::QObject, id: libc::c_int, a: *const ffi::QVariantList, ret: *mut ffi::QVariant) {
    let mut obj = Object::from_ptr(this);
    let args = qmlrs::unpack_varlist(a);
    let state = obj.get_property("state");

    let new = Variant::String("NEW".to_string());
    let ready = Variant::String("READY".to_string());
    let working = Variant::String("WORKING".to_string());

    match (id,state,args.len()) {
        (4...6,new,1) => {
            if let Variant::String(ref s) = args[0] {
                let p = if id == 6 {
                    Project::open(&Path::new(s))
                } else {
                    Some(Project::new("".to_string(),Region::open("".to_string(),&Path::new(s)).unwrap()))
                };

                if p.is_some() {
                    *PROJECT.write().unwrap() = p;
                    obj.set_property("state",Variant::String("READY".to_string()));
                    obj.emit(0,&[]);
                    true.to_qvariant(ret);
                } else {
                    false.to_qvariant(ret);
                }
            } else {
                false.to_qvariant(ret);
            }
        },
        (7,ready,0) => {
            let guard = PROJECT.read().unwrap();
            let maybe_project = guard;

            if !maybe_project.is_some() {
                false.to_qvariant(ret);
            } else {

            obj.set_property("state",Variant::String("WORKING".to_string()));
            obj.emit(0,&[]);

            thread::spawn(move || {
                let prog = {
                    let read_guard = PROJECT.read().unwrap();
                    let pro: &Project = read_guard.as_ref().unwrap();
                    let i = pro.sources.dependencies.vertex_label(pro.sources.root).unwrap().iter();

                    avr::disassemble(avr::Mcu::new(),i,Some(|e| {
                        match e {
                            DisassembleEvent::Discovered(pos) => {
                                obj.emit(1,&vec!(Variant::I64(pos as i64)));
                            },
                            DisassembleEvent::Started(pos) => {
                                obj.emit(2,&vec!(Variant::I64(pos as i64)));
                            },
                            DisassembleEvent::Done(pos) => {
                                obj.emit(3,&vec!(Variant::I64(pos as i64)));
                            },
                        }
                    }))
                };

                {
                    let mut write_guard = PROJECT.write().unwrap();
                    let pro: &mut Project = write_guard.as_mut().unwrap();
                    pro.code.push(prog);
                }

                obj.call(8,&[]);
            });
            true.to_qvariant(ret);
            }
        },
        (8,working,0) => {
            obj.set_property("state",Variant::String("DONE".to_string()));
            obj.emit(0,&[]);
        },
        _ => panic!("unknown Panopticon type call error (id: {}, args: {})",id,args.len())
    }
}

extern "C" fn create_panopticon_singleton(_: *mut qmlrs::ffi::QQmlEngine, _: *mut qmlrs::ffi::QJSEngine) -> *mut qmlrs::ffi::QObject {
    let mut metaobj = MetaObject::new("Panopticon",panopticon_slot);

    assert_eq!(metaobj.add_signal("stateChanged()"),0);
    metaobj.add_property("state","QString",Some("stateChanged()"));

    // READY signals
    assert_eq!(metaobj.add_signal("discoveredFunction(qlonglong)"),1);
    assert_eq!(metaobj.add_signal("startedFunction(qlonglong)"),2);
    assert_eq!(metaobj.add_signal("finishedFunction(qlonglong)"),3);

    // state = NEW -> READY
    assert_eq!(metaobj.add_method("createAvrSession(QString)","bool"),4);
    assert_eq!(metaobj.add_method("createRawSession(QString)","bool"),5);
    assert_eq!(metaobj.add_method("openSession(QString)","bool"),6);

    // state = READY -> WORKING
    assert_eq!(metaobj.add_method("start()","bool"),7);

    // state = WORKING -> DONE
    assert_eq!(metaobj.add_method("done()","void"),8);

    // getter
    assert_eq!(metaobj.add_method("functionInfo(qlonglong)","QString"),9);

    let mut obj = metaobj.instantiate();
    obj.as_ptr()
}

pub fn main() {
    qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_panopticon_singleton);

    let mut engine = qmlrs::Engine::new();
    engine.load_local_file("qt/res/Window.qml");
    engine.exec();
}
