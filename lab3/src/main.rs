use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    use Expr::*;
    use Value::*;

    match expr {
        ArithExpr(expr)     => Value::IntValue(eval_arith_expr(expr)),
        BoolExpr(expr)      => Value::BoolValue(eval_bool_expr(expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    use ArithExpr::*;

    match arith_expr {
        BinArithExpr {left, right, op}      => 
            match op {
                AddOp           => eval_arith_expr(*left) + eval_arith_expr(*right),
                SubOp           => eval_arith_expr(*left) - eval_arith_expr(*right),
                MulOp           => eval_arith_expr(*left) * eval_arith_expr(*right),
                IntDivOp        => eval_arith_expr(*left) / eval_arith_expr(*right),
            }
        
        IntLit(num)     => num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    use BoolExpr::*;
    use ArithExpr::*;

    match bool_expr {
        ArithCmpExpr {left, right, op}      =>
        {
            match op {
                LtOp            => eval_arith_expr(*left) < eval_arith_expr(*right),
                LteOp           => eval_arith_expr(*left) <= eval_arith_expr(*right),
                GtOp            => eval_arith_expr(*left) > eval_arith_expr(*right),
                GteOp           => eval_arith_expr(*left) >= eval_arith_expr(*right),
                ArithEqOp       => eval_arith_expr(*left) == eval_arith_expr(*right),
                ArithNeqOp      => eval_arith_expr(*left) != eval_arith_expr(*right),
            }
        }

        BinBoolExpr {left, right, op}       =>
        {
            match op {
                AndOp           => eval_bool_expr(*left) & eval_bool_expr(*right),
                OrOp            => eval_bool_expr(*left) | eval_bool_expr(*right),
                BoolEqOp        => eval_bool_expr(*left) == eval_bool_expr(*right),
                BoolNeqOp       => eval_bool_expr(*left) != eval_bool_expr(*right),
            }
        }

        NotExpr(expr)              => !eval_bool_expr(*expr),

        BoolLit(bool)              => bool,      

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expr = Expr::ArithExpr(BinArithExpr {left: Box::new(IntLit(51)), right: Box::new(IntLit(17)), op: IntDivOp});
        let answer = IntValue(3);

        assert_eq!(eval(expr), answer);  // eval(Expr::ArithExpr(BinArithExpr {left: Box::new(IntLit(51)), right: Box::new(IntLit(17)), op: IntDivOp})) == IntValue(3)
    }

    #[test]
    fn test_2() {
        let expr = Expr::BoolExpr(BinBoolExpr{left: Box::new(BoolLit(false)), right: Box::new(BoolLit(false)), op: BoolEqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // Expr::BoolExpr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolNeqOp}) == BoolValue(true)
    }

    #[test]
    fn test_3() {
        let expr = Expr::BoolExpr(ArithCmpExpr{left: Box::new(BinArithExpr{left: Box::new(IntLit(12)), right: Box::new(IntLit(155)), op: AddOp}), right: Box::new(IntLit(167)), op: LteOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // Expr::BoolExpr(ArithCmpExpr{left: Box::new(BinArithExpr{left: Box::new(IntLit(12)), right: Box::new(IntLit(155)), op: AddOp}), right: Box::new(IntLit(167)), op: LteOp}) == BoolValue(true)
    }

    #[test]
    fn test_4() {
        let expr = Expr::ArithExpr(BinArithExpr {left: Box::new(IntLit(51)), right: Box::new(IntLit(17)), op: SubOp});
        let answer = IntValue(34);

        assert_eq!(eval(expr), answer);  // Expr::ArithExpr(BinArithExpr {left: Box::new(IntLit(51)), right: Box::new(IntLit(17)), op: SubOp}) == IntValue(34)
    }

    #[test]
    fn test_5() {
        let expr = Expr::BoolExpr(ArithCmpExpr{left: Box::new(BinArithExpr{left: Box::new(IntLit(12)), right: Box::new(IntLit(5)), op: MulOp}), right: Box::new(IntLit(167)), op: LtOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // Expr::BoolExpr(ArithCmpExpr{left: Box::new(BinArithExpr{left: Box::new(IntLit(12)), right: Box::new(IntLit(5)), op: MulOp}), right: Box::new(IntLit(167)), op: LtOp}) == BoolValue(true)
    }

    #[test]
    fn test_6() {
        let expr = Expr::BoolExpr(ArithCmpExpr{left: Box::new(BinArithExpr{left: Box::new(IntLit(12)), right: Box::new(IntLit(5)), op: MulOp}), right: Box::new(IntLit(10)), op: GtOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // Expr::BoolExpr(ArithCmpExpr{left: Box::new(BinArithExpr{left: Box::new(IntLit(12)), right: Box::new(IntLit(5)), op: MulOp}), right: Box::new(IntLit(10)), op: GtOp}) == BoolValue(true)
    }

    #[test]
    fn test_7() {
        let expr = Expr::BoolExpr(ArithCmpExpr{left: Box::new(IntLit(60)), right: Box::new(IntLit(60)), op: ArithEqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // Expr::BoolExpr(ArithCmpExpr{left: Box::new(IntLit(60)), right: Box::new(IntLit(60)), op: ArithEqOp}) == BoolValue(true)
    }

    #[test]
    fn test_8() {
        let expr = Expr::BoolExpr(ArithCmpExpr{left: Box::new(IntLit(10)), right: Box::new(IntLit(60)), op: GteOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);  // Expr::BoolExpr(ArithCmpExpr{left: left: Box::new(IntLit(10)), right: Box::new(IntLit(60)), op: GteOp}) == BoolValue(false)
    }

    #[test]
    fn test_9() {
        let expr = Expr::BoolExpr(ArithCmpExpr{left: Box::new(IntLit(10)), right: Box::new(IntLit(60)), op: ArithNeqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // Expr::BoolExpr((ArithCmpExpr{left: left: Box::new(IntLit(10)), right: Box::new(IntLit(60)), op: ArithNeqOp}) == BoolValue(true)
    }

    #[test]
    fn test_10() {
        let expr = Expr::BoolExpr(BinBoolExpr{left: Box::new(NotExpr(Box::new(BoolLit(true)))), right: Box::new(BoolLit(true)), op: BoolNeqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }



    #[test]
    fn test_11() {
        let expr = Expr::BoolExpr(BinBoolExpr{left: Box::new(NotExpr(Box::new(BoolLit(true)))), right: Box::new(BoolLit(true)), op: AndOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_12() {
        let expr = Expr::BoolExpr(BinBoolExpr{left: Box::new(NotExpr(Box::new(BoolLit(false)))), right: Box::new(BoolLit(true)), op: OrOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }


}

fn main() {
}