WorkerScript.onMessage = function(msg) {
	//console.log(JSON.stringify(msg));

	var pos = {};
	var nodes = [];
	var next_node = msg.nodes.length + 5;
	var in_degree = {};
	var out_degree = {};

	for(var i = 0; i < msg.nodes.length; i++) {
		in_degree[msg.nodes[i]] = 0;
		out_degree[msg.nodes[i]] = 0;
	}

	for(var l = 0; l < msg.edges.length; l++) {
		if(msg.edges[l].from != msg.edges[l].to) {
			in_degree[msg.edges[l].to] += 1;
			out_degree[msg.edges[l].from] += 1;
		}
	}

	var ports = [];
	var used_in_ports = {};
	var used_out_ports = {};
	var loops = [];
	var ret = [];

	for(var j = 0; j < msg.edges.length; j++) {
		if(msg.edges[j].from != msg.edges[j].to) {
			var edge = msg.edges[j];
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
				y: to_box.y - (edge.to.indexOf("virt") === 0 ? 0 : 3)
			};
			var from_port;

			var from_rank = msg.layout[edge.from].rank;
			var to_rank = msg.layout[edge.to].rank;
			var n1 = next_node++;
			var n2 = next_node++;

			if(edge.from.indexOf("virt") !== 0) {
				if(from_rank > to_rank) {
					from_port = {
						x: from_box.x + from_box.width + 6,
						y: from_box.y - 6
					};

					var x1 = (next_node++).toString();
					var x2 = (next_node++).toString();
					var x3 = (next_node++).toString();

					pos[x1] = {x:from_port.x,y:from_box.y + from_box.height + 6};
					pos[x2] = {x:from_box.x + from_box.width / 2 - out_w + 5 * out_off,y:from_box.y + from_box.height + 6};
					pos[x3] = {x:from_box.x + from_box.width / 2 - out_w + 5 * out_off,y:from_box.y};

					nodes.push(x1,x2,x3);
					ret.push([n2,x1,x2,x3]);
				} else {
					from_port = {
						x: from_box.x + from_box.width / 2 - out_w + 5 * out_off,
						y: from_box.y + from_box.height + 3
					};
				}
			} else {
				from_port = {
					x: from_box.x + from_box.width / 2 - out_w + 5 * out_off,
					y: from_box.y
				};
			}

			if(edge.to.indexOf("virt") !== 0) {
				if(from_rank > to_rank) {
					to_port = {
						x: to_box.x + to_box.width + 6,
						y: to_box.y + to_box.height + 6
					};

					var y1 = (next_node++).toString();
					var y2 = (next_node++).toString();
					var y3 = (next_node++).toString();

					pos[y1] = {x:to_port.x,y:to_box.y - 6};
					pos[y2] = {x:to_box.x + to_box.width / 2 - in_w + 5 * in_off,y:to_box.y - 6};
					pos[y3] = {x:to_box.x + to_box.width / 2 - in_w + 5 * in_off,y:to_box.y};

					nodes.push(y1,y2,y3);
					ret.push([n1,y1,y2,y3]);
				} else {
					to_port = {
						x: to_box.x + to_box.width / 2 - in_w + 5 * in_off,
						y: to_box.y - 3
					};
				}
			} else {
				to_port = {
					x: to_box.x + to_box.width / 2 - in_w + 5 * in_off,
					y: to_box.y
				};
			}

			pos[n1] = to_port;
			pos[n2] = from_port;
			nodes.push(n1,n2);
			ports.push({from:n2,to:n1,from_center:from_box.y,to_center:to_box.y});
		} else {
			loops.push(msg.edges[j].from);
		}
	}

	var edges = null;

	for(var k = 0; k < ports.length; k++) {
		var ps = ports[k];
		var vec_f = Qt.vector2d(ps.from.x,ps.to.y);
		var vec_t = Qt.vector2d(ps.to.x,ps.to.y);
		var path;

		if(intersect_boxes(vec_f,vec_t.minus(vec_f),msg.boxes)) {
			if(edges === null) {
				var tmp = visibility_graph(msg.nodes,msg.boxes);

				nodes = nodes.concat(tmp[0]);
				edges = tmp[1];
				pos = pos.concat(tmp[2]);
			}

			path = dijkstra(nodes,edges,ps.from,ps.to);
		} else {
			path = [ps.from,ps.to];
		}

		var from_end = next_node++;
		var to_end = next_node++;

		pos[from_end] = {x:pos[path[0]].x,y:ps.from_center};
		pos[to_end] = {x:pos[path[path.length-1]].x,y:ps.to_center};

		path.unshift(from_end);
		path.push(to_end);
		ret.push(path);
		nodes.push(from_end,to_end);
	}

	ret = ret.reduce(function(acc,cv) {
		var last = cv[0];
		return acc.concat(cv.map(function(x) {
			var ret = {from:pos[last],to:pos[x]};
			last = x;

			return ret;
		}));
	},[]);


	for(var m = 0; m < loops.length; m++) {
		var p = msg.boxes[loops[m]];

		ret.push({
			from:{x:p.x+5,y:p.y+3},
			to:  {x:p.x+5,y:p.y-3}
		},{
			from:{x:p.x+5,y:p.y-3},
			to:  {x:p.x-5,y:p.y-3}
		},{
			from:{x:p.x-5,y:p.y-3},
			to:  {x:p.x-5,y:p.y+p.height+3}
		},{
			from:{x:p.x-5,y:p.y+p.height+3},
			to:  {x:p.x+5,y:p.y+p.height+3}
		},{
			from:{x:p.x+5,y:p.y+p.height+3},
			to:  {x:p.x+5,y:p.y+p.height-3}
		});
	}

	WorkerScript.sendMessage(ret);
};
