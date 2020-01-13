use crate::{pg_sys, FromDatum, IntoDatum};

#[derive(Debug, Clone, Copy)]
pub struct AnyElement {
    datum: pg_sys::Datum,
    typoid: pg_sys::Oid,
}

impl AnyElement {
    pub fn datum(&self) -> pg_sys::Datum {
        self.datum
    }

    pub fn oid(&self) -> pg_sys::Oid {
        self.typoid
    }

    #[inline]
    pub fn into<T: FromDatum<T>>(&self) -> Option<T> {
        T::from_datum(self.datum(), false, self.oid())
    }
}

impl FromDatum<AnyElement> for AnyElement {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: pg_sys::Oid) -> Option<AnyElement> {
        if is_null {
            None
        } else {
            Some(AnyElement { datum, typoid })
        }
    }
}

impl IntoDatum<AnyElement> for AnyElement {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.datum)
    }
}