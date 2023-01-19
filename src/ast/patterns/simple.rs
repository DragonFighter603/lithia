use crate::ast::{AstLiteral, Ident};
use crate::ast::patterns::{Consumer, Pat};
use crate::error::{OnParseErr, ParseError, ParseET};
use crate::tokens::{Token, TokenType, TokIter, Literal, glued};

pub(crate) struct Wrap<Out>(pub(crate) Pat<Out>);

impl<Out> Consumer for Wrap<Out> {
    type Output = Out;
    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        self.0.consume(iter)
    }
}

pub(crate) struct GetNext;

impl Consumer for GetNext{
    type Output = Token;
    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let t = iter.this();
        iter.next();
        t
    }
}


pub(crate) struct ExpectIdent(pub(crate) String);
impl Consumer for ExpectIdent {
    type Output = ();

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Ident(s) = tt {
            if s == self.0 {
                iter.next();
                Ok(())
            } else {
                Err(ParseET::ParsingError(format!("expected '{}', found {}", self.0, s)).at(loc))
            }
        } else {
            Err(ParseET::ParsingError(format!("expected '{}', found {:?}", self.0, tt)).at(loc))
        }
    }
}
pub(crate) struct GetIdent;
impl Consumer for GetIdent {
    type Output = Ident;

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Ident(s) = tt {
            iter.next();
            Ok(Ident(s, loc))
        } else {
            Err(ParseET::ParsingError(format!("expected Ident, found {:?}", tt)).at(loc))
        }
    }
}

pub(crate) struct ExpectParticle(pub(crate) char);
impl Consumer for ExpectParticle {
    type Output = ();

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Particle(c, _) = tt {
            if c == self.0 {
                iter.next();
                Ok(())
            } else {
                Err(ParseET::ParsingError(format!("expected '{}', found '{}'", self.0, c)).at(loc))
            }
        } else {
            Err(ParseET::ParsingError(format!("expected '{}', found {:?}", self.0, tt)).at(loc))
        }
    }
}

pub(crate) struct ExpectParticleExact(pub(crate) char, pub(crate) glued);
impl Consumer for ExpectParticleExact {
    type Output = ();

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Particle(c, g) = tt {
            if c == self.0 && g == self.1 {
                iter.next();
                Ok(())
            } else {
                Err(ParseET::ParsingError(format!("expected ('{}', {}), found ('{}', {})", self.0, self.1, c, g)).at(loc))
            }
        } else {
            Err(ParseET::ParsingError(format!("expected ('{}', {}), found ({:?})", self.0, self.1, tt)).at(loc))
        }.e_when("parsing ident")
    }
}
pub(crate) struct GetParticle;
impl Consumer for GetParticle {
    type Output = (char, glued);

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Particle(p, g) = tt {
            iter.next();
            Ok((p, g))
        } else {
            Err(ParseET::ParsingError(format!("expected Particle, found {:?}", tt)).at(loc))
        }
    }
}
pub(crate) struct GetGluedParticle;
impl Consumer for GetGluedParticle {
    type Output = char;

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Particle(p, true) = tt {
            iter.next();
            Ok(p)
        } else {
            Err(ParseET::ParsingError(format!("expected glued Particle, found {:?}", tt)).at(loc))
        }
    }
}
pub(crate) struct ExpectLiteral(pub(crate) Literal);
impl Consumer for ExpectLiteral {
    type Output = ();

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Literal(lit) = tt {
            if lit == self.0 {
                iter.next();
                Ok(())
            } else {
                Err(ParseET::ParsingError(format!("expected {:?}, found {:?}", self.0, lit)).at(loc))
            }
        } else {
            Err(ParseET::ParsingError(format!("expected {:?}, found {:?}", self.0, tt)).at(loc))
        }
    }
}
pub(crate) struct GetLiteral;
impl Consumer for GetLiteral {
    type Output = AstLiteral;

    fn consume(&self, iter: &mut TokIter) -> Result<Self::Output, ParseError> {
        let Token { tt, loc } = iter.this()?;
        if let TokenType::Literal(lit) = tt {
            iter.next();
            Ok(AstLiteral(lit, loc))
        } else {
            Err(ParseET::ParsingError(format!("expected Literal, found {:?}", tt)).at(loc))
        }
    }
}

