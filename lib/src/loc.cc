/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
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

#include <panopticon/loc.hh>

using namespace po;

std::unordered_map<uuid,std::pair<marshal_poly,marshal_poly>> po::dirty_locations;

#ifdef __MINGW32__
boost::mutex po::dirty_locations_mutex;
#else
std::mutex po::dirty_locations_mutex;
#endif

void po::save_point(rdf::storage &store)
{
#ifdef __MINGW32__
	std::lock_guard<boost::mutex> guard(dirty_locations_mutex);
#else
	std::lock_guard<std::mutex> guard(dirty_locations_mutex);
#endif

	for(const std::pair<uuid,std::pair<marshal_poly,marshal_poly>> &p: dirty_locations)
	{
		archive to_del = p.second.first();
		std::for_each(to_del.triples.cbegin(),to_del.triples.cend(),std::bind((bool(rdf::storage::*)(const rdf::statement&))&rdf::storage::remove,&store,std::placeholders::_1));
		std::for_each(to_del.blobs.cbegin(),to_del.blobs.cend(),std::bind((bool(rdf::storage::*)(const blob&))&rdf::storage::unregister_blob,&store,std::placeholders::_1));

		archive to_new = p.second.second();
		std::for_each(to_new.triples.cbegin(),to_new.triples.cend(),std::bind((bool(rdf::storage::*)(const rdf::statement&))&rdf::storage::insert,&store,std::placeholders::_1));
		std::for_each(to_new.blobs.cbegin(),to_new.blobs.cend(),std::bind((bool(rdf::storage::*)(const blob&))&rdf::storage::register_blob,&store,std::placeholders::_1));
	}

	dirty_locations.clear();
}

void po::discard_changes(void)
{
#ifdef __MINGW32__
	std::lock_guard<boost::mutex> guard(dirty_locations_mutex);
#else
	std::lock_guard<std::mutex> guard(dirty_locations_mutex);
#endif
	dirty_locations.clear();
}
