extern crate cmake;
extern crate pkg_config;

use std::env;
use std::env::consts;
use std::path::PathBuf;

fn build_dos(cmake_cfg: &mut cmake::Config) {
    let dst = cmake_cfg.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}", dst.join("build").join("lib").join("Release").display());
    } else {
        println!("cargo:rustc-link-search=native={}", dst.join("build").join("lib").display());
    }

    println!("cargo:rustc-link-lib=static=panopticon-glue");
}

fn find_qt5(_: &mut cmake::Config) {
    /*
     * Support Qt installed via the Ports system on BSD-like systems.
     *
     * The native libs are in `/usr/local/lib`, which is not linked against by default.
     * This means that either the user or every package has to add this if they want to link
     * against something that is not part of the core distribution in `/usr/lib`.
     *
     * See https://wiki.freebsd.org/WarnerLosh/UsrLocal for the line of reasoning & how this will
     * change in the future.
     */
    if cfg!(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd",
                target_os = "dragonfly", target_os = "bitrig")) {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    /*
     * Parameters for supporting QT on OS X
     *
     * Because QT5 conflicts with QT4 the homebrew package manager won't link
     * the QT5 package into the default search paths for libraries, to deal
     * with this we need to give pkg-config and cmake a nudge in the right
     * direction.
     */
    if cfg!(target_os = "macos") {
        // We use the QTDIR or QTDIR64 env variables to find the location of
        // Qt5. If these are not set, we use the default homebrew install
        // location.
        let qtdir_variable = match consts::ARCH {
            "x86_64" => "QTDIR64",
            _ => "QTDIR",
        };
        let mut qt5_lib_path = PathBuf::new();
        qt5_lib_path.push(env::var(qtdir_variable).unwrap_or(String::from("/usr/local/opt/qt5")));
        qt5_lib_path.push("lib");

        if qt5_lib_path.exists() {
            // First nudge cmake in the direction of the .cmake files added by
            // homebrew. This clobbers the existing value if present, it's
            // unlikely to be present though.
            env::set_var("CMAKE_PREFIX_PATH", qt5_lib_path.join("cmake"));

            // Nudge pkg-config in the direction of the brewed QT to ensure the
            // correct compiler flags get found for the project.
            env::set_var("PKG_CONFIG_PATH", qt5_lib_path.join("pkgconfig"));
        } else {
            panic!("QT5 was not found at the expected location ({}) please install it via homebrew, or set the {} env variable.",
            qt5_lib_path.display(), qtdir_variable);
        }
    }

    if cfg!(windows) {
        let mut qt5_lib_path = PathBuf::new();
        qt5_lib_path.push(env::var("QTDIR").unwrap_or(String::from("C:\\Qt\\5.7\\msvc2015_64")));

        if qt5_lib_path.exists() {
            env::set_var("CMAKE_PREFIX_PATH", &qt5_lib_path);

            qt5_lib_path.push("lib");

            println!("cargo:rustc-link-search=native={}\\system32",env::var("WINDIR").unwrap());
            println!("cargo:rustc-link-search=native={}",qt5_lib_path.display());

            println!("cargo:rustc-link-lib=dylib=Qt5Core");
            println!("cargo:rustc-link-lib=dylib=Qt5Gui");
            println!("cargo:rustc-link-lib=dylib=Qt5Qml");
            println!("cargo:rustc-link-lib=dylib=Qt5Quick");
            println!("cargo:rustc-link-lib=dylib=Qt5Svg");
            println!("cargo:rustc-link-lib=dylib=Qt5Widgets");
        } else {
            panic!("QT5 was not found at the expected location ({}) please install the SDK or set the QTDIR env variable.",
            qt5_lib_path.display());
        }
    } else {
        use pkg_config;

        if cfg!(any(target_os = "macos", target_os = "freebsd", target_os = "bitrig")) {
            println!("cargo:rustc-link-lib=dylib=c++");
        } else {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }

        match pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick Qt5Svg") {
            Ok(lib) => {
                for p in lib.link_paths { println!("cargo:rustc-link-search=native={}",p.display()); }
                for p in lib.libs { println!("cargo:rustc-link-lib=dylib={}",p); }
                if cfg!(target_os = "macos") {
                    for p in lib.framework_paths { println!("cargo:rustc-link-search=framework={}",p.display()); }
                    for p in lib.frameworks { println!("cargo:rustc-link-lib=framework={}",p); }
                }
                for p in lib.include_paths { println!("cargo:include={}",p.display()); }
            }
            Err(e) => panic!("QT5 was not found using pkg-config: {}",e)
        }
    }
}

fn main() {
    let mut cmake_cfg = cmake::Config::new("glue");
    let dos_path = PathBuf::from("glue").join("CMakeLists.txt");

    if let Ok(gen) = env::var("CMAKE_GENERATOR") {
        cmake_cfg.generator(gen);
    }

    find_qt5(&mut cmake_cfg);
    build_dos(&mut cmake_cfg);
}
