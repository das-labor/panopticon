#include <sat.hh>

po::expr::expr(void) : width(0) {}
po::expr::expr(CVC4::Expr e)
: bitvector(e)
{
	CVC4::Type t = e.getType(true);
	
	if(t.isBitVector())
	{
		CVC4::BitVectorType bvt(t);
		width = bvt.getSize();
	}
	else
		width = 0;
}

po::formula_ptr po::sat(proc_ptr proc)
{
	formula_ptr ret(new formula());
	std::map<variable,expr> proxies;
	
	execute(proc,[&](const po::lvalue &left, po::instr::Function fn, const std::vector<po::rvalue> &right)
	{
		if(left.is_variable())
		{
			po::variable v = left.variable();
			proxies.insert(std::make_pair(v,expr(ret->manager.mkVar(v.name() + "-" + std::to_string(v.subscript()),ret->manager.mkBitVectorType(v.width())))));
		}
	});

	execute(proc,[&](const po::lvalue &left, po::instr::Function fn, const std::vector<po::rvalue> &right)
	{
		if(!left.is_variable() || fn == po::instr::Call || fn == po::instr::Phi)
			return;

		std::vector<expr> args;
		args.reserve(right.size());

		for(const po::rvalue &r: right)
			if(r.is_variable())
			{
				po::variable v = r.variable();
				assert(proxies.count(v));
				args.push_back(proxies[v]);
			}
			else if(r.is_memory() || r.is_undefined())
				args.push_back(ret->manager.mkVar(ret->manager.mkBitVectorType(1)));
			else if(r.is_constant())
				args.push_back(ret->manager.mkConst(CVC4::BitVector(8,(unsigned int)r.constant().value())));
			else
				assert(false);

		unsigned int width = std::accumulate(args.begin(),args.end(),0,[](unsigned int acc, const expr &e)
		{
			return std::max(acc,e.width);
		});
		
		std::cout << 1 << std::endl;
		for(const expr &e: args)
			std::cout << e.bitvector << std::endl;
		std::cout << 1.5 << std::endl;
		switch(fn)
		{
		case po::instr::And:
		case po::instr::Or:
		case po::instr::Xor:
		case po::instr::Add:
		case po::instr::Sub:
		case po::instr::Mul:
		case po::instr::UDiv:
		case po::instr::UMod:
			args[1] = adjust_width(args[1],width);
		case po::instr::Not:
		case po::instr::UShr:
		case po::instr::UShl:
		case po::instr::Slice:
			args[0] = adjust_width(args[0],width);
		default:
			;
		}
		std::cout << 2 << std::endl;
		expr e;

		switch(fn)
		{
		// Bitwise Not
		case po::instr::Not: e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_NEG,args[0].bitvector); break;
		
		// Bitwise And
		case po::instr::And:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_AND,args[0].bitvector,args[1].bitvector); break;
		
		// Bitwise Or
		case po::instr::Or:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_OR,args[0].bitvector,args[1].bitvector); break;
		
		// Bitwize Xor
		case po::instr::Xor:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_XOR,args[0].bitvector,args[1].bitvector); break;
		
		// Assign Intermediate
		case po::instr::Assign:	e = args[0]; break;
		
		// Unsigned right shift	*
		case po::instr::UShr:
			e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_LSHR,args[0].bitvector,ret->manager.mkConst(CVC4::BitVector(args[0].width,(unsigned int)right[1].constant().value()))); 
			break;
		
		// Unsigned left shift *
		case po::instr::UShl:
			e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_SHL,args[0].bitvector,ret->manager.mkConst(CVC4::BitVector(args[0].width,(unsigned int)right[1].constant().value())));
			break;
		
		// Slice
		case po::instr::Slice: e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_EXTRACT,ret->manager.mkConst(CVC4::BitVectorExtract(right[2].constant().value(),right[1].constant().value())),args[0].bitvector); break;
		
		// Addition
		case po::instr::Add:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_PLUS,args[0].bitvector,args[1].bitvector); break;
		
		// Subtraction
		case po::instr::Sub:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_SUB,args[0].bitvector,args[1].bitvector); break;
		
		// Multiplication
		case po::instr::Mul:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_MULT,args[0].bitvector,args[1].bitvector); break;
		
		// Unsigned Division
		case po::instr::UDiv:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_UDIV,args[0].bitvector,args[1].bitvector); break;
		
		// Unsigned Modulo reduction
		case po::instr::UMod:	e = ret->manager.mkExpr(CVC4::kind::BITVECTOR_UREM,args[0].bitvector,args[1].bitvector); break;
		
		default:
			std::cout << "Function: " << fn << std::endl;
			assert(false);
		}
		std::cout << 3 << ": " << fn << std::endl;

		assert(proxies.count(left.variable()));

		std::cout << "e: " << e << std::endl;
		std::cout << "l: " << proxies[left.variable()] << std::endl;
		e = adjust_width(e,proxies[left.variable()].width);
		std::cout << 4 << std::endl;
		ret->expressions.insert(std::make_pair(left.variable(),ret->manager.mkExpr(CVC4::kind::EQUAL,proxies[left.variable()].bitvector,e.bitvector)));

		std::cout << e << std::endl;
	});

	return ret;
}

po::expr po::adjust_width(const po::expr &e,unsigned int w)
{
	assert(w);

	CVC4::ExprManager *em = e.bitvector.getExprManager();

	if(e.width < w)
		return em->mkExpr(CVC4::kind::BITVECTOR_ZERO_EXTEND,em->mkConst(CVC4::BitVectorZeroExtend(w - e.width)),e.bitvector);
	else if(e.width > w)
		return em->mkExpr(CVC4::kind::BITVECTOR_EXTRACT,em->mkConst(CVC4::BitVectorExtract(w - 1,0)),e.bitvector);
	else
		return e;
}

std::ostream &po::operator<<(std::ostream &os, const po::expr &e)
{
	os << e.bitvector << " [" << e.width << "]";
	return os;
}

std::ostream &po::operator<<(std::ostream &os, const po::formula &f)
{
	for(const std::pair<po::variable,po::expr> &p: f.expressions)
		os << p.first << " = " << p.second << std::endl;
	return os;
}
