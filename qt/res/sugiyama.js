if (!Array.prototype.fill) {
  Array.prototype.fill = function(value) {

    // Steps 1-2.
    if (this === null) {
      throw new TypeError('this is null or not defined');
    }

    var O = Object(this);

    // Steps 3-5.
    var len = O.length >>> 0;

    // Steps 6-7.
    var start = arguments[1];
    var relativeStart = start >> 0;

    // Step 8.
    var k = relativeStart < 0 ?
      Math.max(len + relativeStart, 0) :
      Math.min(relativeStart, len);

    // Steps 9-10.
    var end = arguments[2];
    var relativeEnd = end === undefined ?
      len : end >> 0;

    // Step 11.
    var fina = relativeEnd < 0 ?
      Math.max(len + relativeEnd, 0) :
      Math.min(relativeEnd, len);

    // Step 12.
    while (k < fina) {
      O[k] = JSON.parse(JSON.stringify(value));
      k++;
    }

    // Step 13.
    return O;
  };
}

function dfs(n,num,nodes,edges,seen,cb_node,cb_edge) {
	cb_node(n);
	seen[n] = num;

	for(var i = 0; i < edges.length; i++) {
		var edge = edges[i];

		if(edge.from == n) {
			cb_edge(edge,seen[edge.to] === undefined || seen[edge.to] >= num);

			if(seen[edge.to] === undefined) {
				dfs(edge.to,num + 1,nodes,edges,seen,cb_node,cb_edge);
			}
		}
	}
}

function adj_positions(n,nodes,edges,order,layout) {
	var ret = [];

	for(var i = 0; i < edges.length; i++) {
		var edge = edges[i];

		if(edge.from == n) {
			ret.push(order[layout[edge.to].rank].indexOf(edge.to));
		} else if(edge.to == n) {
			ret.push(order[layout[edge.from].rank].indexOf(edge.from));
		}
	}

	ret.sort();
	return ret;
}

function median_value(v,nodes,edges,order,layout) {
	var P = adj_positions(v,nodes,edges,order,layout);
	var m = P.length / 2;

	if(P.length === 0) {
		return -1.0;
	} else if(P.length & 1 == 1) {
		return P[m];
	} else if(P.length == 2) {
		return (P[0] + P[1]) / 2;
	} else {
		var left = P[m-1] - P[0];
		var right = P[P.length - 1] - P[m];
		return (P[m-1]*right + P[m]*left) / (left+right);
	}
}

function wmedian(iter,nodes,edges,order,layout) {
	var d = (iter & 1 == 1) ? 1 : -1;
	var s = (iter & 1 == 1) ? 0 : nodes.length - 1;
	var e = (iter & 1 == 1) ? nodes.length : -1;
	for(var r = s; r != e; r += d) {
		var ord = order[r];

		for(var i = 0; i < order.length; i++) {
			var node = nodes[i];
			layout[node].median = median_value(node,nodes,edges,order,layout);
		}

		order[r].sort(function(a,b) {
			return layout[a].median - layout[b].median;
		});
	}
}

function transpose(nodes,edges,order,layout) {
	var imp = true;

	while(imp) {
		imp = false;

		for(var r = 0; r < nodes.length; r++) {
			for(var i = 0; i < order[r].length - 2; i++) {
				var v = order[r][i];
				var w = order[r][i+1];

				var alt_order = JSON.parse(JSON.stringify(order));

				alt_order[r][i] = w;
				alt_order[r][i+1] = v;

				if(crossings(nodes,edges,order,layout) >
					 crossings(nodes,edges,alt_order,layout)) {
					imp = true;

					order[r][i] = w;
					order[r][i+1] = v;
				}
			}
		}
	}
}

function crossings(nodes,edges,order,layout) {
	var ret = 0;

	for(var i = 0; i < edges.length; i++) {
		for(var j = 0; j < edges.length; j++) {
			var e1 = edges[i];
			var e2 = edges[j];
			var e1_start_rank = layout[e1.from].rank;
			var e2_start_rank = layout[e2.from].rank;
			var e1_end_rank = layout[e1.to].rank;
			var e2_end_rank = layout[e2.to].rank;

			if(e1_start_rank == e2_start_rank && e1_end_rank == e2_end_rank) {
				if((order[e1_start_rank].indexOf(e1.from) < order[e1_end_rank].indexOf(e1.to)) !=
				   (order[e2_start_rank].indexOf(e2.from) < order[e2_end_rank].indexOf(e2.to))) {
					ret += 1;
				}
			}
		}
	}

	return ret;
}

WorkerScript.onMessage = function(msg) {
	//console.log("SU: " + JSON.stringify(msg));

	switch(msg.type) {
		case "rankingSimplex": (function(){
			var nodes = msg.nodes;
			var edges = msg.edges;
			var widths = msg.widths;
			var heights = msg.heights;

			// ensure single entry
			var has_in_edges = {};
			for(var i = 0; i < edges.length; i++) {
				var edge = edges[i];
				has_in_edges[edge.to] = true;
			}

			var heads = [];
			for(var i = 0; i < nodes.length; i++) {
				if(has_in_edges[nodes[i]] !== true) {
					heads.push(nodes[i]);
				}
			}

			var head;

			if(msg.head === undefined) {
				if(heads.length == 1) {
					head = heads[0];
				} else {
					head = "__init";
					nodes.push(head);

					for(var i = 0; i < heads.length; i++) {
						edges.push({from:head,to:heads[i]});
					}
				}
			} else {
				head = msg.head;
			}

			// remove circles
			var seen = {};
			var to_inv = [];
			dfs(head,0,nodes,edges,seen,function(v) {},function(e,n) {
				if(!n) {
					to_inv.push(e);
				}
			});

			edges = edges.map(function(e) {
				var i = to_inv.indexOf(e);

				if(i != -1) {
					return {from:e.to,to:e.from};
				} else {
					return e;
				}
			});

			// create ranking integer program
			var lp = {
				A: [],
				b: new Array(edges.length).fill(0),
				c: new Array(nodes.length).fill(0).concat(new Array(edges.length).fill(1)),
				m: edges.length,
				n: edges.length + nodes.length,
				xLB: new Array(nodes.length).fill(0).concat(new Array(edges.length).fill(1)),
				xUB: new Array(nodes.length + edges.length).fill(nodes.length)
			};

			for(var i = 0; i < edges.length; i++) {
				var Arow = new Array(edges.length + nodes.length).fill(0);
				var edge = edges[i];
				var fidx = nodes.indexOf(edge.from);
				var lidx = nodes.indexOf(edge.to);

				Arow[fidx] = -1;
				Arow[lidx] = 1;
				Arow[nodes.length + i] = -1;
				lp.A.push(Arow);
			}

			WorkerScript.sendMessage({
				lp:lp,
				nodes:nodes,
				edges:edges,
				type:"rankingSimplex",
				head:head,
				widths:widths,
				heights:heights
			});
			return;
		})(); break;
		case "order": (function() {
			var lp = msg.lp;
			var nodes = msg.nodes;
			var edges = msg.edges;
			var head = msg.head;

			var layout = {};
			for(var i = 0; i < nodes.length; i++) {
				layout[nodes[i]] = {
					rank:lp.x[i],
					width:msg.widths[nodes[i]],
					height:msg.heights[nodes[i]]
				};
			}

			// virtual nodes
			var virt_cnt = 0;
			var to_delete = [];
			var to_add = [];

			for(var i = 0; i < edges.length; i++) {
				var edge = edges[i];
				var rank_from = layout[edge.from].rank;
				var rank_to = layout[edge.to].rank;
				var prev = edge.from;

				if(rank_to - rank_from > 1) {
					for(var r = rank_from + 1; r < rank_to; r++) {
						var n = "virt_" + virt_cnt.toString();

						nodes.push(n);
						layout[n] = {rank:r};
						to_add.push({from:prev,to:n});
						prev = n;
					}

					to_add.push({from:prev,to:edge.to});
					to_delete.push(edge);
				}
			}

			edges = edges.filter(function(e) { return to_delete.indexOf(e) == -1; });
			edges.concat(to_add);

			// initial ordering
			var seen = {};
			var order = Array(nodes.length).fill([]);
			dfs(head,0,nodes,edges,seen,function(n) {
				order[layout[n].rank].push(n);
			},function(){});

			// optimize intra-rank ordering
			var best = JSON.parse(JSON.stringify(order));
			for(var i = 0; i < 24; i++) {
				wmedian(i,nodes,edges,order,layout);
				transpose(nodes,edges,order,layout);

				if(crossings(nodes,edges,order,layout) < crossings(nodes,edges,best,layout)) {
					best = order;
				}
			}

			order = best;

			for(var i = 0; i < nodes.length; i++) {
				for(var j = 0; j < order[i].length; j++) {
					layout[order[i][j]].order = j;
				}
			}

			// create x-coordinate integer program
			lp = {
				A: [], // n rows X m cols
				b: [],
				c: new Array(nodes.length).fill(0).concat(new Array(2*nodes.length).fill(1)).concat(new Array(nodes.length).fill(0)),
				m: 0,
				n: 0,
				xLB: new Array(4*nodes.length).fill(0),
				xUB: new Array(4*nodes.length).fill(5000)
			};

			for(var i = 0; i < edges.length; i++) {
				var Arow = new Array(4*nodes.length).fill(0);

				var edge = edges[i];
				var from_node_idx = nodes.indexOf(edge.from);
				var to_node_idx = nodes.indexOf(edge.to);
				var xab1_node_idx = nodes.length + i;
				var xab2_node_idx = 2*nodes.length + i;

				Arow[from_node_idx] = -1;
				Arow[to_node_idx] = 1;
				Arow[xab1_node_idx] = 1;
				Arow[xab2_node_idx] = -1;
				lp.A.push(Arow);
				lp.b.push(0);
			}

			for(var r = 0; r < nodes.length; r++) {
				for(var i = 0; i + 1 < order[r].length; i++) {
					var Arow = new Array(4*nodes.length).fill(0);
					var edge = edges[i];
					var left_node_idx = nodes.indexOf(order[r][i]);
					var right_node_idx = nodes.indexOf(order[r][i + 1]);
					var lr_cost_idx = 3*nodes.length + i;

					Arow[left_node_idx] = -1;
					Arow[right_node_idx] = 1;
					Arow[lr_cost_idx] = -1;
					lp.A.push(Arow);
					lp.b.push((layout[edge.from].width + layout[edge.to].width)/2 + 40);
				}
			}

			lp.m = lp.A.length;
			lp.n = lp.A[0].length;

			WorkerScript.sendMessage({nodes:nodes,edges:edges,layout:layout,lp:lp,type:"order"});
			return;
		})(); break;
		case "finalize": (function(){
			var lp = msg.lp;
			var layout = msg.layout;
			var nodes = msg.nodes;
			var edges = msg.edges;
			var rank_height = new Array(nodes.length).fill(0);

			for(var i = 0; i < nodes.length; i++) {
				var node = nodes[i];
				rank_height[layout[node].rank] = Math.max(rank_height[layout[node].rank],layout[node].height);
			}
			/*rank

				var in_edges = edges.filter(function(i) { return i.to == node; });
				var out_edges = edges.filter(function(i) { return i.from == node; });

				if */

			for(var i = 0; i < nodes.length; i++) {
				var node = nodes[i];
				var rank = layout[node].rank;
				var t = rank_height.slice(0,rank).reduce(function(acc,x) { return acc + x + 40; },0);

				t += (rank_height[rank] - layout[node].height) / 2;
				layout[node].x = lp.x[i];
				layout[node].y = t;
			}

			WorkerScript.sendMessage({nodes:nodes,edges:edges,layout:layout,type:"finalize"});
			return;
		})(); break;
		default: {
			WorkerScript.sendMessage({});
			return;
		}
	}
};
