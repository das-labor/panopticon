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

use std::hash::Hash;

pub trait Graph<'a, V, E> {
    type Vertex: Clone + Hash + PartialEq + Eq + Ord + Copy;
    type Edge: Clone + Hash + PartialEq + Eq + Copy;

    fn edge_label(&self, _: Self::Edge) -> Option<&E>;
    #[inline]
    fn vertex_label(&self, _: Self::Vertex) -> Option<&V>;
    fn source(&self, _: Self::Edge) -> Self::Vertex;
    fn target(&self, _: Self::Edge) -> Self::Vertex;
}

pub trait IncidenceGraph<'a, V, E>: Graph<'a, V, E> {
    type Incidence: Iterator<Item = Self::Edge>;
    fn out_degree(&'a self, _: Self::Vertex) -> usize;
    fn out_edges(&'a self, _: Self::Vertex) -> Self::Incidence;
}

pub trait BidirectionalGraph<'a, V, E>: IncidenceGraph<'a, V, E> {
    fn in_degree(&'a self, _: Self::Vertex) -> usize;
    fn degree(&'a self, _: Self::Vertex) -> usize;
    fn in_edges(&'a self, _: Self::Vertex) -> Self::Incidence;
}

pub trait AdjacencyGraph<'a, V, E>: Graph<'a, V, E> {
    type Adjacency: Iterator<Item = Self::Vertex>;
    fn adjacent_vertices(&'a self, _: Self::Vertex) -> Self::Adjacency;
}

pub trait VertexListGraph<'a, V: 'a, E>
    : IncidenceGraph<'a, V, E> + AdjacencyGraph<'a, V, E> {
    type Vertices: Iterator<Item = Self::Vertex>;
    type VertexLabels: Iterator<Item = &'a V>;
    fn vertices(&'a self) -> Self::Vertices;
    fn num_vertices(&self) -> usize;
    fn vertex_labels(&'a self) -> Self::VertexLabels;
}

pub trait EdgeListGraph<'a, V, E: 'a>: Graph<'a, V, E> {
    type Edges: Iterator<Item = Self::Edge>;
    type EdgeLabels: Iterator<Item = &'a E>;
    fn num_edges(&self) -> usize;
    fn edges(&'a self) -> Self::Edges;
    fn edge_labels(&'a self) -> Self::EdgeLabels;
}

pub trait AdjacencyMatrixGraph<'a, V, E>: Graph<'a, V, E> {
    fn edge(&'a self, _: Self::Vertex, _: Self::Vertex) -> Option<Self::Edge>;
}

pub trait MutableGraph<'a, V: 'a, E: 'a>: Graph<'a, V, E> {
    type VertexLabelsMut: Iterator<Item = &'a mut V>;
    type EdgeLabelsMut: Iterator<Item = &'a mut E>;
    fn add_vertex(&mut self, _: V) -> Self::Vertex;
    fn add_edge(&mut self, _: E, _: Self::Vertex, _: Self::Vertex) -> Option<Self::Edge>;
    fn remove_vertex<'t>(&'t mut self, _: Self::Vertex) -> Option<V>;
    fn remove_edge(&mut self, _: Self::Edge) -> Option<E>;
    fn edge_label_mut(&mut self, _: Self::Edge) -> Option<&mut E>;
    fn edge_labels_mut(&'a mut self) -> Self::EdgeLabelsMut;
    fn vertex_label_mut(&mut self, _: Self::Vertex) -> Option<&mut V>;
    fn vertex_labels_mut(&'a mut self) -> Self::VertexLabelsMut;
}
