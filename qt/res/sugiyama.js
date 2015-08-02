/**
 * @license
 * This file is part of the DirectedGraph library.
 *
 * The DirectedGraph library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * The DirectedGraph library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with DirectedGraph Application.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Copyright Simon Speich, 2013
 * written for Swiss Federal Institute for Forest, Snow and Landscape Research WSL
 */

var Sugiyama = {
	/* NOTES:
	 * Remember that in js arrays are passed by reference and so are objects.
	 * There is a tradeoff between separation of logic and performance:
	 * More separation means looping through the same loops again which reduces performance but plays nice with memory
	 */
	numLayer: 0,
	maxNodesAllLayers: 0,
	numPerLayers: null,
	nodes: null,
	incVirt: 0.2,
	inc: 1,
	numRepeat: 20,
	compacted: true,

	constructor: function(args) {
		this.nodes = [];
		this.numPerLayers = [];
		lang.mixin(this, args);
	},

	/**
	 * Maps the node list to a two dim array the size of the grid.
	 * nodes are stored as xy vectors of the grid. The grid is
	 * maxNodesAllLayers x numLayer and dynamically extended with virtual nodes
	 * when splitting long edges.
	 * @param {Array} nodeList list of nodes
	 */
	initNodeList: function(nodeList) {
		var i, len, layer, col, node;

		len = nodeList.length;
		for (i = 0; i < len; i++) {
			layer = nodeList[i].layer;
			if (this.nodes[layer] === undefined) {
				this.nodes[layer] = [];
			}
			col = this.nodes[layer].length;
			node = this.createNode({
				x: col,
				y: layer,
				virt: false,
				label: nodeList[i].label,
				isRaw: nodeList[i].isRaw,
				id: nodeList[i].id,
				selected: nodeList[i].selected
			});
			this.nodes[layer][col] = node;	// internal node list for fast lookup in grid
			nodeList[i].node = node;			// needed to be able to update trgNodes further below (addEdges())
			this.numPerLayers[layer] = col + 1;
			// get maximum grid width
			this.maxNodesAllLayers = this.numPerLayers[layer] > this.maxNodesAllLayers ? this.numPerLayers[layer] : this.maxNodesAllLayers;
		}
		len = this.nodes.length;
		for (i = 0; i < len; i++) {
			if (this.nodes[i] === undefined) {
				this.nodes[i] = [];
			}
		}
		len = this.numPerLayers.length;
		for (i = 0; i < len; i++) {
			if (this.numPerLayers[i] === undefined) {
				this.numPerLayers[i] = 0;
			}
		}
	},

	/**
	 * Adds a node's edges.
	 * Adds a node's neighbours as a list of source and target nodes
	 * @param {Array} nodeList
	 * @param {Array} adjList list
	 */
	addEdges: function(nodeList, adjList) {
		var i, j, len, lenJ, target, source,
			trgNodes, tx, ty, sx, sy;

		len = adjList.length;
		for (i = 0; i < len; i++) {
			if (adjList[i].length > 0) {
				trgNodes = adjList[i];
				lenJ = trgNodes.length;
				for (j = 0; j < lenJ; j++) {
					target = nodeList[trgNodes[j]].node;
					tx = target.x;
					ty = target.y;
					source = nodeList[i].node;	// get target's source nodes
					sx = source.x;
					sy = source.y;
					this.nodes[sy][sx].trgNodes.push([tx, ty]);
					// also add this node to the source list of the edge
					this.nodes[ty][tx].srcNodes.push([sx, sy]);
				}
			}
		}
	},

	/**
	 * Sets the weight of all node'.
	 * A node's weight is either based on up-neighbours
	 * or down-neighbours median.
	 */
	setNodeWeights: function() {
		var i, len, w, l, node;

		len = this.nodes.length;
		for (i = 0; i < len; i++) {
			l = this.nodes[i].length;
			for (w = 0; w < l; w++) {
				node = this.nodes[i][w];
				if (node.srcNodes.length > 0) {
					node.wUpper = this.getMedian(node, 'upper');
				}
				if (node.trgNodes.length > 0) {
					node.wLower = this.getMedian(node, 'lower');
				}
			}
		}
	},

	/**
	 * Adds virtual (dummy) nodes and edges to the node list.
	 */
	expandNodeList: function() {
		var i, j, numRow, numCol, node;

		numRow = this.nodes.length;
		for (i = 0; i < numRow; i++) {
			numCol = this.nodes[i].length;
			for (j = 0; j < numCol; j++) {
				node = this.nodes[i][j];
				if (!node.virt && node.trgNodes.length > 0) {
					this.splitLongEdges(node);
				}
			}
		}
		this.setNodeWeights();
	},

	/**
	 * Inserts virtual nodes and edges to create a proper graph.
	 * Edges spanning more than one layer will be split into multiple edges of unit length.
	 * This is done by shortening, e.g decrementing long edges continuously until it has unit length.
	 * @param {Object} srcNode source node
	 */
	splitLongEdges: function(srcNode) {
		var f, lenF, w, l,
			lastVirtNode, trgNode, trgNodes, span, origTrgNode, node, newNode;

		trgNodes = srcNode.trgNodes;
		l = trgNodes.length;
		for (w = 0; w < l; w++) {
			lastVirtNode = null;
			trgNode = {
				x: trgNodes[w][0],
				y: trgNodes[w][1]
			};

			span = trgNode.y - srcNode.y;
			// skip short edges of unit length
			if (span === 1) {
				continue;
			}
			span--;		// We do not need to crate a new node for the original targetNode,
			while (span > 0) {
				/**** APPEND virtual nodes and edges ***/
				newNode = this.createVirtualNode(srcNode, trgNode, span);
				if (!lastVirtNode) {
					lastVirtNode = newNode;
				}
				if (this.nodes[newNode.y] === undefined) {
					this.nodes[newNode.y] = [newNode];
				}
				else {
					this.nodes[newNode.y].push(newNode);
				}
				// update maximum number of nodes per layer to know width of canvas
				this.numPerLayers[newNode.y]++;
				if (this.numPerLayers[newNode.y] > this.maxNodesAllLayers) {
					this.maxNodesAllLayers++;
				}
				span--;
			}
			// update original targetNode's srcNode to point to first inserted new node
			origTrgNode = trgNodes[w];
			origTrgNode = this.nodes[origTrgNode[1]][origTrgNode[0]];
			lenF = origTrgNode.srcNodes.length;
			for (f = 0; f < lenF; f++) {   // find right srcNode to update
				node = origTrgNode.srcNodes[f];
				if (node[0] === srcNode.x && node[1] === srcNode.y) {
					origTrgNode.srcNodes[f][0] = lastVirtNode.x;
					origTrgNode.srcNodes[f][1] = lastVirtNode.y;
					break;
				}
			}
			// update srcNode's original targetNode to point to last inserted new node
			// note: do this after updating original target's srcNode, otherwise it is overwritten by newNode
			srcNode.trgNodes[w] = [newNode.x, newNode.y];
		}
	},

	/**
	 *  Creates and returns a GraphNode.
	 * @param {Object} params node parameter
	 *	@property {Number} x x-coordinate of unit length
	 * @property {Number} y y-coordinate of unit length
	 * @property {Number} lx x-coordinate of layout
	 * @property {Number} ly y-coordinate of layout
	 * @property {Array} trgNodes list of adjacent target nodes
	 * @property {Array} srcNodes list of adjacent source nodes
	 * @property {Number} wUpper weight of target nodes (upper nodes),
	 * @property {Number} wLower weight of source nodes {lower nodes),
	 *	@property {Boolean} virt is node virtual
	 * @property {String} label label to display at node
	 * @property {Boolean} isRaw is node from raw data or derived data
	 * @property {Boolean} selected is node selected (base node of graph)
	 * @returns {GraphNode|VirtualNode}
	 */
	createNode: function(params) {
		var GraphNode = {
			x: 'x' in params ? params.x : 0,
			y: 'y' in params ? params.y : 0,
			lx: 'lx' in params ? params.lx : ('x' in params ? params.x : 0),
			ly: 'ly' in params ? params.ly : ('y' in params ? params.y : 0),
			trgNodes: 'trgNodes' in params ? [params.trgNodes] : [],
			srcNodes: 'srcNodes' in params ? [params.srcNodes] : [],
			wUpper: 'wUpper' in params ? params.wUpper : null,
			wLower: 'wLower' in params ? params.wLower : null,
			virt: 'virt' in params ? params.virt : false,
			label: 'label' in params ? params.label : false,
			isRaw: 'isRaw' in params ? params.isRaw : false,
			selected: 'selected' in params ? params.selected : false
		};
		if ('id' in params) {
			GraphNode.id = params.id;
		}
		return GraphNode;
	},

	/**
	 * Creates a new virtual node.
	 * New virtual nodes are appended to the layer, e.g x equals number of nodes on this layer
	 * @param {GraphNode} srcNode source node
	 * @param {GraphNode} trgNode target node
	 * @param {Number} span length of span in unit length
	 * @returns {VirtualNode}
	 */
	createVirtualNode: function(srcNode, trgNode, span) {
		// we have 4 cases: target is a VirtualNode
		// target is originalTarget
		// source is virtual
		// source is originalSource
		var target, source, curLayer, x;

		curLayer = srcNode.y + span;
		if (curLayer === trgNode.y - 1) {   // second new node has original target node as target
			target = [trgNode.x, trgNode.y];
		}
		else {
			target = [this.numPerLayers[curLayer + 1] - 1, curLayer + 1];
		}
		if (span === 1) { // second to last has original source as source
			source = [srcNode.x, srcNode.y];
		}
		else {
			source = [this.numPerLayers[curLayer - 1], curLayer - 1];
		}
		x = this.numPerLayers[curLayer];
		return this.createNode({
			x: x,
			y: curLayer,
			trgNodes: target,
			srcNodes: source,
			virt: true,
			label: null,
			isRaw: false
		});
	},

	/**
	 * Counts the number of edge crossings of a layer's nodes.
	 * Compares the x-values of a node's edges with all the x-edges
	 * of the following nodes on that layer.
	 * @param {Array} layer layer's nodes
	 * @param {Number} direction
	 * @return {Number} number of crossings
	 */
	countCrossings: function(layer, direction) {
		var propName = direction === 'up' ? 'trgNodes' : 'srcNodes',
			nodes = this.nodes[layer],
			i, lenI, numCross = 0,
			w, lenW, edges, edge1, edge2, node1, node2,
			p, lenP, z;

		lenI = nodes.length;
		// loop nodes
		for (i = 0; i < lenI; i++) {
			node1 = nodes[i];
			edges = node1[propName];
			lenW = edges.length;
			// loop node's targets
			for (w = 0; w < lenW; w++) {
				edge1 = edges[w];
				edge1 = this.nodes[edge1[1]][edge1[0]];
				// compare
				z = i + 1;
				for (z; z < lenI; z++) {
					node2 = nodes[z];
					lenP = node2[propName].length;
					for (p = 0; p < lenP; p++) {
						edge2 = node2[propName][p];
						edge2 = this.nodes[edge2[1]][edge2[0]];
						if (edge1.x > edge2.x) {
							numCross++;
						}
					}
				}
			}
		}
		return numCross;
	},

	/**
	 * Minimize number of crossings between layers.
	 * Important: We only manipulate the x and y properties,
	 * when finding right ordering of nodes, the lookup properties
	 * srcNodes and trgNodes are kept unchanged. Otherwise we
	 * would have to do a lot of updating these lists.
	 * numSweep: Up and down counts as one sweep
	 * @param {Number} [numRepeat] number of sweeps
	 */
	minimizeCrossings: function(numRepeat) {

		numRepeat = numRepeat || this.numRepeat;

		/*
		 // TODO: take into account two or more nodes on layer having same median
		 // TODO: also use countCrossings instead of numRepeat
		 var sweepDown = true;
		 var sweepUp = true;
		 var z= 0;
		 while (sweepDown == true || sweepUp == true) {
		 var i = 1;	// first layer does not have any upperWeights
		 var len = this.nodes.length;
		 for (; i < len; i++) {
		 sweepDown = this.orderLayer(i, 'down');
		 }
		 i = len - 2;	// last layer does not have any lowerWeights
		 for (; i > -1; i--) {
		 sweepUp = this.orderLayer(i, 'up');
		 }
		 z++;

		 }
		 console.debug(z);
		 */
		var z, i, len;
		for (z = 0; z < numRepeat; z++) {
			i = 1;	// first layer does not have any upperWeights
			len = this.nodes.length;
			for (i; i < len; i++) {
				this.orderLayer(i, 'down');
			}
			i = len - 2;	// last layer does not have any lowerWeights
			for (i; i > -1; i--) {
				this.orderLayer(i, 'up');
			}
		}
	},

	/**
	 * Orders nodes per layer based on median weight of incident nodes.
	 * Ordering is only done as long as the reducing in #crossings is > than threshold
	 * @param {Number} layer
	 * @param {String} direction sweep direction up/down
	 * @return {Boolean}
	 */
	orderLayer: function(layer, direction) {
		var nodeOrderRestore = this.nodes[layer].slice(0),
			numCross1 = this.countCrossings(layer, direction),
			nodeOrder = this.nodes[layer],	// node ordering has to be done on this.nodes for countCrossings to work
			numCross2, i, len, oldX;

		nodeOrder.sort(direction === 'up' ? this.compareByLowerWeight : this.compareByUpperWeight);

		numCross2 = this.countCrossings(layer, direction);
		if (numCross2 < numCross1) {
			// reassign new position (rank)
			len = nodeOrder.length;
			for (i = 0; i < len; i++) {
				if (i !== nodeOrder[i].x) {   // set new pos only if node position has changed
					oldX = nodeOrder[i].x;
					nodeOrder[i].x = i;
					nodeOrder[i].lx = i;
					// update edges and weights of rearranged nodes
					this.updateEdges(nodeOrder[i], oldX);
				}
			}
			return true;
		}

		// update order in nodelist
		this.nodes[layer] = nodeOrderRestore.slice(0);
		return false;
	},

	/**
	 * Updates a node's edges to point to the node's new position.
	 * Since only x positions are changed we only have to update x pos.
	 * @param {Object} node
	 * @param {Number} oldIndex of node
	 */
	updateEdges: function(node, oldIndex) {
		var i, x, y , lenV, v,
			nodes, len, trgNode, trgNodes, srcNode, srcNodes;

		// update node's source nodes
		nodes = node.srcNodes;
		len = nodes.length;
		for (i = 0; i < len; i++) {
			x = nodes[i][0];
			y = nodes[i][1];
			// find srcNode's target nodes to update
			trgNode = this.nodes[y][x];
			trgNodes = trgNode.trgNodes;
			lenV = trgNodes.length;
			for (v = 0; v < lenV; v++) {
				if (trgNodes[v][0] === oldIndex) {
					trgNodes[v][0] = node.x;
					trgNode.wLower = this.getMedian(trgNode, 'lower');
					break;
				}
			}
		}
		// update node's target nodes
		nodes = node.trgNodes;
		len = nodes.length;
		for (i = 0; i < len; i++) {
			x = nodes[i][0];
			y = nodes[i][1];
			// find trgNode's source nodes to update
			srcNode = this.nodes[y][x];
			srcNodes = srcNode.srcNodes;
			lenV = srcNodes.length;
			for (v = 0; v < lenV; v++) {
				if (srcNodes[v][0] === oldIndex) {
					srcNodes[v][0] = node.x;
					srcNode.wUpper = this.getMedian(srcNode, 'upper');
					break;
				}
			}
		}
	},

	/**
	 * Calculates the median of the node's edges.
	 * The median is calculated from the x values relative to the
	 * x value of the node. This can either be the nodes on the previous (up)
	 * or next (down) layer.
	 * @param {Object} node
	 * @param {String} type median of lower or upper layer
	 * @return {number} median
	 */
	getMedian: function(node, type) {
		var weights = [], i, len, middle, nodes;

		nodes = (type === 'upper' ? node.srcNodes : node.trgNodes);
		len = nodes.length;
		for (i = 0; i < len; i++) {
			// do not sort srcNodes or trgNodes directly
			weights[i] = this.nodes[nodes[i][1]][nodes[i][0]].lx;
		}
		weights.sort(function ascend(a, b) {
			return a - b;
		});
		middle = Math.floor(weights.length / 2);
		if ((weights.length % 2) !== 0) {
			return weights[middle];
		}
		return (weights[middle - 1] + weights[middle]) / 2;
	},

	/**
	 * Align nodes on layer according to barrycenter and priority.
	 */
	setLayoutPosition: function() {
		var i, len, w, lenW = 4;

		for (w = 0; w < lenW; w++) {
			len = this.nodes.length;
			for (i = 0; i < len; i++) {
				//	this.setDegree(i, 'down');
				this.align(i, 'down');
			}
			this.setNodeWeights();

			i = len - 1;
			for (i; i > -1; i--) {
				//	this.setDegree(i, 'up');
				this.align(i, 'up');
			}
			// TODO: Improve performance by only updating lower(upper) (e.g layer +- 1) when on a layer
			this.setNodeWeights();
		}
	},

	/**
	 * Defines an order based on degree and edge type.
	 * Virtual nodes get highest degree. Degree is
	 * based on number of edges.
	 * @param {Number} layer layer the node is on
	 * @param {String} direction up or down
	 */
	setDegree: function(layer, direction) {
		var i, len, z, degree,
			maxDegree = 0,
			nodes = this.nodes[layer];

		// find highest degree
		maxDegree = 0;
		len = nodes.length;
		for (i = 0; i < len; i++) {
			if (direction === 'up') {
				degree = nodes[i].trgNodes ? nodes[i].trgNodes.length : 0;
			}
			else {
				degree = nodes[i].srcNodes ? nodes[i].srcNodes.length : 0;
			}
			degree *= 2;
			if (maxDegree < degree) {
				maxDegree = degree;
			}
			if (!nodes[i].virt) {
				nodes[i].degree = degree;
			}
		}
		// give virtual nodes higher degree than max degree
		maxDegree++;
		for (z = 0; z < len; z++) {
			if (nodes[z].virt) {
				nodes[z].degree = maxDegree;
			}
		}
	},

	/**
	 * Compares the upper weight of two GraphNodes.
	 * @param {GraphNode} a
	 * @param {GraphNode} b
	 */
	compareByUpperWeight: function(a, b) {
		if (a.wUpper === null || b.wUpper === null) {
			return 0;
		}
		return (a.wUpper - b.wUpper);
	},

	/**
	 * Compares the lower weight of two GraphNodes.
	 * @param {GraphNode} a
	 * @param {GraphNode} b
	 */
	compareByLowerWeight: function(a, b) {
		if (a.wLower === null || b.wLower === null) {
			return 0;
		}
		return (a.wLower - b.wLower);
	},

	/**
	 * Returns some statistics about the graph.
	 * @return {Object}
	 */
	getGraphStat: function() {
		// since graph might have changed in the meantime we
		// have to recalculate this every time
		var stat = {
			nodesTotal: 0,
			nodesDerived: 0,
			nodesRaw: 0,
			edges: 0
			},
			node,
			z, lenZ, i, len, j, lenJ;

		len = this.nodes.length;
		for (i = 0; i < len; i++) {
			lenZ = this.nodes[i].length;
			for (z = 0; z < lenZ; z++) {
				node = this.nodes[i][z];
				if (node.isRaw) {
					stat.nodesRaw++;
				}
				if (!node.virt) {
					lenJ = node.trgNodes.length;
					for (j = 0; j < lenJ; j++) {
						stat.edges++;
					}
					stat.nodesTotal++;
				}
			}
		}
		stat.nodesDerived = stat.nodesTotal - stat.nodesRaw;
		return stat;
	},

	/**
	 * Align nodes according to the up/down barycenter.
	 * They are moved as closely to the barycenter as possible
	 * using the priority list.
	 * @param {Number} layer list of nodes to move
	 * @param {String} direction string
	 *
	 */
	align: function(layer, direction) {
		// move node according to it's upper/lower median
		var numDec = this.getDecimalPlaces(this.incVirt, '.'),
			len = this.nodes[layer].length,
			i = len - 1,
			node, newX, lx;

		for (i; i > -1; i--) {
			node = this.nodes[layer][i];
			newX = direction === 'up' ? node.wLower : node.wUpper;
			if (node.virt) {
				newX = this.round(newX, numDec);
			}
			else {
				newX = Math.round(newX / this.inc) * this.inc;	// align to inc grid
			}
			if (node.lx < newX) { // nodes can only be moved to the right, because after compacting we are already as far left as possible
				if (i === len - 1) {   // right most node can always move right
					node.lx = newX;
					continue;
				}
				// shift left as long as pos is unoccupied
				// note: when using inc you also have to check for incVirt in between
				lx = this.round(node.lx + this.incVirt, numDec);
				while (lx <= newX) {
					if (lx === this.nodes[layer][i + 1].lx) {   // pos already occupied
						break;
					}
					else if (node.virt || lx % this.inc === 0) {
						node.lx = lx;
					}
					lx = this.round(lx + this.incVirt, numDec);
				}
			}
		}
	},

	/**
	 * Compact graph
	 * Compacts graph by moving nodes as far to left as possible.
	 * Graph is assumed to be integers only, changed graph can
	 * have fractions.
	 */
	compact: function() {
		var numDec = this.getDecimalPlaces(this.incVirt, '.'),
			g, lenG, node, inc, lx,
			i, len = this.nodes.length;

		for (i = 0; i < len; i++) {
			g = 1;	// first node is always lx == 0
			lenG = this.nodes[i].length;
			for (g; g < lenG; g++) {
				node = this.nodes[i][g];
				inc = node.virt ? this.incVirt : this.inc;
				lx = node.lx - inc;
				if (lx === this.nodes[i][g - 1].lx) {   // quickly check pos if already occupied
					continue;
				}
				// shift left as long as unoccupied
				while (lx > 0) {
					if (lx === this.nodes[i][g - 1].lx) {   // pos already occupied
						break;
					}
					else if (node.virt || lx % this.inc === 0) {
						node.lx = lx;
					}
					lx = this.round(lx - this.incVirt, numDec);
				}
			}
		}
		this.setNodeWeights();
	},

	/**
	 * Returns the width of the graph.
	 * @return {Number}
	 */
	getGraphWidth: function() {
		var i, len, z, lenZ, width = 0;

		len = this.nodes.length;
		for (i = 0; i < len; i++) {
			lenZ = this.nodes[i].length;
			for (z = 0; z < lenZ; z++) {
				if (this.nodes[i][z].lx > width) {
					width = this.nodes[i][z].lx;
				}
			}
		}
		return width + 1;
	},

	/**
	 * Returns the rounded number.
	 * @param {Number} num number
	 * @param {Number} decimalPlaces number of decimal places
	 */
	round: function(num, decimalPlaces) {
		return Math.round(parseFloat(num) * Math.pow(10, decimalPlaces)) / Math.pow(10, decimalPlaces);
	},

	/**
	 * Returns the number of decimal places.
	 * @param {Number} x number
	 * @param {String} decSeparator decimal separator
	 * @return {Number}
	 */
	getDecimalPlaces: function(x, decSeparator) {
		var str = x.toString();
		if (str.indexOf(decSeparator) > -1) {
			return str.length - str.indexOf(decSeparator) - 1;
		}

		return 0;
	},

	/**
	 * Convenience function to render graph.
	 * @param {Object} graphData
	 * @param {Object} graphData.nodeList
	 * @param {Object} graphData.adjList
	 */
	render: function(graphData) {
		this.initNodeList(graphData.nodeList);
		this.addEdges(graphData.nodeList, graphData.adjList);
		this.expandNodeList();
		this.minimizeCrossings();
		if (this.compacted) {
			this.compact();
		}
		this.setLayoutPosition();
	}
};

WorkerScript.onMessage = function(msg) {
	console.log(JSON.stringify(msg));
};
