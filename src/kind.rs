use serde::de::{Deserializer, Expected, IntoDeserializer, Unexpected, Visitor};
use serde::{forward_to_deserialize_any, Deserialize};
use std::borrow::Cow;
use std::fmt::{self, Debug, Display};
use std::marker::PhantomData;
use std::str::FromStr;

macro_rules! kind {
    ($($kind:ident,)*) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[non_exhaustive]
        pub enum Kind {
            $(
                $kind,
            )*
            #[allow(non_camel_case_types)]
            null,
        }

        impl Kind {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Kind::$kind => stringify!($kind),
                    )*
                    Kind::null => "null",
                }
            }
        }

        impl FromStr for Kind {
            type Err = ParseKindError;

            fn from_str(kind: &str) -> Result<Self, Self::Err> {
                match kind {
                    $(
                        stringify!($kind) => Ok(Kind::$kind),
                    )*
                    _other => Err(ParseKindError { _private: () }),
                }
            }
        }

        static VARIANTS: &'static [&'static str] = &[
            $(
                stringify!($kind),
            )*
        ];
    };
}

kind! {
    AbiTagAttr,
    AccessSpecDecl,
    AliasAttr,
    AlignedAttr,
    AllocSizeAttr,
    AlwaysInlineAttr,
    ArrayInitIndexExpr,
    ArrayInitLoopExpr,
    ArraySubscriptExpr,
    AsmLabelAttr,
    AtomicExpr,
    BinaryOperator,
    BreakStmt,
    BuiltinTemplateDecl,
    BuiltinType,
    CStyleCastExpr,
    CXX11NoReturnAttr,
    CXXBindTemporaryExpr,
    CXXBoolLiteralExpr,
    CXXCatchStmt,
    CXXConstCastExpr,
    CXXConstructExpr,
    CXXConstructorDecl,
    CXXConversionDecl,
    CXXCtorInitializer,
    CXXDefaultArgExpr,
    CXXDefaultInitExpr,
    CXXDeleteExpr,
    CXXDependentScopeMemberExpr,
    CXXDestructorDecl,
    CXXDynamicCastExpr,
    CXXForRangeStmt,
    CXXFunctionalCastExpr,
    CXXInheritedCtorInitExpr,
    CXXMemberCallExpr,
    CXXMethodDecl,
    CXXNewExpr,
    CXXNoexceptExpr,
    CXXNullPtrLiteralExpr,
    CXXOperatorCallExpr,
    CXXPseudoDestructorExpr,
    CXXRecordDecl,
    CXXReinterpretCastExpr,
    CXXScalarValueInitExpr,
    CXXStaticCastExpr,
    CXXTemporaryObjectExpr,
    CXXThisExpr,
    CXXThrowExpr,
    CXXTryStmt,
    CXXTypeidExpr,
    CXXUnresolvedConstructExpr,
    CallExpr,
    CallbackAttr,
    CaseStmt,
    CharacterLiteral,
    ClassTemplateDecl,
    ClassTemplatePartialSpecializationDecl,
    ClassTemplateSpecializationDecl,
    ComplexType,
    CompoundAssignOperator,
    CompoundStmt,
    ConditionalOperator,
    ConstAttr,
    ConstantArrayType,
    ConstantExpr,
    ConstructorUsingShadowDecl,
    ContinueStmt,
    DeclRefExpr,
    DeclStmt,
    DecltypeType,
    DefaultStmt,
    DependentNameType,
    DependentScopeDeclRefExpr,
    DependentSizedArrayType,
    DependentTemplateSpecializationType,
    DeprecatedAttr,
    DoStmt,
    ElaboratedType,
    EmptyDecl,
    EnumConstantDecl,
    EnumDecl,
    EnumType,
    ExprWithCleanups,
    FieldDecl,
    FinalAttr,
    FloatingLiteral,
    ForStmt,
    FormatArgAttr,
    FormatAttr,
    FriendDecl,
    FunctionDecl,
    FunctionProtoType,
    FunctionTemplateDecl,
    GNUNullExpr,
    IfStmt,
    ImplicitCastExpr,
    ImplicitValueInitExpr,
    IncompleteArrayType,
    IndirectFieldDecl,
    InitListExpr,
    InjectedClassNameType,
    IntegerLiteral,
    LValueReferenceType,
    LambdaExpr,
    LinkageSpecDecl,
    MaterializeTemporaryExpr,
    MayAliasAttr,
    MemberExpr,
    MemberPointerType,
    ModeAttr,
    NamespaceDecl,
    NoThrowAttr,
    NonNullAttr,
    NonTypeTemplateParmDecl,
    NullStmt,
    OpaqueValueExpr,
    OverrideAttr,
    OwnerAttr,
    PackExpansionExpr,
    PackExpansionType,
    ParenExpr,
    ParenListExpr,
    ParenType,
    ParmVarDecl,
    PointerAttr,
    PointerType,
    PredefinedExpr,
    PureAttr,
    QualType,
    RValueReferenceType,
    RecordType,
    RestrictAttr,
    ReturnStmt,
    ReturnsNonNullAttr,
    ReturnsTwiceAttr,
    SizeOfPackExpr,
    StaticAssertDecl,
    StringLiteral,
    SubstNonTypeTemplateParmExpr,
    SubstTemplateTypeParmType,
    SwitchStmt,
    TemplateArgument,
    TemplateSpecializationType,
    TemplateTemplateParmDecl,
    TemplateTypeParmDecl,
    TemplateTypeParmType,
    TranslationUnitDecl,
    TypeAliasDecl,
    TypeAliasTemplateDecl,
    TypeOfExprType,
    TypeTraitExpr,
    TypedefDecl,
    TypedefType,
    UnaryExprOrTypeTraitExpr,
    UnaryOperator,
    UnaryTransformType,
    UnresolvedLookupExpr,
    UnresolvedMemberExpr,
    UnresolvedUsingValueDecl,
    UnusedAttr,
    UsingDecl,
    UsingDirectiveDecl,
    UsingShadowDecl,
    VarDecl,
    VarTemplateDecl,
    VarTemplateSpecializationDecl,
    VisibilityAttr,
    WarnUnusedResultAttr,
    WeakRefAttr,
    WhileStmt,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::null
    }
}

impl Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Debug for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(KindVisitor)
    }
}

struct KindVisitor;

impl<'de> Visitor<'de> for KindVisitor {
    type Value = Kind;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum Kind")
    }

    fn visit_str<E>(self, kind: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Kind::from_str(kind) {
            Ok(kind) => Ok(kind),
            Err(ParseKindError { .. }) => {
                Err(serde::de::Error::unknown_variant(kind, self::VARIANTS))
            }
        }
    }
}

pub struct ParseKindError {
    _private: (),
}

impl Debug for ParseKindError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_struct("ParseKindError").finish()
    }
}

impl Display for ParseKindError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("unrecognized clang syntax tree node kind")
    }
}

impl std::error::Error for ParseKindError {}

pub(crate) enum AnyKind<'de> {
    Kind(Kind),
    Other(Cow<'de, str>),
}

impl<'de> AnyKind<'de> {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            AnyKind::Kind(kind) => kind.as_str(),
            AnyKind::Other(kind) => kind.as_ref(),
        }
    }
}

impl<'de> Deserialize<'de> for AnyKind<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let marker = PhantomData;
        let visitor = AnyKindVisitor { marker };
        deserializer.deserialize_str(visitor)
    }
}

struct AnyKindVisitor<'de> {
    marker: PhantomData<fn() -> AnyKind<'de>>,
}

impl<'de> Visitor<'de> for AnyKindVisitor<'de> {
    type Value = AnyKind<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum Kind")
    }

    fn visit_str<E>(self, kind: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Kind::deserialize(kind.into_deserializer()) {
            Ok(kind) => Ok(AnyKind::Kind(kind)),
            Err(UnknownVariant) => Ok(AnyKind::Other(Cow::Owned(kind.to_owned()))),
        }
    }

    fn visit_borrowed_str<E>(self, kind: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Kind::deserialize(kind.into_deserializer()) {
            Ok(kind) => Ok(AnyKind::Kind(kind)),
            Err(UnknownVariant) => Ok(AnyKind::Other(Cow::Borrowed(kind))),
        }
    }

    fn visit_string<E>(self, kind: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Kind::deserialize(kind.as_str().into_deserializer()) {
            Ok(kind) => Ok(AnyKind::Kind(kind)),
            Err(UnknownVariant) => Ok(AnyKind::Other(Cow::Owned(kind))),
        }
    }
}

#[derive(Debug)]
struct UnknownVariant;

impl std::error::Error for UnknownVariant {}

impl serde::de::Error for UnknownVariant {
    fn custom<T: Display>(msg: T) -> Self {
        let _ = msg;
        unreachable!()
    }

    fn invalid_type(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self {
        let _ = unexp;
        let _ = exp;
        unreachable!()
    }

    fn invalid_value(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self {
        let _ = unexp;
        let _ = exp;
        unreachable!()
    }

    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
        let _ = len;
        let _ = exp;
        unreachable!()
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        let _ = variant;
        let _ = expected;
        UnknownVariant
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        let _ = field;
        let _ = expected;
        unreachable!()
    }

    fn missing_field(field: &'static str) -> Self {
        let _ = field;
        unreachable!()
    }

    fn duplicate_field(field: &'static str) -> Self {
        let _ = field;
        unreachable!()
    }
}

impl Display for UnknownVariant {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("unknown variant")
    }
}

pub(crate) struct BorrowedCowStrDeserializer<'a, 'de, E> {
    value: &'a Cow<'de, str>,
    error: PhantomData<E>,
}

impl<'a, 'de, E> BorrowedCowStrDeserializer<'a, 'de, E> {
    pub(crate) fn new(value: &'a Cow<'de, str>) -> Self {
        BorrowedCowStrDeserializer {
            value,
            error: PhantomData,
        }
    }
}

impl<'a, 'de, E> Deserializer<'de> for BorrowedCowStrDeserializer<'a, 'de, E>
where
    E: serde::de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Cow::Borrowed(string) => visitor.visit_borrowed_str(string),
            Cow::Owned(string) => visitor.visit_str(string),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier ignored_any
    }
}
