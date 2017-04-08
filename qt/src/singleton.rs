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

use qml::{
    QObject,
    QVariant,
    QMetaType,
    QObjectMacro,
};

pub struct Panopticon;

impl Default for Panopticon {
    fn default() -> Panopticon {
        Panopticon
    }
}

Q_OBJECT!(
pub Panopticon as QPanopticon {
    signals:
    slots:
    properties:
});
