use crate::lexer::SyntaxKind;
use cstree::Language;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Lang;

impl Language for Lang {
    type Kind = SyntaxKind;

    fn kind_to_raw(kind: Self::Kind) -> cstree::SyntaxKind {
        cstree::SyntaxKind(kind as u16)
    }

    fn kind_from_raw(raw: cstree::SyntaxKind) -> Self::Kind {
        unsafe { transmute::<u16, SyntaxKind>(raw.0) }
    }
}

pub type SyntaxNode = cstree::SyntaxNode<Lang>;
