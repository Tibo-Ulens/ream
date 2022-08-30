use super::{Node, ToNode};
use crate::parse::{
	AssignExpr,
	AssignTarget,
	AssignValue,
	CallExpr,
	CallOperands,
	CallOperator,
	Datum,
	DefineExpr,
	DefineTarget,
	DefineValue,
	Expression,
	IdentifierExpr,
	IfAlternate,
	IfConsequent,
	IfExpr,
	IfTest,
	LambdaBody,
	LambdaExpr,
	LambdaFormals,
	LiteralExpr,
	Root,
	SequenceExpr,
};

impl ToNode for Root {
	fn to_node(&self) -> Node {
		Node {
			repr:     "Root".to_string(),
			children: self.exprs.iter().map(|e| e.to_node()).collect(),
		}
	}
}

impl ToNode for Expression {
	fn to_node(&self) -> Node {
		let (repr, children) = match self {
			Self::Identifier(i) => ("Expression(Identifier)".to_string(), vec![i.to_node()]),
			Self::Literal(l) => ("Expression(Literal)".to_string(), vec![l.to_node()]),
			Self::Sequence(s) => ("Expression(Sequence)".to_string(), vec![s.to_node()]),
			Self::Call(c) => ("Expression(Call)".to_string(), vec![c.to_node()]),
			Self::Lambda(l) => ("Expression(Lambda)".to_string(), vec![l.to_node()]),
			Self::If(i) => ("Expression(If)".to_string(), vec![i.to_node()]),
			Self::Define(d) => ("Expression(Define)".to_string(), vec![d.to_node()]),
			Self::Assign(a) => ("Expression(Assign)".to_string(), vec![a.to_node()]),
		};

		Node { repr, children }
	}
}

impl ToNode for IdentifierExpr {
	fn to_node(&self) -> Node {
		Node { repr: format!("Identifier(`{}`)", self.0), children: vec![] }
	}
}

impl ToNode for LiteralExpr {
	fn to_node(&self) -> Node {
		let (repr, children) = match self {
			Self::Quotation(d) => ("Literal(Quotation)".to_string(), vec![d.to_node()]),
			Self::Bool(t) => (format!("Literal(Bool(`{}`))", t), vec![]),
			Self::Number(t) => (format!("Literal(Number(`{}`))", t), vec![]),
			Self::String(t) => (format!("Literal(String(`{}`))", t), vec![]),
			Self::Nil => ("Literal(Nil)".to_string(), vec![]),
		};

		Node { repr, children }
	}
}

impl ToNode for Datum {
	fn to_node(&self) -> Node {
		let (repr, children) = match self {
			Self::IdentDatum(i) => ("Datum(Identifier)".to_string(), vec![i.to_node()]),
			Self::LitDatum(l) => ("Datum(Literal)".to_string(), vec![l.to_node()]),
			Self::ListDatum(v) => {
				("Datum(List)".to_string(), v.iter().map(|d| d.to_node()).collect())
			},
		};

		Node { repr, children }
	}
}

impl ToNode for SequenceExpr {
	fn to_node(&self) -> Node {
		Node {
			repr:     "Sequence".to_string(),
			children: self.0.iter().map(|e| e.to_node()).collect(),
		}
	}
}

impl ToNode for LambdaExpr {
	fn to_node(&self) -> Node {
		Node {
			repr:     "Lambda".to_string(),
			children: vec![self.formals.to_node(), self.body.to_node()],
		}
	}
}

impl ToNode for LambdaFormals {
	fn to_node(&self) -> Node {
		Node {
			repr:     "LambdaFormals".to_string(),
			children: self.0.iter().map(|i| i.to_node()).collect(),
		}
	}
}

impl ToNode for LambdaBody {
	fn to_node(&self) -> Node {
		Node {
			repr:     "LambdaBody".to_string(),
			children: self.0.iter().map(|e| e.to_node()).collect(),
		}
	}
}

impl ToNode for IfExpr {
	fn to_node(&self) -> Node {
		let children = match &self.alternate {
			Some(a) => vec![self.test.to_node(), self.consequent.to_node(), a.to_node()],
			None => vec![self.test.to_node(), self.consequent.to_node()],
		};

		Node { repr: "If".to_string(), children }
	}
}

impl ToNode for IfTest {
	fn to_node(&self) -> Node {
		Node { repr: "IfTest".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for IfConsequent {
	fn to_node(&self) -> Node {
		Node { repr: "IfConsequent".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for IfAlternate {
	fn to_node(&self) -> Node {
		Node { repr: "IfAlternate".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for DefineExpr {
	fn to_node(&self) -> Node {
		Node {
			repr:     "Define".to_string(),
			children: vec![self.target.to_node(), self.value.to_node()],
		}
	}
}

impl ToNode for DefineTarget {
	fn to_node(&self) -> Node {
		Node { repr: "DefineTarget".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for DefineValue {
	fn to_node(&self) -> Node {
		Node { repr: "DefineValue".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for AssignExpr {
	fn to_node(&self) -> Node {
		Node {
			repr:     "Assign".to_string(),
			children: vec![self.target.to_node(), self.value.to_node()],
		}
	}
}

impl ToNode for AssignTarget {
	fn to_node(&self) -> Node {
		Node { repr: "AssignTarget".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for AssignValue {
	fn to_node(&self) -> Node {
		Node { repr: "AssignValue".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for CallExpr {
	fn to_node(&self) -> Node {
		Node {
			repr:     "Call".to_string(),
			children: vec![self.operator.to_node(), self.operands.to_node()],
		}
	}
}

impl ToNode for CallOperator {
	fn to_node(&self) -> Node {
		Node { repr: "CallOperator".to_string(), children: vec![self.0.to_node()] }
	}
}

impl ToNode for CallOperands {
	fn to_node(&self) -> Node {
		Node {
			repr:     "CallOperands".to_string(),
			children: self.0.iter().map(|e| e.to_node()).collect(),
		}
	}
}
