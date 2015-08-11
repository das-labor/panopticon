function crossProduct(a,b) {
	return (a.x * b.y) - (a.y * b.x);
}

function intersect_line(p,r,q,s) {
	var rs = crossProduct(r,s);
	var pq = q.minus(p);
	var u = crossProduct(pq,Qt.vector2d(r.x/rs,r.y/rs));
	var t = crossProduct(pq,Qt.vector2d(s.x/rs,s.y/rs));

	return rs !== 0 && u >= 0 && u <= 1 && t >= 0 && t <= 1;
}

function intersect_boxes(p,r,boxes) {
	for(var i in boxes) {
		if(boxes.hasOwnProperty(i)) {
			var o = Qt.vector2d(boxes[i].x,boxes[i].y);
			var sy = Qt.vector2d(0,boxes[i].height);
			var sx = Qt.vector2d(boxes[i].width,0);

			if(intersect_line(p,r,o,sx) ||
				 intersect_line(p,r,o,sy) ||
				 intersect_line(p,r,o.plus(sx),sy) ||
				 intersect_line(p,r,o.plus(sy),sx)) {
				return true;
			}
		}
	}

	return false;
}

WorkerScript.onMessage = function(msg) {
	console.log(JSON.stringify(msg));

	var next_node = 0;
	var nodes = [];
	var edges = [];
	var pos = {};
	var in_degree = {};
	var out_degree = {};

	for(var i = 0; i < msg.nodes.length; i++) {
		in_degree[msg.nodes[i]] = 0;
		out_degree[msg.nodes[i]] = 0;
	}

	for(var i = 0; i < msg.edges.length; i++) {
		if(msg.edges[i].from != msg.edges[i].to) {
			in_degree[msg.edges[i].to] += 1;
			out_degree[msg.edges[i].from] += 1;
		}
	}

	for(var i = 0; i < msg.nodes.length; i++) {
		var node = msg.nodes[i];
		var box = msg.boxes[node];
		var o = Qt.vector2d(box.x-3,box.y-3);
		var sx = Qt.vector2d(box.width+6,0);
		var sy = Qt.vector2d(0,box.height+6);

		var n1 = next_node++;
		var n2 = next_node++;
		var n3 = next_node++;
		var n4 = next_node++;

		pos[n1] = {x:o.x,y:o.y};
		pos[n2] = {x:o.plus(sx).x,y:o.plus(sx).y};
		pos[n3] = {x:o.plus(sy).x,y:o.plus(sy).y};
		pos[n4] = {x:o.plus(sx).plus(sy).x,y:o.plus(sx).plus(sy).y};

		nodes.push(n1,n2,n3,n4);
	}

	var ports = [];
	var used_in_ports = {};
	var used_out_ports = {};

	for(var i = 0; i < msg.edges.length; i++) {
		if(msg.edges[i].from != msg.edges[i].to) {
			var edge = msg.edges[i];
			var in_w = (in_degree[edge.to] * 5 - 5) / 2;
			var out_w = (out_degree[edge.from] * 5 - 5) / 2;
			var in_off;
			var out_off;

			if(used_in_ports[edge.to] === undefined) {
				used_in_ports[edge.to] = 1;
				in_off = 0;
			} else {
				in_off = used_in_ports[edge.to];
				used_in_ports[edge.to] += 1;
			}

			if(used_out_ports[edge.from] === undefined) {
				used_out_ports[edge.from] = 1;
				out_off = 0;
			} else {
				out_off = used_out_ports[edge.from];
				used_out_ports[edge.from] += 1;
			}

			var from_box = msg.boxes[edge.from];
			var to_box = msg.boxes[edge.to];
			var to_port = {
				x: to_box.x + to_box.width / 2 - in_w + 5 * in_off,
				y: to_box.y - 3
			};
			var from_port = {
				x: from_box.x + from_box.width / 2 - out_w + 5 * out_off,
				y: from_box.y + from_box.height + 3
			};

			var n1 = next_node++;
			var n2 = next_node++;

			pos[n1] = to_port;
			pos[n2] = from_port;
			nodes.push(n1,n2);
			ports.push({from:n2,to:n1});
		}
	}

	for(var i = 0; i < nodes.length; i++) {
		var box1 = pos[nodes[i]];
		var from = Qt.vector2d(box1.x,box1.y);

		for(var j = 0; j < nodes.length; j++) {
			if(j != i) {
				var box2 = pos[nodes[j]];

				var to = Qt.vector2d(box2.x,box2.y);

				if(!intersect_boxes(from,to.minus(from),msg.boxes)) {
					edges.push({from:nodes[i],to:nodes[j]});
				}
			}
		}
	}

	var ret = [];
	for(var i = 0; i < ports.length; i++) {
		var ps = ports[i];
		ret.push(dijkstra(nodes,edges,ps.from,ps.to));
		//ret.push([ps.from,ps.to]);
	}

	ret = ret.reduce(function(acc,cv) {
		return acc.concat([{from:pos[cv[0]],to:pos[cv[1]]}]);
	},[]);

	console.log(JSON.stringify(ret));
	WorkerScript.sendMessage(ret);
};

function smallest(q,dist) {
	var ret = q[0];

	for(var i = 1; i < q.length; i++) {
		if(dist[q[i]] < dist[ret]) {
			ret = q[i];
		}
	}

	return ret;
}

function dijkstra(nodes,edges,start,end) {
	var dist = [];
	var prev = [];
	var q = [];

	dist[start] = 0;

	for(var i = 0; i < nodes.length; i++) {
		var node = nodes[i];

		if(node != start) {
			dist[node] = Infinity;
		}

		q.push(node);
	}

	while(q.length > 0) {
		var u = q.splice(q.indexOf(smallest(q,dist)),1)[0];

		if(u == end)
			break;

		for(var i = 0; i < edges.length; i++) {
			if(edges[i].from == u) {
				var v = edges[i].to;
				var alt = dist[u] + 1;

				if(alt < dist[v]) {
					dist[v] = alt;
					prev[v] = u;
				}
			}
		}
	}

	var ret = [];
	var w = end;

	while(prev[w] !== undefined) {
		ret.unshift(w);
		w = prev[w];
	}

	if(ret.length === 0) {
		console.error("no path from " + start + " to " + end);
		ret.push(end);
	}

	ret.push(start);
	return ret;
}
