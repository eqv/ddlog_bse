//#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
//pub enum Expr64T {
//    Reg {
//        r: Reg64T
//    },
//    Const {
//        v: std_u32
//    },
//    BinOp {
//        op: OpType64,
//        a1: std_Ref<Expr64T>,
//        a2: std_Ref<Expr64T>
//    }
//}
use crate::ddlog_std::Vec;
use crate::ddlog_std::Ref;

pub fn sub_state(val: & Vec<Ref<Expr1T>>, subst: &ExprSubst) -> Vec<Ref<Expr1T>>  {
    let res = val.iter().map(|v| maybe_substitute_expr_1(v, subst).unwrap_or(v.clone()) ).collect::<std::vec::Vec<_>>();
    return Vec{x: res}
}


//the various many substitute functions return None if no replacement was performed. This allows us to avoid unecessary copies. 
pub fn maybe_substitute_expr_64(val: & Ref<Expr64T>, subst: &ExprSubst) ->Option<Ref<Expr64T>>  {
    if let ExprSubst::Subst64{pat64, repl64} = subst {
        if pat64 == val {
            return Some(repl64.clone())
        }
    }
    match &**val {
        Expr64T::Reg64{..} => None,
        Expr64T::Const64{..} => None,
        Expr64T::Alloc{a1} => maybe_substitute_expr_64(&a1, subst).map(|a1| Ref::from( Expr64T::Alloc{a1} ) ),
        Expr64T::BinOp64{op, a1, a2} => {
            let na1 = maybe_substitute_expr_64(&a1, subst);
            let na2 = maybe_substitute_expr_64(&a2, subst);
            if na1.is_none() && na2.is_none() {
                None
            } else {
                let op =Expr64T::BinOp64{op: op.clone(), a1: na1.unwrap_or(a1.clone()), a2: na2.unwrap_or(a2.clone())};
                Some(Ref::from(op))
            }
        },
    }
}

pub fn maybe_substitute_expr_1(val: & Ref<Expr1T>, subst: &ExprSubst) -> Option<Ref<Expr1T>> {
    if let ExprSubst::Subst1{pat1, repl1} = subst {
        if pat1 == val {
            return Some(repl1.clone())
        }
    }
    match &**val {
        Expr1T::Reg1{..} => None,
        Expr1T::Const1{..} => None,
        Expr1T::Not{b1} => maybe_substitute_expr_1(&b1, subst).map(|b1| Ref::from(Expr1T::Not{b1} ) ),
        Expr1T::Boundcheck{e} => maybe_substitute_expr_64(&e, subst).map(|e| Ref::from(Expr1T::Boundcheck{e } ) ),
        Expr1T::BinOp1_64{op, a1, a2} => {
            let na1 = maybe_substitute_expr_64(&a1, subst);
            let na2 = maybe_substitute_expr_64(&a2, subst);
            if na1.is_none() && na2.is_none() {
                None
            } else {
                let op =Expr1T::BinOp1_64{op: op.clone(), a1: na1.unwrap_or(a1.clone()), a2: na2.unwrap_or(a2.clone())};
                Some(Ref::from(op))
            }
        }
        
    }
}