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

pub use adjacency_list::AdjacencyList;
pub use adjacency_matrix::AdjacencyMatrix;
pub use traits::AdjacencyGraph as AdjacencyGraphTrait;
pub use traits::AdjacencyMatrixGraph as AdjacencyMatrixGraphTrait;
pub use traits::BidirectionalGraph as BidirectionalGraphTrait;
pub use traits::EdgeListGraph as EdgeListGraphTrait;

pub use traits::Graph as GraphTrait;
pub use traits::IncidenceGraph as IncidenceGraphTrait;
pub use traits::MutableGraph as MutableGraphTrait;
pub use traits::VertexListGraph as VertexListGraphTrait;
