use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

use crate::{AnyNumeric, Numeric};

unsafe impl<const P: u32, const S: u32> SqlTranslatable for Numeric<P, S> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match (P, S) {
            (0, 0) => Ok(SqlMapping::literal("NUMERIC")),
            (p, 0) => Ok(SqlMapping::As(format!("NUMERIC({p})"))),
            (p, s) => Ok(SqlMapping::As(format!("NUMERIC({p}, {s})"))),
        }
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        match (P, S) {
            (0, 0) => Ok(Returns::One(SqlMapping::literal("NUMERIC"))),
            (p, 0) => Ok(Returns::One(SqlMapping::As(format!("NUMERIC({p})")))),
            (p, s) => Ok(Returns::One(SqlMapping::As(format!("NUMERIC({p}, {s})")))),
        }
    }
}

unsafe impl SqlTranslatable for AnyNumeric {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("NUMERIC"))
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("NUMERIC")))
    }
}
