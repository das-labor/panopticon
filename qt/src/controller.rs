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

        _ => panic!("Unknown controller call id '{}' with {} arguments.",id,args.len())
    }
}

pub const STATE_CHANGED: isize = 0;

pub const DISCOVERED_FUNCTION: isize = 1;
pub const STARTED_FUNCTION: isize = 2;
pub const FINISHED_FUNCTION: isize = 3;
pub const LAYOUTED_FUNCTION: isize = 4;
pub const CHANGED_FUNCTION: isize = 5;

pub const CREATE_AVR_SESSION: isize = 6;
pub const CREATE_RAW_SESSION: isize = 7;
pub const OPEN_SESSION: isize = 8;

pub const START: isize = 9;
pub const DONE: isize = 10;

pub const FUNCTION_INFO: isize = 11;
pub const FUNCTION_CFG: isize = 12;

pub const SUGIYAMA_LAYOUT: isize = 13;

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

    // self-contained
    assert_eq!(metaobj.add_method("sugiyamaLayout(QString,QString,int,int)","QString"),SUGIYAMA_LAYOUT);

    let mut obj = metaobj.instantiate();

    obj.set_property("state",Variant::String("NEW".to_string()));
    obj.emit(0,&[]);
    obj.as_ptr()
}

