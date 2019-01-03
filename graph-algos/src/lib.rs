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

mod traits;
pub mod search;
pub mod dominator;
pub mod order;
pub mod adjacency_list;
pub mod adjacency_matrix;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bit_set;

#[cfg(test)]
extern crate rmp_serde;

pub use crate::adjacency_list::AdjacencyList;
pub use crate::adjacency_matrix::AdjacencyMatrix;
pub use crate::traits::AdjacencyGraph as AdjacencyGraphTrait;
pub use crate::traits::AdjacencyMatrixGraph as AdjacencyMatrixGraphTrait;
pub use crate::traits::BidirectionalGraph as BidirectionalGraphTrait;
pub use crate::traits::EdgeListGraph as EdgeListGraphTrait;

pub use crate::traits::Graph as GraphTrait;
pub use crate::traits::IncidenceGraph as IncidenceGraphTrait;
pub use crate::traits::MutableGraph as MutableGraphTrait;
pub use crate::traits::VertexListGraph as VertexListGraphTrait;
