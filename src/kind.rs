use serde::de::{
    DeserializeSeed, Deserializer, EnumAccess, Expected, IntoDeserializer, Unexpected,
    VariantAccess, Visitor,
};
use serde::ser::{Serialize, Serializer};
use serde::{forward_to_deserialize_any, Deserialize};
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
    AllocAlignAttr,
    AllocSizeAttr,
    AlwaysInlineAttr,
    ArrayInitIndexExpr,
    ArrayInitLoopExpr,
    ArraySubscriptExpr,
    ArrayTypeTraitExpr,
    AsmLabelAttr,
    AtomicExpr,
    AtomicType,
    AttributedStmt,
    AttributedType,
    AutoType,
    AvailabilityAttr,
    BinaryOperator,
    BindingDecl,
    BlockPointerType,
    BreakStmt,
    BuiltinAttr,
    BuiltinBitCastExpr,
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
    CXXDeductionGuideDecl,
    CXXDefaultArgExpr,
    CXXDefaultInitExpr,
    CXXDeleteExpr,
    CXXDependentScopeMemberExpr,
    CXXDestructorDecl,
    CXXDynamicCastExpr,
    CXXFoldExpr,
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
    CXXRewrittenBinaryOperator,
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
    ColdAttr,
    ComplexType,
    CompoundAssignOperator,
    CompoundLiteralExpr,
    CompoundRequirement,
    CompoundStmt,
    ConceptDecl,
    ConceptSpecializationExpr,
    ConditionalOperator,
    ConstAttr,
    ConstantArrayType,
    ConstantExpr,
    ConstructorUsingShadowDecl,
    ContinueStmt,
    ConvertVectorExpr,
    DLLImportAttr,
    DecayedType,
    DeclRefExpr,
    DeclStmt,
    DecltypeType,
    DecompositionDecl,
    DefaultStmt,
    DependentNameType,
    DependentScopeDeclRefExpr,
    DependentSizedArrayType,
    DependentTemplateSpecializationType,
    DeprecatedAttr,
    DiagnoseIfAttr,
    DisableTailCallsAttr,
    DoStmt,
    ElaboratedType,
    EmptyDecl,
    EnableIfAttr,
    EnumConstantDecl,
    EnumDecl,
    EnumType,
    ExprWithCleanups,
    FallThroughAttr,
    FieldDecl,
    FinalAttr,
    FloatingLiteral,
    ForStmt,
    FormatArgAttr,
    FormatAttr,
    FriendDecl,
    FullComment,
    FunctionDecl,
    FunctionProtoType,
    FunctionTemplateDecl,
    GCCAsmStmt,
    GNUInlineAttr,
    GNUNullExpr,
    GotoStmt,
    HTMLEndTagComment,
    HTMLStartTagComment,
    IfStmt,
    ImplicitCastExpr,
    ImplicitValueInitExpr,
    IncompleteArrayType,
    IndirectFieldDecl,
    InitListExpr,
    InjectedClassNameType,
    InlineCommandComment,
    IntegerLiteral,
    InternalLinkageAttr,
    LValueReferenceType,
    LabelStmt,
    LambdaExpr,
    LikelyAttr,
    LinkageSpecDecl,
    MaterializeTemporaryExpr,
    MaxFieldAlignmentAttr,
    MayAliasAttr,
    MemberExpr,
    MemberPointerType,
    MinVectorWidthAttr,
    ModeAttr,
    NamespaceAliasDecl,
    NamespaceDecl,
    NestedRequirement,
    NoAliasAttr,
    NoDebugAttr,
    NoEscapeAttr,
    NoInlineAttr,
    NoSanitizeAttr,
    NoThrowAttr,
    NoUniqueAddressAttr,
    NonNullAttr,
    NonTypeTemplateParmDecl,
    NullStmt,
    OffsetOfExpr,
    OpaqueValueExpr,
    OverrideAttr,
    OwnerAttr,
    PackExpansionExpr,
    PackExpansionType,
    PackedAttr,
    ParagraphComment,
    ParamCommandComment,
    ParenExpr,
    ParenListExpr,
    ParenType,
    ParmVarDecl,
    PointerAttr,
    PointerType,
    PredefinedExpr,
    PreferredNameAttr,
    PureAttr,
    QualType,
    RValueReferenceType,
    RecordDecl,
    RecordType,
    RecoveryExpr,
    RequiresExpr,
    RestrictAttr,
    ReturnStmt,
    ReturnsNonNullAttr,
    ReturnsTwiceAttr,
    ShuffleVectorExpr,
    SimpleRequirement,
    SizeOfPackExpr,
    StaticAssertDecl,
    StringLiteral,
    SubstNonTypeTemplateParmExpr,
    SubstTemplateTypeParmType,
    SwiftAttrAttr,
    SwitchStmt,
    TargetAttr,
    TemplateArgument,
    TemplateSpecializationType,
    TemplateTemplateParmDecl,
    TemplateTypeParmDecl,
    TemplateTypeParmType,
    TextComment,
    TranslationUnitDecl,
    TypeAliasDecl,
    TypeAliasTemplateDecl,
    TypeOfExprType,
    TypeRequirement,
    TypeTraitExpr,
    TypeVisibilityAttr,
    TypedefDecl,
    TypedefType,
    UnaryExprOrTypeTraitExpr,
    UnaryOperator,
    UnaryTransformType,
    UnavailableAttr,
    UnlikelyAttr,
    UnresolvedLookupExpr,
    UnresolvedMemberExpr,
    UnresolvedUsingIfExistsDecl,
    UnresolvedUsingTypenameDecl,
    UnresolvedUsingValueDecl,
    UnusedAttr,
    UserDefinedLiteral,
    UsingDecl,
    UsingDirectiveDecl,
    UsingEnumDecl,
    UsingIfExistsAttr,
    UsingShadowDecl,
    UsingType,
    VTablePointerAuthenticationAttr,
    VarDecl,
    VarTemplateDecl,
    VarTemplatePartialSpecializationDecl,
    VarTemplateSpecializationDecl,
    VectorType,
    VerbatimLineComment,
    VisibilityAttr,
    WarnUnusedResultAttr,
    WeakAttr,
    WeakImportAttr,
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

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Kind::null = self {
            serializer.serialize_unit()
        } else {
            serializer.serialize_str(self.as_str())
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
    Borrowed(&'de str),
    Owned(Box<str>),
}

impl<'de> AnyKind<'de> {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            AnyKind::Kind(kind) => kind.as_str(),
            AnyKind::Borrowed(kind) => kind,
            AnyKind::Owned(kind) => kind,
        }
    }
}

impl<'de> Display for AnyKind<'de> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.as_str())
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
            Err(UnknownVariant) => Ok(AnyKind::Owned(Box::from(kind))),
        }
    }

    fn visit_borrowed_str<E>(self, kind: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Kind::deserialize(kind.into_deserializer()) {
            Ok(kind) => Ok(AnyKind::Kind(kind)),
            Err(UnknownVariant) => Ok(AnyKind::Borrowed(kind)),
        }
    }

    fn visit_string<E>(self, kind: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Kind::deserialize(kind.as_str().into_deserializer()) {
            Ok(kind) => Ok(AnyKind::Kind(kind)),
            Err(UnknownVariant) => Ok(AnyKind::Owned(kind.into_boxed_str())),
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

enum SometimesBorrowedStr<'a, 'de> {
    Transient(&'a str),
    Borrowed(&'de str),
}

pub(crate) struct SometimesBorrowedStrDeserializer<'a, 'de, E> {
    value: SometimesBorrowedStr<'a, 'de>,
    error: PhantomData<E>,
}

impl<'a, 'de, E> SometimesBorrowedStrDeserializer<'a, 'de, E> {
    pub(crate) fn transient(value: &'a str) -> Self {
        SometimesBorrowedStrDeserializer {
            value: SometimesBorrowedStr::Transient(value),
            error: PhantomData,
        }
    }

    pub(crate) fn borrowed(value: &'de str) -> Self {
        SometimesBorrowedStrDeserializer {
            value: SometimesBorrowedStr::Borrowed(value),
            error: PhantomData,
        }
    }
}

impl<'a, 'de, E> Deserializer<'de> for SometimesBorrowedStrDeserializer<'a, 'de, E>
where
    E: serde::de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            SometimesBorrowedStr::Transient(string) => visitor.visit_str(string),
            SometimesBorrowedStr::Borrowed(string) => visitor.visit_borrowed_str(string),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
        map struct identifier ignored_any
    }
}

impl<'a, 'de, E> EnumAccess<'de> for SometimesBorrowedStrDeserializer<'a, 'de, E>
where
    E: serde::de::Error,
{
    type Error = E;
    type Variant = UnitOnly<E>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(self)?;
        let variant = UnitOnly {
            marker: PhantomData,
        };
        Ok((value, variant))
    }
}

pub(crate) struct UnitOnly<E> {
    marker: PhantomData<E>,
}

impl<'de, E> VariantAccess<'de> for UnitOnly<E>
where
    E: serde::de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let _ = seed;
        Err(E::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = len;
        let _ = visitor;
        Err(E::invalid_type(Unexpected::UnitVariant, &"tuple variant"))
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = fields;
        let _ = visitor;
        Err(E::invalid_type(Unexpected::UnitVariant, &"struct variant"))
    }
}
