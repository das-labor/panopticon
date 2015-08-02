//-------------------------------------------------------------------
// SimplexJS
// https://github.com/IainNZ/SimplexJS
// A linear and mixed-integer linear programming solver.
//
// By Iain Dunning (c)
// Licensed under the MIT License.
//-------------------------------------------------------------------

// Place everything under the SimplexJS namespace
var SimplexJS = {
	// Solver status constants
	INFEASIBLE : 0,
	OPTIMAL    : 1,
	UNBOUNDED  : 2,

	// Parameters
	MAXITERATIONS: Infinity,

	//---------------------------------------------------------------
	// ModelCopy
	// 'Deep copies' a model by creating a copy of all children
	// (e.g. A, b, c, ...).  By default, copying an ojbect just
	// creates a 'shallow' copy, where the two objects would both
	// refer to the same matrices.
	// Variations of this code can be found everywhere with a quick
	// Google search - I'm not sure exactly where I got it.
	ModelCopy : function(model) {
		if (model == null || typeof(model) != 'object') return model;
		var newModel = new model.constructor();
		for (var key in model) newModel[key] = SimplexJS.ModelCopy(model[key]);
		return newModel;
	},

	//---------------------------------------------------------------
	// SolveMILP
	// Uses branch-and-bound to solve a (mixed-integer) linear
	// program. This implementation is about as basic as it gets!
	// Assumes the root model is in the form
	// 	min c.x
	//  st  A.x = b
	//      l <= x <= u
	//  Some x are integer, and b >= 0
	SolveMILP : function(rootModel, maxNodes, log) {

		if (maxNodes === undefined) maxNodes = Infinity;
		if (log === undefined) log = [];

		// Track the best integer solution found
		var bestFeasible = Infinity;
		var bestFeasibleX = new Array(rootModel.n);

		// Branch on most fractional variable
		var mostFracIndex, mostFracValue, fracValue;

		// Create and start branch-and-bound tree
		var unsolvedLPs = new Array();
		rootModel.solved = false;
		unsolvedLPs.push(rootModel);
		var nodeCount = 0;

		// Process unsolved nodes on the b-and-b tree
		while (unsolvedLPs.length >= 1) {
			nodeCount += 1;
			model = unsolvedLPs.shift();

			// Stop if we reached max node count
			if (nodeCount >= maxNodes) {
				log.push("Tried to solve node #"+nodeCount.toString()+" which is >= max = "+maxNodes.toString());
				unsolvedLPs = [];
				rootModel.status = SimplexJS.INFEASIBLE;
				return;
			}

			// Solve the LP at this node
			log.push("Solving node #"+nodeCount.toString()+", nodes on tree: "+(unsolvedLPs.length+1).toString());
			SimplexJS.PrimalSimplex(model, log);
			if (model.status == SimplexJS.INFEASIBLE) {
				// LP is infeasible, fathom it
				log.push("Node infeasible, fathoming.");
				continue;
			}
			log.push("LP solution at node = "+model.z.toString());
			//console.log(model.x);

			// Is this worse than the best integer solution?
			if (model.z > bestFeasible) {
				// Fathom
				log.push("LP solution worse than best integer feasible, fathoming.");
				continue;
			}

			// Check integrality of LP solution
			mostFracIndex = -1;
			mostFracValue = 0;
			for (var i = 0; i < model.n; i++) {
				// Is this variable integer?
				if (model.xINT[i]) {
					// Is it fractional?
					if (Math.abs(Math.floor(model.x[i]) - model.x[i]) > 0.0001) {
						// Does not satisfy integrality - will need to branch
						fracValue = Math.min( Math.abs(Math.floor(model.x[i]) - model.x[i]),
											  Math.abs(Math.ceil (model.x[i]) - model.x[i])   );
						if (fracValue > mostFracValue) {
							mostFracIndex = i;
							mostFracValue = fracValue;
						}
					}
				}
			}

			// Did we find any fractional ints?
			if (mostFracIndex == -1) {
				// No fractional ints - update best feasible?
				log.push("Node is integer feasible.");
				if (model.z < bestFeasible) {
					log.push("Best integer feasible was "+bestFeasible.toString()+", is now = "+model.z.toString());
					bestFeasible = model.z;
					for (var i = 0; i < model.n; i++) bestFeasibleX[i] = model.x[i];
				}
			} else {
				// Some fractional - create two new LPs to solve
				log.push("Node is fractional, branching on most fractional variable, "+mostFracIndex.toString());
				downBranchModel = SimplexJS.ModelCopy(model);
				downBranchModel.xUB[mostFracIndex] = Math.floor(downBranchModel.x[mostFracIndex])
				downBranchModel.z = model.z;
				unsolvedLPs.push(downBranchModel);

				upBranchModel = SimplexJS.ModelCopy(model);
				upBranchModel.xLB[mostFracIndex] = Math.ceil(upBranchModel.x[mostFracIndex])
				upBranchModel.z = model.z;
				unsolvedLPs.push(upBranchModel);
			}


		}

		// How did it go?
		rootModel.nodeCount = nodeCount;
		if (bestFeasible < Infinity) {
			// Done!
			log.push("All nodes solved or fathomed - integer solution found");
			rootModel.x = bestFeasibleX;
			rootModel.z = bestFeasible;
			rootModel.status = SimplexJS.OPTIMAL;
		} else {
			log.push("All nodes solved or fathomed - NO integer solution found");
			rootModel.status = SimplexJS.INFEASIBLE;
		}

	},

	//---------------------------------------------------------------
	// PrimalSimplex
	// Uses the revised simplex method with bounds to solve the
	// primal of a linear program. Assumes the model is in the form:
	// 	min c.x
	//  st  A.x = b
	//      l <= x <= u
	//  Some x are integer, and b >= 0
	PrimalSimplex : function(model, log) {

		if (log === undefined) log = [];

		// Get shorter names
		var A = model.A;
		var b = model.b;
		var c = model.c;
		var m = model.m;
		var n = model.n;
		var xLB = model.xLB;
		var xUB = model.xUB;

		// Define some temporary variables we will need for RSM
		var i, j;
		var varStatus = new Array(n + m);
		var basicVars = new Array(m);
		var Binv      = new Array(m);
		for (var i = 0; i < m; i++) { Binv[i] = new Array(m); }
		var cBT       = new Array(m);
		var pi        = new Array(m);
		var rc        = new Array(n);
		var BinvAs    = new Array(m);

		// Some useful constants
		var BASIC = 0, NONBASIC_L = +1, NONBASIC_U = -1;
		var TOL = 0.000001;

		// The solution
		var x = new Array(n + m), z, status;

		// Create the initial solution to Phase 1
		// - Real variables
		for (var i = 0; i < n; i++) {
			var absLB = Math.abs(xLB[i]);
			var absUB = Math.abs(xUB[i]);
			x[i]         = (absLB < absUB) ? xLB[i]     : xUB[i]    ;
			varStatus[i] = (absLB < absUB) ? NONBASIC_L : NONBASIC_U;
		}
		// - Artificial variables
		for (var i = 0; i < m; i++) {
			x[i+n] = b[i];
			// Some of the real variables might be non-zero, so need
			// to reduce x[artificials] accordingly
			for (var j = 0; j < n; j++) { x[i+n] -= A[i][j] * x[j]; }
			varStatus[i+n] = BASIC;
			basicVars[i] = i+n;
		}
		// - Basis
		for (var i = 0; i < m; i++) { cBT[i] = +1.0; }
		for (var i = 0; i < m; i++) {
			for (var j = 0; j < m; j++) {
				Binv[i][j] = (i == j) ? 1.0 : 0.0;
			}
		}

		// Being simplex iterations
		var phaseOne = true, iter = 0;
		while (true) {
			iter++;
			if (iter >= SimplexJS.MAXITERATIONS) {
				log.push("PrimalSimplex: Reached MAXITERATIONS="+iter.toString());
				z = 0.0;
				for (var i = 0; i < n; i++) z += c[i] * x[i];
				model.z = z; //Infinity;
				model.x = x;
				break;
			}

			//---------------------------------------------------------------------
			// Step 1. Duals and reduced Costs
			//console.log(Binv)
			for (var i = 0; i < m; i++) {
				pi[i] = 0.0;
				for (var j = 0; j < m; j++) {
					pi[i] += cBT[j] * Binv[j][i]
				}
			}
			//console.log(pi);
			for (var j = 0; j < n; j++) {
				rc[j] = phaseOne ? 0.0 : c[j];
				for (var i = 0; i < m; i++) {
					rc[j] -= pi[i] * A[i][j];
				}
			}
			//console.log(rc);
			//---------------------------------------------------------------------

			//---------------------------------------------------------------------
			// Step 2. Check optimality and pick entering variable
			var minRC = -TOL, s = -1;
			for (var i = 0; i < n; i++) {
				// If NONBASIC_L (= +1), rc[i] must be negative (< 0) -> +rc[i] < -TOL
				// If NONBASIC_U (= -1), rc[i] must be positive (> 0) -> -rc[i] < -TOL
				//                                                      -> +rc[i] > +TOL
				// If BASIC    (= 0), can't use this rc -> 0 * rc[i] < -LPG_TOL -> alway FALSE
				// Then, by setting initial value of minRC to -TOL, can collapse this
				// check and the check for a better RC into 1 IF statement!
				if (varStatus[i] * rc[i] < minRC) {
					minRC = varStatus[i] * rc[i];
					s = i;
				}
			}
			//console.log(minRC, s);
			// If no entering variable
			if (s == -1) {
				if (phaseOne) {
					//console.log("Phase one optimal")
					z = 0.0;
					for (var i = 0; i < m; i++) z += cBT[i] * x[basicVars[i]];
					if (z > TOL) {
						//console.log("Phase 1 objective: z = ", z, " > 0 -> infeasible!");
						log.push("PrimalSimplex: Phase 1 objective: z = "+z.toString()+" > 0 -> infeasible!");
						model.status = SimplexJS.INFEASIBLE;
						break;
					} else {
						//log.push("Transitioning to phase 2");
						phaseOne = false;
						for (var i = 0; i < m; i++) {
							cBT[i] = (basicVars[i] < n) ? (c[basicVars[i]]) : (0.0);
						}
						continue;
					}
				} else {
					model.status = SimplexJS.OPTIMAL;
					z = 0.0;
					for (var i = 0; i < n; i++) {
						z += c[i] * x[i];
					}
					model.z = z;
					model.x = x;
					//console.log("Optimality in Phase 2!",z);
					log.push("PrimalSimplex: Optimality in Phase 2: z = "+z.toString());
					//console.log(x);
					break;
				}
			}
			//---------------------------------------------------------------------

			//---------------------------------------------------------------------
			// Step 3. Calculate BinvAs
			for (var i = 0; i < m; i++) {
				BinvAs[i] = 0.0;
				for (var k = 0; k < m; k++) BinvAs[i] += Binv[i][k] * A[k][s];
			}
			//console.log(BinvAs);
			//---------------------------------------------------------------------

			//---------------------------------------------------------------------
			// Step 4. Ratio test
			var minRatio = Infinity, ratio = 0.0, r = -1;
			var rIsEV = false;
			// If EV is...
			// NBL, -> rc[s] < 0 -> want to INCREASE x[s]
			// NBU, -> rc[s] > 0 -> want to DECREASE x[s]
			// Option 1: Degenerate iteration
			ratio = xUB[s] - xLB[s];
			if (ratio <= minRatio) { minRatio = ratio; r = -1; rIsEV = true; }
			// Option 2: Basic variables leaving basis
			for (var i = 0; i < m; i++) {
				j = basicVars[i];
				var jLB = (j >= n) ? 0.0 : xLB[j];
				var jUB = (j >= n) ? Infinity : xUB[j];
				if (-1*varStatus[s]*BinvAs[i] > +TOL) { // NBL: BinvAs[i] < 0, NBU: BinvAs[i] > 0
					ratio = (x[j] - jUB) / (varStatus[s]*BinvAs[i]);
					if (ratio <= minRatio) { minRatio = ratio; r = i; rIsEV = false; }
				}
				if (+1*varStatus[s]*BinvAs[i] > +TOL) { // NBL: BinvAs[i] > 0, NBU: BinvAs[i] < 0
					ratio = (x[j] - jLB) / (varStatus[s]*BinvAs[i]);
					if (ratio <= minRatio) { minRatio = ratio; r = i; rIsEV = false; }
				}
			}

			// Check ratio
			if (minRatio >= Infinity) {
				if (phaseOne) {
					// Not sure what this means - nothing good!
					//console.log("Something bad happened");
					log.push("PrimalSimplex: Something bad happened in Phase 1...");
					break;
				} else {
					// PHASE 2: Unbounded!
					model.status = SimplexJS.UNBOUNDED;
					//console.log("Unbounded in Phase 2!");
					log.push("PrimalSimplex: Unbounded in Phase 2!");
					break;
				}
			}
			//---------------------------------------------------------------------

			//---------------------------------------------------------------------
			// Step 5. Update solution and basis
			x[s] += varStatus[s] * minRatio;
			for (var i = 0; i < m; i++) x[basicVars[i]] -= varStatus[s] * minRatio * BinvAs[i];

			if (!rIsEV) {
				// Basis change! Update Binv, flags
				// RSM tableau: [Binv B | Binv | Binv As]
				// -> GJ pivot on the BinvAs column, rth row
				var erBinvAs = BinvAs[r];
				// All non-r rows
				for (var i = 0; i < m; i++) {
					if (i != r) {
						var eiBinvAsOvererBinvAs = BinvAs[i] / erBinvAs;
						for (var j = 0; j < m; j++) {
							Binv[i][j] -= eiBinvAsOvererBinvAs * Binv[r][j]
						}
					}
				}
				// rth row
				for (var j = 0; j < m; j++) Binv[r][j] /= erBinvAs;

				// Update status flags
				varStatus[s] = BASIC;
				if (basicVars[r] < n) {
					if (Math.abs(x[basicVars[r]] - xLB[basicVars[r]]) < TOL) varStatus[basicVars[r]] = NONBASIC_L;
					if (Math.abs(x[basicVars[r]] - xUB[basicVars[r]]) < TOL) varStatus[basicVars[r]] = NONBASIC_U;
				} else {
					if (Math.abs(x[basicVars[r]] - 0.00000) < TOL) varStatus[basicVars[r]] = NONBASIC_L;
					if (Math.abs(x[basicVars[r]] - Infinity) < TOL) varStatus[basicVars[r]] = NONBASIC_U;
				}
				cBT[r] = phaseOne ? 0.0 : c[s];
				basicVars[r] = s;

			} else {
				// Degenerate iteration
				if (varStatus[s] == NONBASIC_L) { varStatus[s] = NONBASIC_U; }
				else { varStatus[s] = NONBASIC_L; }
			}
			//---------------------------------------------------------------------
			//console.log(x);
		}
	}
};

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
      O[k] = value;
      k++;
    }

    // Step 13.
    return O;
  };
}
WorkerScript.onMessage = function(msg) {
	console.log(JSON.stringify(msg));

	var nodes = ["a","b","c","d","e","f","g","h"];
	var edges = [{from: "a",to: "b"},{from: "b",to: "c"},{from: "c",to: "d"},{from: "d",to: "h"},
			{from: "a",to: "f"},{from: "a",to: "e"},{from: "e",to: "g"},{from: "f",to: "g"},{from: "g",to: "h"}];
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

	SimplexJS.PrimalSimplex(lp);

	var ret = {};
	var ranks = new Array(nodes.length).fill(0);
	for(var i = 0; i < nodes.length; i++) {
		ret[nodes[i]] = {rank:lp.x[i],order:ranks[lp.x[i]]};
		ranks[lp.x[i]] += 1;
	}

	console.log(lp.status, lp.x, lp.z);
	WorkerScript.sendMessage(ret);
};
