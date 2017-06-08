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

extern crate panopticon_core;
extern crate tempdir;

use panopticon_core::{Region,Layer,Bound};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;

#[test]
fn read_one_layer() {
    if let Ok(ref tmpdir) = TempDir::new("test-panop") {
        let p1 = tmpdir.path().join(Path::new("test"));

        let mut r1 = Region::undefined("test".to_string(), 128);

        {
            let fd = File::create(p1.clone());
            assert!(fd.unwrap().write_all(b"Hello, World").is_ok());
        }

        assert!(
            r1.cover(
                Bound::new(1, 8),
                Layer::wrap(vec![1, 2, 3, 4, 5, 6, 7]),
                )
            );
        assert!(
            r1.cover(
                Bound::new(50, 62),
                Layer::wrap(vec![1, 2, 3, 4, 5, 6, 6, 5, 4, 3, 2, 1]),
                )
            );
        assert!(r1.cover(Bound::new(62, 63), Layer::wrap(vec![1])));
        assert!(r1.cover(Bound::new(70, 82), Layer::open(&p1).unwrap()));

        let s = r1.iter();
        let mut idx = 0;

        assert_eq!(s.len(), 128);

        for i in s {
            if idx >= 1 && idx < 8 {
                assert_eq!(i, Some(idx));
            } else if idx >= 50 && idx < 56 {
                assert_eq!(i, Some(idx - 49));
            } else if idx >= 56 && idx < 62 {
                assert_eq!(i, Some(6 - (idx - 56)));
            } else if idx >= 70 && idx < 82 {
                assert_eq!(
                    i,
                    Some(
                        "Hello, World".to_string().into_bytes()[(idx - 70) as
                        usize]
                        )
                    );
            } else if idx == 62 {
                assert_eq!(i, Some(1));
            } else {
                assert_eq!(i, None);
            }
            idx += 1;
        }
    }
}
