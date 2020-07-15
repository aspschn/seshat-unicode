pub mod props;

pub(crate) mod ucd;

use self::props::*;

#[derive(Clone, Copy)]
pub struct CodePoint {
    code_point: u32,
}

impl CodePoint {
    pub fn new(cp: u32) -> Result<CodePoint, &'static str> {
        if cp > 0x10FFFF {
            return Err("IllegalCodePoint: Code point cannot be over U+10FFFF.");
        }
        Ok(CodePoint { code_point: cp })
    }

    pub fn to_u32(&self) -> u32 {
        self.code_point
    }
}

pub trait ToCodePoint {
    fn to_code_point(&self) -> CodePoint;
}

impl ToCodePoint for char {
    fn to_code_point(&self) -> CodePoint {
        CodePoint::new(*self as u32).unwrap()
    }
}

impl std::fmt::Display for CodePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "U+{:04X}", self.code_point)
    }
}

pub trait Ucd {
    fn na(&self) -> String;
    fn gc(&self) -> Gc;
    fn hst(&self) -> Hst;
    fn wspace(&self) -> bool;
    fn bidi_c(&self) -> bool;
    fn join_c(&self) -> bool;
    fn dash(&self) -> bool;
    fn hyphen(&self) -> bool;
    fn qmark(&self) -> bool;
    fn term(&self) -> bool;
    fn omath(&self) -> bool;
    fn hex(&self) -> bool;
    fn ahex(&self) -> bool;
    fn oalpha(&self) -> bool;
    fn ideo(&self) -> bool;
    fn dia(&self) -> bool;
    fn ext(&self) -> bool;
    fn olower(&self) -> bool;
    fn oupper(&self) -> bool;
    fn nchar(&self) -> bool;
    fn ogr_ext(&self) -> bool;
    fn idsb(&self) -> bool;
    fn idst(&self) -> bool;
    fn radical(&self) -> bool;
    fn uideo(&self) -> bool;
    fn odi(&self) -> bool;
    fn dep(&self) -> bool;
    fn sd(&self) -> bool;
    fn loe(&self) -> bool;
    fn oids(&self) -> bool;
    fn oidc(&self) -> bool;
    fn sterm(&self) -> bool;
    fn vs(&self) -> bool;
    fn pat_ws(&self) -> bool;
    fn pat_syn(&self) -> bool;
    fn pcm(&self) -> bool;
    fn ri(&self) -> bool;
}

impl Ucd for CodePoint {
    fn na(&self) -> String {
        ucd::na::na(self.code_point)
    }

    fn gc(&self) -> Gc {
        ucd::gc::gc(self.code_point)
    }

    fn hst(&self) -> Hst {
        ucd::hst::hst(self.code_point)
    }

    fn wspace(&self) -> bool {
        ucd::binary_props::wspace(self.code_point)
    }

    fn bidi_c(&self) -> bool {
        ucd::binary_props::bidi_c(self.code_point)
    }

    fn join_c(&self) -> bool {
        ucd::binary_props::join_c(self.code_point)
    }

    fn dash(&self) -> bool {
        ucd::binary_props::dash(self.code_point)
    }

    fn hyphen(&self) -> bool {
        ucd::binary_props::hyphen(self.code_point)
    }

    fn qmark(&self) -> bool {
        ucd::binary_props::qmark(self.code_point)
    }

    fn term(&self) -> bool {
        ucd::binary_props::term(self.code_point)
    }

    fn omath(&self) -> bool {
        ucd::binary_props::omath(self.code_point)
    }

    fn hex(&self) -> bool {
        ucd::binary_props::hex(self.code_point)
    }

    fn ahex(&self) -> bool {
        ucd::binary_props::ahex(self.code_point)
    }

    fn oalpha(&self) -> bool {
        ucd::binary_props::oalpha(self.code_point)
    }

    fn ideo(&self) -> bool {
        ucd::binary_props::ideo(self.code_point)
    }

    fn dia(&self) -> bool {
        ucd::binary_props::dia(self.code_point)
    }

    fn ext(&self) -> bool {
        ucd::binary_props::ext(self.code_point)
    }

    fn olower(&self) -> bool {
        ucd::binary_props::olower(self.code_point)
    }

    fn oupper(&self) -> bool {
        ucd::binary_props::oupper(self.code_point)
    }

    fn nchar(&self) -> bool {
        ucd::binary_props::nchar(self.code_point)
    }

    fn ogr_ext(&self) -> bool {
        ucd::binary_props::ogr_ext(self.code_point)
    }

    fn idsb(&self) -> bool {
        ucd::binary_props::idsb(self.code_point)
    }

    fn idst(&self) -> bool {
        ucd::binary_props::idst(self.code_point)
    }

    fn radical(&self) -> bool {
        ucd::binary_props::radical(self.code_point)
    }

    fn uideo(&self) -> bool {
        ucd::binary_props::uideo(self.code_point)
    }

    fn odi(&self) -> bool {
        ucd::binary_props::odi(self.code_point)
    }

    fn dep(&self) -> bool {
        ucd::binary_props::dep(self.code_point)
    }

    fn sd(&self) -> bool {
        ucd::binary_props::sd(self.code_point)
    }

    fn loe(&self) -> bool {
        ucd::binary_props::loe(self.code_point)
    }

    fn oids(&self) -> bool {
        ucd::binary_props::oids(self.code_point)
    }

    fn oidc(&self) -> bool {
        ucd::binary_props::oidc(self.code_point)
    }

    fn sterm(&self) -> bool {
        ucd::binary_props::sterm(self.code_point)
    }

    fn vs(&self) -> bool {
        ucd::binary_props::vs(self.code_point)
    }

    fn pat_ws(&self) -> bool {
        ucd::binary_props::pat_ws(self.code_point)
    }

    fn pat_syn(&self) -> bool {
        ucd::binary_props::pat_syn(self.code_point)
    }

    fn pcm(&self) -> bool {
        ucd::binary_props::pcm(self.code_point)
    }

    fn ri(&self) -> bool {
        ucd::binary_props::ri(self.code_point)
    }
}

impl Ucd for char {
    fn na(&self) -> String {
        ucd::na::na(*self as u32)
    }

    fn gc(&self) -> Gc {
        ucd::gc::gc(*self as u32)
    }

    fn hst(&self) -> Hst {
        ucd::hst::hst(*self as u32)
    }

    fn wspace(&self) -> bool {
        ucd::binary_props::wspace(*self as u32)
    }

    fn bidi_c(&self) -> bool {
        ucd::binary_props::bidi_c(*self as u32)
    }

    fn join_c(&self) -> bool {
        ucd::binary_props::join_c(*self as u32)
    }

    fn dash(&self) -> bool {
        ucd::binary_props::dash(*self as u32)
    }

    fn hyphen(&self) -> bool {
        ucd::binary_props::hyphen(*self as u32)
    }

    fn qmark(&self) -> bool {
        ucd::binary_props::qmark(*self as u32)
    }

    fn term(&self) -> bool {
        ucd::binary_props::term(*self as u32)
    }

    fn omath(&self) -> bool {
        ucd::binary_props::omath(*self as u32)
    }

    fn hex(&self) -> bool {
        ucd::binary_props::hex(*self as u32)
    }

    fn ahex(&self) -> bool {
        ucd::binary_props::ahex(*self as u32)
    }

    fn oalpha(&self) -> bool {
        ucd::binary_props::oalpha(*self as u32)
    }

    fn ideo(&self) -> bool {
        ucd::binary_props::ideo(*self as u32)
    }

    fn dia(&self) -> bool {
        ucd::binary_props::dia(*self as u32)
    }

    fn ext(&self) -> bool {
        ucd::binary_props::ext(*self as u32)
    }

    fn olower(&self) -> bool {
        ucd::binary_props::olower(*self as u32)
    }

    fn oupper(&self) -> bool {
        ucd::binary_props::oupper(*self as u32)
    }

    fn nchar(&self) -> bool {
        ucd::binary_props::nchar(*self as u32)
    }

    fn ogr_ext(&self) -> bool {
        ucd::binary_props::ogr_ext(*self as u32)
    }

    fn idsb(&self) -> bool {
        ucd::binary_props::idsb(*self as u32)
    }

    fn idst(&self) -> bool {
        ucd::binary_props::idst(*self as u32)
    }

    fn radical(&self) -> bool {
        ucd::binary_props::radical(*self as u32)
    }

    fn uideo(&self) -> bool {
        ucd::binary_props::uideo(*self as u32)
    }

    fn odi(&self) -> bool {
        ucd::binary_props::odi(*self as u32)
    }

    fn dep(&self) -> bool {
        ucd::binary_props::dep(*self as u32)
    }

    fn sd(&self) -> bool {
        ucd::binary_props::sd(*self as u32)
    }

    fn loe(&self) -> bool {
        ucd::binary_props::loe(*self as u32)
    }

    fn oids(&self) -> bool {
        ucd::binary_props::oids(*self as u32)
    }

    fn oidc(&self) -> bool {
        ucd::binary_props::oidc(*self as u32)
    }

    fn sterm(&self) -> bool {
        ucd::binary_props::sterm(*self as u32)
    }

    fn vs(&self) -> bool {
        ucd::binary_props::vs(*self as u32)
    }

    fn pat_ws(&self) -> bool {
        ucd::binary_props::pat_ws(*self as u32)
    }

    fn pat_syn(&self) -> bool {
        ucd::binary_props::pat_syn(*self as u32)
    }

    fn pcm(&self) -> bool {
        ucd::binary_props::pcm(*self as u32)
    }

    fn ri(&self) -> bool {
        ucd::binary_props::ri(*self as u32)
    }
}