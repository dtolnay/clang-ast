#![allow(
    clippy::derivable_impls,
    clippy::doc_markdown,
    clippy::large_enum_variant,
    clippy::struct_excessive_bools,
    clippy::uninlined_format_args
)]

use clang_ast::{Id, Kind, SourceLocation, SourceRange};
use monostate::MustBe;
use serde_derive::Deserialize;
use std::env;
use std::io::{self, Write as _};
use std::str;
use std::thread::{self, Builder as ThreadBuilder};

pub type Node = clang_ast::Node<Clang>;

#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub enum Clang {
    AbiTagAttr(AbiTagAttr),
    AccessSpecDecl(AccessSpecDecl),
    AliasAttr(AliasAttr),
    AlignedAttr(AlignedAttr),
    AllocAlignAttr(AllocAlignAttr),
    AllocSizeAttr(AllocSizeAttr),
    AlwaysInlineAttr(AlwaysInlineAttr),
    ArrayInitIndexExpr(ArrayInitIndexExpr),
    ArrayInitLoopExpr(ArrayInitLoopExpr),
    ArraySubscriptExpr(ArraySubscriptExpr),
    ArrayTypeTraitExpr(ArrayTypeTraitExpr),
    AsmLabelAttr(AsmLabelAttr),
    AtomicExpr(AtomicExpr),
    AtomicType(AtomicType),
    AttributedStmt(AttributedStmt),
    AttributedType(AttributedType),
    AutoType(AutoType),
    AvailabilityAttr(AvailabilityAttr),
    AvailableOnlyInDefaultEvalMethodAttr(AvailableOnlyInDefaultEvalMethodAttr),
    BinaryConditionalOperator(BinaryConditionalOperator),
    BinaryOperator(BinaryOperator),
    BindingDecl(BindingDecl),
    BlockCommandComment(BlockCommandComment),
    BlockPointerType(BlockPointerType),
    BreakStmt(BreakStmt),
    BuiltinAttr(BuiltinAttr),
    BuiltinBitCastExpr(BuiltinBitCastExpr),
    BuiltinTemplateDecl(BuiltinTemplateDecl),
    BuiltinType(BuiltinType),
    CStyleCastExpr(CStyleCastExpr),
    CXX11NoReturnAttr(CXX11NoReturnAttr),
    CXXBindTemporaryExpr(CXXBindTemporaryExpr),
    CXXBoolLiteralExpr(CXXBoolLiteralExpr),
    CXXCatchStmt(CXXCatchStmt),
    CXXConstCastExpr(CXXConstCastExpr),
    CXXConstructExpr(CXXConstructExpr),
    CXXConstructorDecl(CXXConstructorDecl),
    CXXConversionDecl(CXXConversionDecl),
    CXXCtorInitializer(CXXCtorInitializer),
    CXXDeductionGuideDecl(CXXDeductionGuideDecl),
    CXXDefaultArgExpr(CXXDefaultArgExpr),
    CXXDefaultInitExpr(CXXDefaultInitExpr),
    CXXDeleteExpr(CXXDeleteExpr),
    CXXDependentScopeMemberExpr(CXXDependentScopeMemberExpr),
    CXXDestructorDecl(CXXDestructorDecl),
    CXXDynamicCastExpr(CXXDynamicCastExpr),
    CXXFoldExpr(CXXFoldExpr),
    CXXForRangeStmt(CXXForRangeStmt),
    CXXFunctionalCastExpr(CXXFunctionalCastExpr),
    CXXInheritedCtorInitExpr(CXXInheritedCtorInitExpr),
    CXXMemberCallExpr(CXXMemberCallExpr),
    CXXMethodDecl(CXXMethodDecl),
    CXXNewExpr(CXXNewExpr),
    CXXNoexceptExpr(CXXNoexceptExpr),
    CXXNullPtrLiteralExpr(CXXNullPtrLiteralExpr),
    CXXOperatorCallExpr(CXXOperatorCallExpr),
    CXXPseudoDestructorExpr(CXXPseudoDestructorExpr),
    CXXRecordDecl(CXXRecordDecl),
    CXXReinterpretCastExpr(CXXReinterpretCastExpr),
    CXXRewrittenBinaryOperator(CXXRewrittenBinaryOperator),
    CXXScalarValueInitExpr(CXXScalarValueInitExpr),
    CXXStaticCastExpr(CXXStaticCastExpr),
    CXXTemporaryObjectExpr(CXXTemporaryObjectExpr),
    CXXThisExpr(CXXThisExpr),
    CXXThrowExpr(CXXThrowExpr),
    CXXTryStmt(CXXTryStmt),
    CXXTypeidExpr(CXXTypeidExpr),
    CXXUnresolvedConstructExpr(CXXUnresolvedConstructExpr),
    CallExpr(CallExpr),
    CallbackAttr(CallbackAttr),
    CaseStmt(CaseStmt),
    CharacterLiteral(CharacterLiteral),
    ClassScopeFunctionSpecializationDecl(ClassScopeFunctionSpecializationDecl),
    ClassTemplateDecl(ClassTemplateDecl),
    ClassTemplatePartialSpecializationDecl(ClassTemplatePartialSpecializationDecl),
    ClassTemplateSpecializationDecl(ClassTemplateSpecializationDecl),
    ColdAttr(ColdAttr),
    ComplexType(ComplexType),
    CompoundAssignOperator(CompoundAssignOperator),
    CompoundLiteralExpr(CompoundLiteralExpr),
    CompoundRequirement(CompoundRequirement),
    CompoundStmt(CompoundStmt),
    ConceptDecl(ConceptDecl),
    ConceptSpecializationExpr(ConceptSpecializationExpr),
    ConditionalOperator(ConditionalOperator),
    ConstAttr(ConstAttr),
    ConstantArrayType(ConstantArrayType),
    ConstantExpr(ConstantExpr),
    ConstructorUsingShadowDecl(ConstructorUsingShadowDecl),
    ContinueStmt(ContinueStmt),
    ConvertVectorExpr(ConvertVectorExpr),
    DLLImportAttr(DLLImportAttr),
    DecayedType(DecayedType),
    DeclRefExpr(DeclRefExpr),
    DeclStmt(DeclStmt),
    DecltypeType(DecltypeType),
    DecompositionDecl(DecompositionDecl),
    DefaultStmt(DefaultStmt),
    DependentNameType(DependentNameType),
    DependentScopeDeclRefExpr(DependentScopeDeclRefExpr),
    DependentSizedArrayType(DependentSizedArrayType),
    DependentTemplateSpecializationType(DependentTemplateSpecializationType),
    DeprecatedAttr(DeprecatedAttr),
    DesignatedInitExpr(DesignatedInitExpr),
    DiagnoseIfAttr(DiagnoseIfAttr),
    DisableTailCallsAttr(DisableTailCallsAttr),
    DoStmt(DoStmt),
    ElaboratedType(ElaboratedType),
    EmptyDecl(EmptyDecl),
    EnableIfAttr(EnableIfAttr),
    EnumConstantDecl(EnumConstantDecl),
    EnumDecl(EnumDecl),
    EnumType(EnumType),
    ExcludeFromExplicitInstantiationAttr(ExcludeFromExplicitInstantiationAttr),
    ExprWithCleanups(ExprWithCleanups),
    FallThroughAttr(FallThroughAttr),
    FieldDecl(FieldDecl),
    FileScopeAsmDecl(FileScopeAsmDecl),
    FinalAttr(FinalAttr),
    FloatingLiteral(FloatingLiteral),
    ForStmt(ForStmt),
    FormatArgAttr(FormatArgAttr),
    FormatAttr(FormatAttr),
    FriendDecl(FriendDecl),
    FullComment(FullComment),
    FunctionDecl(FunctionDecl),
    FunctionProtoType(FunctionProtoType),
    FunctionTemplateDecl(FunctionTemplateDecl),
    GCCAsmStmt(GCCAsmStmt),
    GNUInlineAttr(GNUInlineAttr),
    GNUNullExpr(GNUNullExpr),
    GotoStmt(GotoStmt),
    HTMLEndTagComment(HTMLEndTagComment),
    HTMLStartTagComment(HTMLStartTagComment),
    IfStmt(IfStmt),
    ImplicitCastExpr(ImplicitCastExpr),
    ImplicitConceptSpecializationDecl(ImplicitConceptSpecializationDecl),
    ImplicitValueInitExpr(ImplicitValueInitExpr),
    IncompleteArrayType(IncompleteArrayType),
    IndirectFieldDecl(IndirectFieldDecl),
    IndirectGotoStmt(IndirectGotoStmt),
    InitListExpr(InitListExpr),
    InjectedClassNameType(InjectedClassNameType),
    InlineCommandComment(InlineCommandComment),
    IntegerLiteral(IntegerLiteral),
    InternalLinkageAttr(InternalLinkageAttr),
    LValueReferenceType(LValueReferenceType),
    LabelStmt(LabelStmt),
    LambdaExpr(LambdaExpr),
    LifetimeBoundAttr(LifetimeBoundAttr),
    LikelyAttr(LikelyAttr),
    LinkageSpecDecl(LinkageSpecDecl),
    MaterializeTemporaryExpr(MaterializeTemporaryExpr),
    MaxFieldAlignmentAttr(MaxFieldAlignmentAttr),
    MayAliasAttr(MayAliasAttr),
    MemberExpr(MemberExpr),
    MemberPointerType(MemberPointerType),
    MinVectorWidthAttr(MinVectorWidthAttr),
    ModeAttr(ModeAttr),
    NamespaceAliasDecl(NamespaceAliasDecl),
    NamespaceDecl(NamespaceDecl),
    NestedRequirement(NestedRequirement),
    NoAliasAttr(NoAliasAttr),
    NoDebugAttr(NoDebugAttr),
    NoEscapeAttr(NoEscapeAttr),
    NoInlineAttr(NoInlineAttr),
    NoSanitizeAttr(NoSanitizeAttr),
    NoThrowAttr(NoThrowAttr),
    NoUniqueAddressAttr(NoUniqueAddressAttr),
    NonNullAttr(NonNullAttr),
    NonTypeTemplateParmDecl(NonTypeTemplateParmDecl),
    NullStmt(NullStmt),
    OffsetOfExpr(OffsetOfExpr),
    OpaqueValueExpr(OpaqueValueExpr),
    OverrideAttr(OverrideAttr),
    OwnerAttr(OwnerAttr),
    PackExpansionExpr(PackExpansionExpr),
    PackExpansionType(PackExpansionType),
    PackedAttr(PackedAttr),
    ParagraphComment(ParagraphComment),
    ParamCommandComment(ParamCommandComment),
    ParenExpr(ParenExpr),
    ParenListExpr(ParenListExpr),
    ParenType(ParenType),
    ParmVarDecl(ParmVarDecl),
    PointerAttr(PointerAttr),
    PointerType(PointerType),
    PredefinedExpr(PredefinedExpr),
    PreferredNameAttr(PreferredNameAttr),
    PureAttr(PureAttr),
    QualType(QualType),
    RValueReferenceType(RValueReferenceType),
    RecordDecl(RecordDecl),
    RecordType(RecordType),
    RecoveryExpr(RecoveryExpr),
    RequiresExpr(RequiresExpr),
    RestrictAttr(RestrictAttr),
    ReturnStmt(ReturnStmt),
    ReturnsNonNullAttr(ReturnsNonNullAttr),
    ReturnsTwiceAttr(ReturnsTwiceAttr),
    ShuffleVectorExpr(ShuffleVectorExpr),
    SimpleRequirement(SimpleRequirement),
    SizeOfPackExpr(SizeOfPackExpr),
    SourceLocExpr(SourceLocExpr),
    StandaloneDebugAttr(StandaloneDebugAttr),
    StaticAssertDecl(StaticAssertDecl),
    StmtExpr(StmtExpr),
    StringLiteral(StringLiteral),
    SubstNonTypeTemplateParmExpr(SubstNonTypeTemplateParmExpr),
    SubstTemplateTypeParmPackType(SubstTemplateTypeParmPackType),
    SubstTemplateTypeParmType(SubstTemplateTypeParmType),
    SwiftAttrAttr(SwiftAttrAttr),
    SwitchStmt(SwitchStmt),
    TParamCommandComment(TParamCommandComment),
    TargetAttr(TargetAttr),
    TemplateArgument(TemplateArgument),
    TemplateSpecializationType(TemplateSpecializationType),
    TemplateTemplateParmDecl(TemplateTemplateParmDecl),
    TemplateTypeParmDecl(TemplateTypeParmDecl),
    TemplateTypeParmType(TemplateTypeParmType),
    TextComment(TextComment),
    TranslationUnitDecl(TranslationUnitDecl),
    TypeAliasDecl(TypeAliasDecl),
    TypeAliasTemplateDecl(TypeAliasTemplateDecl),
    TypeOfExprType(TypeOfExprType),
    TypeRequirement(TypeRequirement),
    TypeTraitExpr(TypeTraitExpr),
    TypeVisibilityAttr(TypeVisibilityAttr),
    TypedefDecl(TypedefDecl),
    TypedefType(TypedefType),
    UnaryExprOrTypeTraitExpr(UnaryExprOrTypeTraitExpr),
    UnaryOperator(UnaryOperator),
    UnaryTransformType(UnaryTransformType),
    UnavailableAttr(UnavailableAttr),
    UnlikelyAttr(UnlikelyAttr),
    UnresolvedLookupExpr(UnresolvedLookupExpr),
    UnresolvedMemberExpr(UnresolvedMemberExpr),
    UnresolvedUsingIfExistsDecl(UnresolvedUsingIfExistsDecl),
    UnresolvedUsingTypenameDecl(UnresolvedUsingTypenameDecl),
    UnresolvedUsingValueDecl(UnresolvedUsingValueDecl),
    UnusedAttr(UnusedAttr),
    UserDefinedLiteral(UserDefinedLiteral),
    UsingDecl(UsingDecl),
    UsingDirectiveDecl(UsingDirectiveDecl),
    UsingEnumDecl(UsingEnumDecl),
    UsingIfExistsAttr(UsingIfExistsAttr),
    UsingShadowDecl(UsingShadowDecl),
    UsingType(UsingType),
    VTablePointerAuthenticationAttr(VTablePointerAuthenticationAttr),
    VarDecl(VarDecl),
    VarTemplateDecl(VarTemplateDecl),
    VarTemplatePartialSpecializationDecl(VarTemplatePartialSpecializationDecl),
    VarTemplateSpecializationDecl(VarTemplateSpecializationDecl),
    VectorType(VectorType),
    VerbatimBlockComment(VerbatimBlockComment),
    VerbatimBlockLineComment(VerbatimBlockLineComment),
    VerbatimLineComment(VerbatimLineComment),
    VisibilityAttr(VisibilityAttr),
    WarnUnusedResultAttr(WarnUnusedResultAttr),
    WeakAttr(WeakAttr),
    WeakImportAttr(WeakImportAttr),
    WeakRefAttr(WeakRefAttr),
    WhileStmt(WhileStmt),
    #[serde(rename = "null")]
    Null,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AbiTagAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AccessSpecDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub access: AccessSpecifier,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AliasAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AlignedAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AllocAlignAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AllocSizeAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AlwaysInlineAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ArrayInitIndexExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ArrayInitLoopExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ArraySubscriptExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ArrayTypeTraitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AsmLabelAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AtomicExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub name: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AtomicType {
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AttributedStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AttributedType {
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AutoType {
    pub r#type: Type,
    pub undeduced: bool,
    #[serde(rename = "typeKeyword")]
    pub type_keyword: AutoTypeKeyword,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AvailabilityAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct AvailableOnlyInDefaultEvalMethodAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BinaryConditionalOperator {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BinaryOperator {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub opcode: BinaryOpcode,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BindingDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BlockCommandComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BlockPointerType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BreakStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BuiltinAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BuiltinBitCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BuiltinTemplateDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct BuiltinType {
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CStyleCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXX11NoReturnAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXBindTemporaryExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub temp: Id,
    pub dtor: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXBoolLiteralExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub value: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXCatchStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXConstCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXConstructExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "ctorType")]
    pub ctor_type: Type,
    #[serde(default)]
    pub elidable: bool,
    #[serde(default)]
    pub list: bool,
    #[serde(default)]
    pub initializer_list: bool,
    #[serde(default)]
    pub zeroing: bool,
    #[serde(rename = "hadMultipleCandidates", default)]
    pub had_multiple_candidates: bool,
    #[serde(rename = "constructionKind")]
    pub construction_kind: ConstructionKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXConstructorDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(rename = "storageClass", default)]
    pub storage_class: StorageClass,
    #[serde(default)]
    pub inline: bool,
    #[serde(rename = "explicitlyDeleted", default)]
    pub explicitly_deleted: bool,
    #[serde(default)]
    pub constexpr: bool,
    #[serde(default)]
    pub variadic: bool,
    #[serde(default)]
    pub immediate: bool,
    #[serde(rename = "explicitlyDefaulted", default)]
    pub explicitly_defaulted: ExplicitlyDefaulted,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXConversionDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub constexpr: bool,
}

#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub enum CXXCtorInitializer {
    #[serde(rename = "anyInit")]
    AnyMemberInitializer(Decl),
    #[serde(rename = "baseInit")]
    BaseInitializer(Type),
    #[serde(rename = "delegatingInit")]
    DelegatingInitializer(Type),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXDeductionGuideDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(default)]
    pub variadic: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXDefaultArgExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXDefaultInitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXDeleteExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "isGlobal", default)]
    pub is_global: bool,
    #[serde(rename = "isArray", default)]
    pub is_array: bool,
    #[serde(rename = "isArrayAsWritten", default)]
    pub is_array_as_written: bool,
    #[serde(rename = "operatorDeleteDecl")]
    pub operator_delete_decl: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXDependentScopeMemberExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "isArrow")]
    pub is_arrow: bool,
    pub member: Box<str>,
    #[serde(rename = "hasTemplateKeyword", default)]
    pub has_template_keyword: bool,
    #[serde(rename = "hasExplicitTemplateArgs", default)]
    pub has_explicit_template_args: bool,
    #[serde(rename = "explicitTemplateArgs", default)]
    pub explicit_template_args: Vec<TemplateArgument>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXDestructorDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub r#virtual: bool,
    #[serde(rename = "explicitlyDeleted", default)]
    pub explicitly_deleted: bool,
    #[serde(default)]
    pub constexpr: bool,
    #[serde(rename = "explicitlyDefaulted", default)]
    pub explicitly_defaulted: ExplicitlyDefaulted,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXDynamicCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXFoldExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXForRangeStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXFunctionalCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
    #[serde(rename = "conversionFunc")]
    pub conversion_func: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXInheritedCtorInitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXMemberCallExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXMethodDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(rename = "storageClass", default)]
    pub storage_class: StorageClass,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub r#virtual: bool,
    #[serde(default)]
    pub pure: bool,
    #[serde(rename = "explicitlyDeleted", default)]
    pub explicitly_deleted: bool,
    #[serde(default)]
    pub constexpr: bool,
    #[serde(default)]
    pub variadic: bool,
    #[serde(default)]
    pub immediate: bool,
    #[serde(rename = "explicitlyDefaulted", default)]
    pub explicitly_defaulted: ExplicitlyDefaulted,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXNewExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "isGlobal", default)]
    pub is_global: bool,
    #[serde(rename = "isArray", default)]
    pub is_array: bool,
    #[serde(rename = "isPlacement", default)]
    pub is_placement: bool,
    #[serde(rename = "initStyle", default)]
    pub init_style: InitStyle,
    #[serde(rename = "operatorNewDecl")]
    pub operator_new_decl: Option<Decl>,
    #[serde(rename = "operatorDeleteDecl")]
    pub operator_delete_decl: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXNoexceptExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXNullPtrLiteralExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXOperatorCallExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(default)]
    pub adl: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXPseudoDestructorExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXRecordDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Option<Box<str>>,
    #[serde(rename = "tagUsed")]
    pub tag_used: TagTypeKind,
    #[serde(rename = "completeDefinition", default)]
    pub complete_definition: bool,
    #[serde(rename = "definitionData")]
    pub definition_data: Option<CXXRecordDefinitionData>,
    #[serde(default)]
    pub bases: Vec<CXXBaseSpecifier>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXReinterpretCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXScalarValueInitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXStaticCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
    #[serde(default)]
    pub path: Vec<CastPath>,
    #[serde(rename = "conversionFunc")]
    pub conversion_func: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXTemporaryObjectExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "ctorType")]
    pub ctor_type: Type,
    #[serde(default)]
    pub list: bool,
    #[serde(default)]
    pub zeroing: bool,
    #[serde(rename = "hadMultipleCandidates", default)]
    pub had_multiple_candidates: bool,
    #[serde(rename = "constructionKind")]
    pub construction_kind: ConstructionKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXThisExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXThrowExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXTryStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXTypeidExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "typeArg")]
    pub type_arg: Option<Type>,
    #[serde(rename = "adjustedTypeArg")]
    pub adjusted_type_arg: Option<Type>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXUnresolvedConstructExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "typeAsWritten")]
    pub type_as_written: Option<Type>,
    #[serde(default)]
    pub list: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CallExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(default)]
    pub adl: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CallbackAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CaseStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CharacterLiteral {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub value: u32,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ClassScopeFunctionSpecializationDecl {}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ClassTemplateDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ClassTemplatePartialSpecializationDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "tagUsed")]
    pub tag_used: TagTypeKind,
    #[serde(rename = "completeDefinition", default)]
    pub complete_definition: bool,
    #[serde(rename = "definitionData")]
    pub definition_data: Option<CXXRecordDefinitionData>,
    #[serde(default)]
    pub bases: Vec<CXXBaseSpecifier>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ClassTemplateSpecializationDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "tagUsed")]
    pub tag_used: Option<TagTypeKind>,
    #[serde(rename = "completeDefinition", default)]
    pub complete_definition: bool,
    #[serde(rename = "definitionData")]
    pub definition_data: Option<CXXRecordDefinitionData>,
    #[serde(default)]
    pub bases: Vec<CXXBaseSpecifier>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ColdAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ComplexType {
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CompoundAssignOperator {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub opcode: BinaryOpcode,
    #[serde(rename = "computeLHSType")]
    pub compute_lhs_type: Type,
    #[serde(rename = "computeResultType")]
    pub compute_result_type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CompoundLiteralExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CompoundRequirement {
    #[serde(default)]
    pub noexcept: bool,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    pub satisfied: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CompoundStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConceptDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConceptSpecializationExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConditionalOperator {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConstAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConstantArrayType {
    pub r#type: Type,
    pub size: usize,
    #[serde(rename = "sizeModifier", default)]
    pub size_modifier: ArrayType,
    #[serde(rename = "indexTypeQualifiers")]
    pub index_type_qualifiers: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConstantExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub value: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConstructorUsingShadowDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    pub target: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ContinueStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ConvertVectorExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DLLImportAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DecayedType {
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DeclRefExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "referencedDecl")]
    pub referenced_decl: Decl,
    #[serde(rename = "foundReferencedDecl")]
    pub found_referenced_decl: Option<Decl>,
    #[serde(rename = "nonOdrUseReason", default)]
    pub non_odr_use_reason: NonOdrUseReason,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DeclStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DecltypeType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DecompositionDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(default)]
    pub init: InitStyle,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DefaultStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DependentNameType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DependentScopeDeclRefExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DependentSizedArrayType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DependentTemplateSpecializationType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DeprecatedAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    pub message: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DesignatedInitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DiagnoseIfAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DisableTailCallsAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DoStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ElaboratedType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
    pub qualifier: Option<Box<str>>,
    #[serde(rename = "ownedTagDecl")]
    pub owned_tag_decl: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct EmptyDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct EnableIfAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct EnumConstantDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    pub name: Box<str>,
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct EnumDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Option<Box<str>>,
    #[serde(rename = "fixedUnderlyingType")]
    pub fixed_underlying_type: Option<Type>,
    #[serde(rename = "scopedEnumTag", default)]
    pub scoped_enum_tag: ScopedEnumTag,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct EnumType {
    pub r#type: Type,
    pub decl: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ExcludeFromExplicitInstantiationAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ExprWithCleanups {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "cleanupsHaveSideEffects", default)]
    pub cleanups_have_side_effects: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FallThroughAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FieldDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    pub name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(default)]
    pub mutable: bool,
    #[serde(rename = "isBitfield", default)]
    pub is_bitfield: bool,
    #[serde(rename = "hasInClassInitializer", default)]
    pub has_in_class_initializer: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FileScopeAsmDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FinalAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FloatingLiteral {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub value: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ForStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FormatArgAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FormatAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FriendDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub r#type: Option<Type>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FullComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FunctionDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(rename = "storageClass", default)]
    pub storage_class: StorageClass,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub r#virtual: bool,
    #[serde(default)]
    pub pure: bool,
    #[serde(rename = "explicitlyDeleted", default)]
    pub explicitly_deleted: bool,
    #[serde(default)]
    pub constexpr: bool,
    #[serde(default)]
    pub variadic: bool,
    #[serde(default)]
    pub immediate: bool,
    #[serde(rename = "explicitlyDefaulted", default)]
    pub explicitly_defaulted: ExplicitlyDefaulted,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FunctionProtoType {
    #[serde(rename = "trailingReturn", default)]
    pub trailing_return: bool,
    #[serde(default)]
    pub r#const: bool,
    #[serde(default)]
    pub volatile: bool,
    #[serde(default)]
    pub restrict: bool,
    #[serde(default)]
    pub variadic: bool,
    #[serde(rename = "refQualifier", default)]
    pub ref_qualifier: RefQualifier,
    #[serde(rename = "exceptionSpec", default)]
    pub exception_spec: ExceptionSpec,
    #[serde(rename = "exceptionTypes", default)]
    pub exception_types: Vec<Type>,
    #[serde(rename = "conditionEvaluatesTo")]
    pub condition_evaluates_to: Option<bool>,
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    pub cc: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct FunctionTemplateDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct GCCAsmStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct GNUInlineAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct GNUNullExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct GotoStmt {
    pub range: SourceRange,
    #[serde(rename = "targetLabelDeclId")]
    pub target_label_decl_id: Id,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct HTMLStartTagComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
    #[serde(default, rename = "selfClosing")]
    pub self_closing: bool,
    #[serde(default)]
    pub malformed: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct HTMLEndTagComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IfStmt {
    pub range: SourceRange,
    #[serde(rename = "hasInit", default)]
    pub has_init: bool,
    #[serde(rename = "hasVar", default)]
    pub has_var: bool,
    #[serde(rename = "hasElse", default)]
    pub has_else: bool,
    #[serde(rename = "isConstexpr", default)]
    pub is_constexpr: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ImplicitCastExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "castKind")]
    pub cast_kind: CastKind,
    #[serde(default)]
    pub path: Vec<CastPath>,
    #[serde(rename = "conversionFunc")]
    pub conversion_func: Option<Decl>,
    #[serde(rename = "isPartOfExplicitCast", default)]
    pub is_part_of_explicit_cast: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ImplicitConceptSpecializationDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ImplicitValueInitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IncompleteArrayType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IndirectFieldDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IndirectGotoStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct InitListExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(default)]
    pub array_filler: Vec<Node>,
    pub field: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct InjectedClassNameType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    pub decl: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct InlineCommandComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
    #[serde(rename = "renderKind")]
    pub render_kind: RenderKind,
    #[serde(default)]
    pub args: Vec<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IntegerLiteral {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub value: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct InternalLinkageAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LValueReferenceType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LabelStmt {
    pub range: SourceRange,
    pub name: Box<str>,
    #[serde(rename = "declId")]
    pub decl_id: Id,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LambdaExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LifetimeBoundAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LikelyAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LinkageSpecDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    pub language: Language,
    #[serde(rename = "hasBraces", default)]
    pub has_braces: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MaterializeTemporaryExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "extendingDecl")]
    pub extending_decl: Option<Decl>,
    #[serde(rename = "storageDuration")]
    pub storage_duration: StorageDuration,
    #[serde(rename = "boundToLValueRef", default)]
    pub bound_to_lvalue_ref: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MaxFieldAlignmentAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MayAliasAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MemberExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub name: Box<str>,
    #[serde(rename = "isArrow")]
    pub is_arrow: bool,
    #[serde(rename = "referencedMemberDecl")]
    pub referenced_member_decl: Id,
    #[serde(rename = "nonOdrUseReason", default)]
    pub non_odr_use_reason: NonOdrUseReason,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MemberPointerType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "isData", default)]
    pub is_data: bool,
    #[serde(rename = "isFunction", default)]
    pub is_function: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MinVectorWidthAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ModeAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NamespaceAliasDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
    #[serde(rename = "aliasedNamespace")]
    pub aliased_namespace: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NamespaceDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Option<Box<str>>,
    #[serde(rename = "isInline", default)]
    pub is_inline: bool,
    #[serde(rename = "isNested", default)]
    pub is_nested: bool,
    #[serde(rename = "originalNamespace")]
    pub original_namespace: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NoAliasAttr {}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NestedRequirement {
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NoDebugAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NoEscapeAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NoInlineAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NoSanitizeAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NoThrowAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NoUniqueAddressAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NonNullAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NonTypeTemplateParmDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    pub name: Option<Box<str>>,
    pub r#type: Type,
    pub depth: usize,
    pub index: usize,
    #[serde(rename = "isParameterPack", default)]
    pub is_parameter_pack: bool,
    #[serde(rename = "defaultArg")]
    pub default_arg: Option<TemplateArgument>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct NullStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct OffsetOfExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct OpaqueValueExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct OverrideAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct OwnerAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PackExpansionExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PackExpansionType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "numExpansions")]
    pub num_expansions: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PackedAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ParagraphComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ParamCommandComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub direction: ParamDirection,
    #[serde(default)]
    pub explicit: bool,
    pub param: Box<str>,
    #[serde(rename = "paramIdx")]
    pub param_idx: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ParenExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ParenListExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ParenType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ParmVarDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    pub name: Option<Box<str>>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(default)]
    pub init: InitStyle,
    #[serde(rename = "isParameterPack", default)]
    pub is_parameter_pack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PointerAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PointerType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PredefinedExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PreferredNameAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PureAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct QualType {
    pub r#type: Type,
    pub qualifiers: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct RValueReferenceType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
    #[serde(rename = "spelledAsLValue", default)]
    pub spelled_as_lvalue: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct RecordDecl {
    pub range: SourceRange,
    pub name: Box<str>,
    #[serde(rename = "tagUsed")]
    pub tag_used: Option<TagTypeKind>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    #[serde(rename = "completeDefinition", default)]
    pub complete_definition: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct RecordType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    pub decl: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct RecoveryExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct RequiresExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub satisfied: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct RestrictAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ReturnStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ReturnsNonNullAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ReturnsTwiceAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ShuffleVectorExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SimpleRequirement {
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    pub satisfied: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SizeOfPackExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SourceLocExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct StandaloneDebugAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct StaticAssertDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct StmtExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct StringLiteral {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub value: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SubstNonTypeTemplateParmExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SubstTemplateTypeParmPackType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
    pub index: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SubstTemplateTypeParmType {
    pub r#type: Type,
    pub index: Option<usize>,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
    pub pack_index: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SwiftAttrAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SwitchStmt {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TParamCommandComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub param: Box<str>,
    #[serde(default)]
    pub positions: Vec<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TargetAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TemplateArgument {
    #[serde(default)]
    pub kind: MustBe!("TemplateArgument"),
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "inherited from")]
    pub inherited_from: Option<Decl>,
    #[serde(rename = "isNull", default)]
    pub is_null: bool,
    pub r#type: Option<Type>,
    pub decl: Option<Decl>,
    #[serde(rename = "isNullptr", default)]
    pub is_nullptr: bool,
    pub value: Option<i64>,
    #[serde(rename = "isExpr", default)]
    pub is_expr: bool,
    #[serde(rename = "isPack", default)]
    pub is_pack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TemplateSpecializationType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
    #[serde(rename = "isAlias", default)]
    pub is_alias: bool,
    #[serde(rename = "templateName")]
    pub template_name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TemplateTemplateParmDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    pub name: Option<Box<str>>,
    pub depth: usize,
    pub index: usize,
    #[serde(rename = "defaultArg")]
    pub default_arg: Option<TemplateArgument>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TemplateTypeParmDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    pub name: Option<Box<str>>,
    #[serde(rename = "tagUsed")]
    pub tag_used: TemplateTypeParmTag,
    pub depth: usize,
    pub index: usize,
    #[serde(rename = "isParameterPack", default)]
    pub is_parameter_pack: bool,
    #[serde(rename = "defaultArg")]
    pub default_arg: Option<TemplateArgument>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TemplateTypeParmType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
    pub depth: usize,
    pub index: usize,
    #[serde(rename = "isPack", default)]
    pub is_pack: bool,
    pub decl: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TextComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub text: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TranslationUnitDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypeAliasDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypeAliasTemplateDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypeOfExprType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypeRequirement {
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypeTraitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypeVisibilityAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypedefDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
    pub r#type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TypedefType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    pub decl: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnaryExprOrTypeTraitExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    pub name: Box<str>,
    #[serde(rename = "argType")]
    pub arg_type: Option<Type>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnaryOperator {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "isPostfix")]
    pub is_postfix: bool,
    pub opcode: UnaryOpcode,
    #[serde(rename = "canOverflow", default = "default_true")]
    pub can_overflow: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnaryTransformType {
    pub r#type: Type,
    #[serde(rename = "isDependent", default)]
    pub is_dependent: bool,
    #[serde(rename = "isInstantiationDependent", default)]
    pub is_instantiation_dependent: bool,
    #[serde(rename = "containsUnexpandedPack", default)]
    pub contains_unexpanded_pack: bool,
    #[serde(rename = "transformKind")]
    pub transform_kind: UnaryTransformTypeKind,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnavailableAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnlikelyAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnresolvedLookupExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
    #[serde(rename = "usesADL")]
    pub uses_adl: bool,
    pub name: Box<str>,
    pub lookups: Vec<Node>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnresolvedMemberExpr {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnresolvedUsingIfExistsDecl {}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnresolvedUsingTypenameDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnresolvedUsingValueDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    pub name: Box<str>,
    pub r#type: Option<Type>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UnusedAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UserDefinedLiteral {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UsingDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UsingDirectiveDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "nominatedNamespace")]
    pub nominated_namespace: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UsingEnumDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub target: Decl,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UsingIfExistsAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UsingShadowDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub target: Option<Decl>,
    pub name: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct UsingType {
    pub r#type: Type,
    pub decl: Option<Decl>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VTablePointerAuthenticationAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VarDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "isImplicit", default)]
    pub is_implicit: bool,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Option<Box<str>>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(rename = "storageClass", default)]
    pub storage_class: StorageClass,
    #[serde(default)]
    pub tls: TLSKind,
    #[serde(default)]
    pub nrvo: bool,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub constexpr: bool,
    #[serde(rename = "modulePrivate", default)]
    pub module_private: bool,
    #[serde(default)]
    pub init: InitStyle,
    #[serde(rename = "isParameterPack", default)]
    pub is_parameter_pack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VarTemplateDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "previousDecl")]
    pub previous_decl: Option<Id>,
    pub name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VarTemplatePartialSpecializationDecl {
    pub loc: SourceLocation,
    pub range: SourceRange,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(rename = "storageClass", default)]
    pub storage_class: StorageClass,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub constexpr: bool,
    pub init: InitStyle,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VarTemplateSpecializationDecl {
    #[serde(default)]
    pub loc: SourceLocation,
    #[serde(default)]
    pub range: SourceRange,
    #[serde(rename = "isUsed", default)]
    pub is_used: bool,
    #[serde(rename = "isReferenced", default)]
    pub is_referenced: bool,
    #[serde(rename = "parentDeclContextId")]
    pub parent_decl_context_id: Option<Id>,
    pub name: Box<str>,
    #[serde(rename = "mangledName")]
    pub mangled_name: Option<Box<str>>,
    pub r#type: Type,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub constexpr: bool,
    #[serde(default)]
    pub init: InitStyle,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VectorType {
    pub r#type: Type,
    #[serde(rename = "numElements")]
    pub num_elements: usize,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VerbatimBlockComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub name: Box<str>,
    #[serde(rename = "closeName")]
    pub close_name: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VerbatimBlockLineComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub text: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VerbatimLineComment {
    pub loc: SourceLocation,
    pub range: SourceRange,
    pub text: Box<str>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct VisibilityAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default)]
    pub implicit: bool,
    #[serde(default)]
    pub visibility: Visibility,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WarnUnusedResultAttr {
    pub range: SourceRange,
    #[serde(default)]
    pub inherited: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WeakAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WeakImportAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WeakRefAttr {
    pub range: SourceRange,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WhileStmt {
    pub range: SourceRange,
    #[serde(rename = "hasVar", default)]
    pub has_var: bool,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum AccessSpecifier {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "none")]
    None,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ArrayType {
    #[serde(rename = "*")]
    Star,
    #[serde(rename = "static")]
    Static,
    #[serde(skip_deserializing)]
    Normal,
}

impl Default for ArrayType {
    fn default() -> Self {
        ArrayType::Normal
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum AutoTypeKeyword {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "decltype(auto)")]
    DecltypeAuto,
    #[serde(rename = "__auto_type")]
    GNUAutoType,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum BinaryOpcode {
    #[serde(rename = ".*")]
    PtrMemD,
    #[serde(rename = "->*")]
    PtrMemI,
    #[serde(rename = "*")]
    Mul,
    #[serde(rename = "/")]
    Div,
    #[serde(rename = "%")]
    Rem,
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Sub,
    #[serde(rename = "<<")]
    Shl,
    #[serde(rename = ">>")]
    Shr,
    #[serde(rename = "<=>")]
    Cmp,
    #[serde(rename = "<")]
    LT,
    #[serde(rename = ">")]
    GT,
    #[serde(rename = "<=")]
    LE,
    #[serde(rename = ">=")]
    GE,
    #[serde(rename = "==")]
    EQ,
    #[serde(rename = "!=")]
    NE,
    #[serde(rename = "&")]
    And,
    #[serde(rename = "^")]
    Xor,
    #[serde(rename = "|")]
    Or,
    #[serde(rename = "&&")]
    LAnd,
    #[serde(rename = "||")]
    LOr,
    #[serde(rename = "=")]
    Assign,
    #[serde(rename = "*=")]
    MulAssign,
    #[serde(rename = "/=")]
    DivAssign,
    #[serde(rename = "%=")]
    RemAssign,
    #[serde(rename = "+=")]
    AddAssign,
    #[serde(rename = "-=")]
    SubAssign,
    #[serde(rename = "<<=")]
    ShlAssign,
    #[serde(rename = ">>=")]
    ShrAssign,
    #[serde(rename = "&=")]
    AndAssign,
    #[serde(rename = "^=")]
    XorAssign,
    #[serde(rename = "|=")]
    OrAssign,
    #[serde(rename = ",")]
    Comma,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum CastKind {
    /// A conversion which cannot yet be analyzed because either the expression
    /// or target type is dependent. These are created only for explicit casts;
    /// dependent ASTs aren't required to even approximately type-check.
    ///
    /// - `(T*) malloc(sizeof(T))`
    /// - `reinterpret_cast<intptr_t>(A<T>::alloc());`
    Dependent,

    /// A conversion which causes a bit pattern of one type to be reinterpreted
    /// as a bit pattern of another type. Generally the operands must have
    /// equivalent size and unrelated types.
    ///
    /// The pointer conversion char* -> int* is a bitcast. A conversion from any
    /// pointer type to a C pointer type is a bitcast unless it's actually
    /// BaseToDerived or DerivedToBase. A conversion to a block pointer or ObjC
    /// pointer type is a bitcast only if the operand has the same type kind;
    /// otherwise, it's one of the specialized casts below.
    ///
    /// Vector coercions are bitcasts.
    BitCast,

    /// A conversion which reinterprets the address of an l-value as an l-value
    /// of a different kind. Used for reinterpret_casts of l-value expressions
    /// to reference types.
    ///
    /// - `bool b; reinterpret_cast<char&>(b) = 'a';`
    LValueBitCast,

    /// A conversion that causes us to reinterpret the object representation of
    /// an lvalue as an rvalue. Created by __builtin_bit_cast.
    LValueToRValueBitCast,

    /// A conversion which causes the extraction of an r-value from the operand
    /// gl-value. The result of an r-value conversion is always unqualified.
    LValueToRValue,

    /// A conversion which does not affect the type other than (possibly) adding
    /// qualifiers or removing noexcept.
    ///
    /// - `int` -> `int`
    /// - `char**` -> `const char * const *`
    /// - `void () noexcept` -> `void ()`
    NoOp,

    /// A conversion from a C++ class pointer/reference to a derived class
    /// pointer/reference.
    ///
    /// - `B *b = static_cast<B*>(a);`
    BaseToDerived,

    /// A conversion from a C++ class pointer to a base class pointer.
    ///
    /// - `A *a = new B();`
    DerivedToBase,

    /// A conversion from a C++ class pointer/reference to a base class that can
    /// assume that the derived pointer is not null.
    ///
    /// - `const A &a = B();`
    /// - `b->method_from_a();`
    UncheckedDerivedToBase,

    /// A C++ dynamic_cast.
    Dynamic,

    /// The GCC cast-to-union extension.
    ///
    /// - `int` -> `union { int x; float y; }`
    /// - `float` -> `union { int x; float y; }`
    ToUnion,

    /// Array to pointer decay.
    ///
    /// - `int[10]` -> `int*`
    /// - `char[5][6]` -> `char(*)[6]`
    ArrayToPointerDecay,

    /// Function to pointer decay.
    ///
    /// - `void(int)` -> `void(*)(int)`
    FunctionToPointerDecay,

    /// Null pointer constant to pointer, ObjC pointer, or block pointer.
    ///
    /// - `(void*) 0`
    /// - `void (^block)() = 0;`
    NullToPointer,

    /// Null pointer constant to member pointer.
    ///
    /// - `int A::*mptr = 0;`
    /// - `int (A::*fptr)(int) = nullptr;`
    NullToMemberPointer,

    /// Member pointer in base class to member pointer in derived class.
    ///
    /// - `int B::*mptr = &A::member;`
    BaseToDerivedMemberPointer,

    /// Member pointer in derived class to member pointer in base class.
    ///
    /// - `int A::*mptr = static_cast<int A::*>(&B::member);`
    DerivedToBaseMemberPointer,

    /// Member pointer to boolean. A check against the null member pointer.
    MemberPointerToBoolean,

    /// Reinterpret a member pointer as a different kind of member pointer. C++
    /// forbids this from crossing between function and object types, but
    /// otherwise does not restrict it. However, the only operation that is
    /// permitted on a "punned" member pointer is casting it back to the
    /// original type, which is required to be a lossless operation (although
    /// many ABIs do not guarantee this on all possible intermediate types).
    ReinterpretMemberPointer,

    /// Conversion using a user defined type conversion function.
    ///
    /// - `struct A { operator int(); }; int i = int(A());`
    UserDefinedConversion,

    /// Conversion by constructor.
    ///
    /// - `struct A { A(int); }; A a = A(10);`
    ConstructorConversion,

    /// Integral to pointer. A special kind of reinterpreting conversion.
    /// Applies to normal, ObjC, and block pointers.
    ///
    /// - `(char*) 0x1001aab0`
    /// - `reinterpret_cast<int*>(0)`
    IntegralToPointer,

    /// Pointer to integral. A special kind of reinterpreting conversion.
    /// Applies to normal, ObjC, and block pointers.
    ///
    /// - `(intptr_t) "help!"`
    PointerToIntegral,

    /// Pointer to boolean conversion. A check against null. Applies to normal,
    /// ObjC, and block pointers.
    PointerToBoolean,

    /// Cast to void, discarding the computed value.
    ///
    /// - `(void) malloc(2048)`
    ToVoid,

    /// A cast between matrix types of the same dimensions.
    MatrixCast,

    /// A conversion from an arithmetic type to a vector of that element type.
    /// Fills all elements ("splats") with the source value.
    ///
    /// - `__attribute__((ext_vector_type(4))) int v = 5;`
    VectorSplat,

    /// A cast between integral types (other than to boolean). Variously a
    /// bitcast, a truncation, a sign-extension, or a zero-extension.
    ///
    /// - `long l = 5;`
    /// - `(unsigned) i`
    IntegralCast,

    /// Integral to boolean. A check against zero.
    ///
    /// - `(bool) i`
    IntegralToBoolean,

    /// Integral to floating point.
    ///
    /// - `float f = i;`
    IntegralToFloating,

    /// Floating to fixed point.
    ///
    /// - `_Accum a = f;`
    FloatingToFixedPoint,

    /// Fixed point to floating.
    ///
    /// - `(float) 2.5k`
    FixedPointToFloating,

    /// Fixed point to fixed point.
    ///
    /// - `(_Accum) 0.5r`
    FixedPointCast,

    /// Fixed point to integral.
    ///
    /// - `(int) 2.0k`
    FixedPointToIntegral,

    /// Integral to a fixed point.
    ///
    /// - `(_Accum) 2`
    IntegralToFixedPoint,

    /// Fixed point to boolean.
    ///
    /// - `(bool) 0.5r`
    FixedPointToBoolean,

    /// Floating point to integral. Rounds towards zero, discarding any
    /// fractional component.
    ///
    /// - `(int) f`
    FloatingToIntegral,

    /// Floating point to boolean.
    ///
    /// - `(bool) f`
    FloatingToBoolean,

    /// Convert a boolean to -1 or 0 for true and false, respectively.
    BooleanToSignedIntegral,

    /// Casting between floating types of different size.
    ///
    /// - `(double) f`
    /// - `(float) ld`
    FloatingCast,

    /// Casting a C pointer kind to an Objective-C pointer.
    CPointerToObjCPointerCast,

    /// Casting a block pointer to an ObjC pointer.
    BlockPointerToObjCPointerCast,

    /// Casting any non-block pointer to a block pointer. Block-to-block casts
    /// are bitcasts.
    AnyPointerToBlockPointerCast,

    /// Converting between two Objective-C object types, which can occur when
    /// performing reference binding to an Objective-C object.
    ObjCObjectLValueCast,

    /// A conversion of a floating point real to a floating point complex of the
    /// original type. Injects the value as the real component with a zero
    /// imaginary component.
    ///
    /// - `float` -> `_Complex float`
    FloatingRealToComplex,

    /// Converts a floating point complex to floating point real of the source's
    /// element type. Just discards the imaginary component.
    ///
    /// - `_Complex long double` -> `long double`
    FloatingComplexToReal,

    /// Converts a floating point complex to bool by comparing against 0+0i.
    FloatingComplexToBoolean,

    /// Converts between different floating point complex types.
    ///
    /// - `_Complex float` -> `_Complex double`
    FloatingComplexCast,

    /// Converts from a floating complex to an integral complex.
    ///
    /// - `_Complex float` -> `_Complex int`
    FloatingComplexToIntegralComplex,

    /// Converts from an integral real to an integral complex whose element type
    /// matches the source. Injects the value as the real component with a zero
    /// imaginary component.
    ///
    /// - `long` -> `_Complex long`
    IntegralRealToComplex,

    /// Converts an integral complex to an integral real of the source's element
    /// type by discarding the imaginary component.
    ///
    /// - `_Complex short` -> `short`
    IntegralComplexToReal,

    /// Converts an integral complex to bool by comparing against 0+0i.
    IntegralComplexToBoolean,

    /// Converts between different integral complex types.
    ///
    /// - `_Complex char` -> `_Complex long long`
    /// - `_Complex unsigned int` -> `_Complex signed int`
    IntegralComplexCast,

    /// Converts from an integral complex to a floating complex.
    ///
    /// - `_Complex unsigned` -> `_Complex float`
    IntegralComplexToFloatingComplex,

    /// Converts from _Atomic(T) to T.
    AtomicToNonAtomic,

    /// Converts from T to _Atomic(T).
    NonAtomicToAtomic,

    /// Causes a block literal to by copied to the heap and then autoreleased.
    ///
    /// This particular cast kind is used for the conversion from a C++11 lambda
    /// expression to a block pointer.
    CopyAndAutoreleaseBlockObject,

    /// Convert a builtin function to a function pointer; only allowed in the
    /// callee of a call expression.
    BuiltinFnToFnPtr,

    /// Convert a zero value for OpenCL opaque types initialization (event_t,
    /// queue_t, etc.)
    ZeroToOCLOpaqueType,

    /// Convert a pointer to a different address space.
    AddressSpaceConversion,

    /// Convert an integer initializer to an OpenCL sampler.
    IntToOCLSampler,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CastPath {
    pub name: Box<str>,
    #[serde(rename = "isVirtual", default)]
    pub is_virtual: bool,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ConstructionKind {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "delegating")]
    Delegating,
    #[serde(rename = "non-virtual base")]
    NonVirtualBase,
    #[serde(rename = "virtual base")]
    VirtualBase,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CopyAssignmentDefinitionData {
    #[serde(default)]
    pub simple: bool,
    #[serde(default)]
    pub trivial: bool,
    #[serde(rename = "nonTrivial", default)]
    pub non_trivial: bool,
    #[serde(rename = "hasConstParam", default)]
    pub has_const_param: bool,
    #[serde(rename = "implicitHasConstParam", default)]
    pub implicit_has_const_param: bool,
    #[serde(rename = "userDeclared", default)]
    pub user_declared: bool,
    #[serde(rename = "needsImplicit", default)]
    pub needs_implicit: bool,
    #[serde(rename = "needsOverloadResolution", default)]
    pub needs_overload_resolution: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CopyConstructorDefinitionData {
    #[serde(default)]
    pub simple: bool,
    #[serde(default)]
    pub trivial: bool,
    #[serde(rename = "nonTrivial", default)]
    pub non_trivial: bool,
    #[serde(rename = "userDeclared", default)]
    pub user_declared: bool,
    #[serde(rename = "hasConstParam", default)]
    pub has_const_param: bool,
    #[serde(rename = "implicitHasConstParam", default)]
    pub implicit_has_const_param: bool,
    #[serde(rename = "needsImplicit", default)]
    pub needs_implicit: bool,
    #[serde(rename = "needsOverloadResolution", default)]
    pub needs_overload_resolution: bool,
    #[serde(rename = "defaultedIsDeleted", default)]
    pub defaulted_is_deleted: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXBaseSpecifier {
    pub r#type: Type,
    pub access: AccessSpecifier,
    #[serde(rename = "writtenAccess")]
    pub written_access: AccessSpecifier,
    #[serde(rename = "isVirtual", default)]
    pub is_virtual: bool,
    #[serde(rename = "isPackExpansion", default)]
    pub is_pack_expansion: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXRecordDefinitionData {
    #[serde(rename = "isGenericLambda", default)]
    pub is_generic_lambda: bool,
    #[serde(rename = "isLambda", default)]
    pub is_lambda: bool,
    #[serde(rename = "isEmpty", default)]
    pub is_empty: bool,
    #[serde(rename = "isAggregate", default)]
    pub is_aggregate: bool,
    #[serde(rename = "isStandardLayout", default)]
    pub is_standard_layout: bool,
    #[serde(rename = "isTriviallyCopyable", default)]
    pub is_trivially_copyable: bool,
    #[serde(rename = "isPOD", default)]
    pub is_pod: bool,
    #[serde(rename = "isTrivial", default)]
    pub is_trivial: bool,
    #[serde(rename = "isPolymorphic", default)]
    pub is_polymorphic: bool,
    #[serde(rename = "isAbstract", default)]
    pub is_abstract: bool,
    #[serde(rename = "isLiteral", default)]
    pub is_literal: bool,
    #[serde(rename = "canPassInRegisters", default)]
    pub can_pass_in_registers: bool,
    #[serde(rename = "hasUserDeclaredConstructor", default)]
    pub has_user_declared_constructor: bool,
    #[serde(rename = "hasConstexprNonCopyMoveConstructor", default)]
    pub has_constexpr_non_copy_move_constructor: bool,
    #[serde(rename = "hasMutableFields", default)]
    pub has_mutable_fields: bool,
    #[serde(rename = "hasVariantMembers", default)]
    pub has_variant_members: bool,
    #[serde(rename = "canConstDefaultInit", default)]
    pub can_const_default_init: bool,
    #[serde(rename = "defaultCtor")]
    pub default_ctor: DefaultConstructorDefinitionData,
    #[serde(rename = "copyCtor")]
    pub copy_ctor: CopyConstructorDefinitionData,
    #[serde(rename = "moveCtor")]
    pub move_ctor: MoveConstructorDefinitionData,
    #[serde(rename = "copyAssign")]
    pub copy_assign: CopyAssignmentDefinitionData,
    #[serde(rename = "moveAssign")]
    pub move_assign: MoveAssignmentDefinitionData,
    pub dtor: DestructorDefinitionData,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CXXRewrittenBinaryOperator {
    pub range: SourceRange,
    pub r#type: Type,
    #[serde(rename = "valueCategory")]
    pub value_category: ValueCategory,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Decl {
    pub id: Id,
    #[serde(default)]
    pub kind: Kind,
    pub name: Option<Box<str>>,
    pub r#type: Option<Type>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DefaultConstructorDefinitionData {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub trivial: bool,
    #[serde(rename = "nonTrivial", default)]
    pub non_trivial: bool,
    #[serde(rename = "userProvided", default)]
    pub user_provided: bool,
    #[serde(rename = "isConstexpr", default)]
    pub is_constexpr: bool,
    #[serde(rename = "needsImplicit", default)]
    pub needs_implicit: bool,
    #[serde(rename = "defaultedIsConstexpr", default)]
    pub defaulted_is_constexpr: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct DestructorDefinitionData {
    #[serde(default)]
    pub simple: bool,
    #[serde(default)]
    pub irrelevant: bool,
    #[serde(default)]
    pub trivial: bool,
    #[serde(rename = "nonTrivial", default)]
    pub non_trivial: bool,
    #[serde(rename = "userDeclared", default)]
    pub user_declared: bool,
    #[serde(rename = "needsImplicit", default)]
    pub needs_implicit: bool,
    #[serde(rename = "needsOverloadResolution", default)]
    pub needs_overload_resolution: bool,
    #[serde(rename = "defaultedIsDeleted", default)]
    pub defaulted_is_deleted: bool,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ExceptionSpec {
    #[serde(skip_deserializing)]
    None,
    #[serde(rename = "throw")]
    Throw,
    #[serde(rename = "noexcept")]
    Noexcept,
    #[serde(rename = "nothrow")]
    Nothrow,
}

impl Default for ExceptionSpec {
    fn default() -> Self {
        ExceptionSpec::None
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ExplicitlyDefaulted {
    #[serde(skip_deserializing)]
    No,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "default")]
    Default,
}

impl Default for ExplicitlyDefaulted {
    fn default() -> Self {
        ExplicitlyDefaulted::No
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum InitStyle {
    #[serde(skip_deserializing)]
    NoInit,
    #[serde(rename = "c")]
    CInit,
    #[serde(rename = "call")]
    CallInit,
    #[serde(rename = "list")]
    ListInit,
}

impl Default for InitStyle {
    fn default() -> Self {
        InitStyle::NoInit
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum Language {
    #[serde(rename = "C")]
    C,
    #[serde(rename = "C++")]
    Cxx,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MoveAssignmentDefinitionData {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub simple: bool,
    #[serde(default)]
    pub trivial: bool,
    #[serde(rename = "nonTrivial", default)]
    pub non_trivial: bool,
    #[serde(rename = "userDeclared", default)]
    pub user_declared: bool,
    #[serde(rename = "needsImplicit", default)]
    pub needs_implicit: bool,
    #[serde(rename = "needsOverloadResolution", default)]
    pub needs_overload_resolution: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MoveConstructorDefinitionData {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub simple: bool,
    #[serde(default)]
    pub trivial: bool,
    #[serde(rename = "nonTrivial", default)]
    pub non_trivial: bool,
    #[serde(rename = "userDeclared", default)]
    pub user_declared: bool,
    #[serde(rename = "needsImplicit", default)]
    pub needs_implicit: bool,
    #[serde(rename = "needsOverloadResolution", default)]
    pub needs_overload_resolution: bool,
    #[serde(rename = "defaultedIsDeleted", default)]
    pub defaulted_is_deleted: bool,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum NonOdrUseReason {
    #[serde(skip_deserializing)]
    None,
    #[serde(rename = "unevaluated")]
    Unevaluated,
    #[serde(rename = "constant")]
    Constant,
    #[serde(rename = "discarded")]
    Discarded,
}

impl Default for NonOdrUseReason {
    fn default() -> Self {
        NonOdrUseReason::None
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ParamDirection {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "in,out")]
    InOut,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum RefQualifier {
    #[serde(skip_deserializing)]
    None,
    #[serde(rename = "&")]
    LValue,
    #[serde(rename = "&&")]
    RValue,
}

impl Default for RefQualifier {
    fn default() -> Self {
        RefQualifier::None
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum RenderKind {
    #[serde(rename = "anchor")]
    Anchor,
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "emphasized")]
    Emphasized,
    #[serde(rename = "monospaced")]
    Monospaced,
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ScopedEnumTag {
    #[serde(skip_deserializing)]
    None,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "struct")]
    Struct,
}

impl Default for ScopedEnumTag {
    fn default() -> Self {
        ScopedEnumTag::None
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum StorageClass {
    #[serde(skip_deserializing)]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "extern")]
    Extern,
    #[serde(rename = "__private_extern__")]
    PrivateExtern,
    #[serde(rename = "register")]
    Register,
    #[serde(rename = "static")]
    Static,
}

impl Default for StorageClass {
    fn default() -> Self {
        StorageClass::None
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum StorageDuration {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "full expression")]
    FullExpression,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "thread")]
    Thread,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum TagTypeKind {
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "union")]
    Union,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "enum")]
    Enum,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum TemplateTypeParmTag {
    #[serde(rename = "typename")]
    Typename,
    #[serde(rename = "class")]
    Class,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum TLSKind {
    #[serde(skip_deserializing)]
    None,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "static")]
    Static,
}

impl Default for TLSKind {
    fn default() -> Self {
        TLSKind::None
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Type {
    #[serde(rename = "qualType")]
    pub qual_type: Box<str>,
    #[serde(rename = "desugaredQualType")]
    pub desugared_qual_type: Option<Box<str>>,
    #[serde(rename = "typeAliasDeclId")]
    pub type_alias_decl_id: Option<Id>,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum UnaryOpcode {
    #[serde(rename = "++")]
    Inc,
    #[serde(rename = "--")]
    Dec,
    #[serde(rename = "&")]
    AddrOf,
    #[serde(rename = "*")]
    Deref,
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "~")]
    Not,
    #[serde(rename = "!")]
    LNot,
    #[serde(rename = "__real")]
    Real,
    #[serde(rename = "__imag")]
    Imag,
    #[serde(rename = "__extension__")]
    Extension,
    #[serde(rename = "co_await")]
    Coawait,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum UnaryTransformTypeKind {
    #[serde(rename = "add_lvalue_reference")]
    AddLvalueReference,
    #[serde(rename = "add_pointer")]
    AddPointer,
    #[serde(rename = "add_rvalue_reference")]
    AddRvalueReference,
    #[serde(rename = "decay")]
    Decay,
    #[serde(rename = "make_signed")]
    MakeSigned,
    #[serde(rename = "make_unsigned")]
    MakeUnsigned,
    #[serde(rename = "remove_all_extents")]
    RemoveAllExtents,
    #[serde(rename = "remove_const")]
    RemoveConst,
    #[serde(rename = "remove_cv")]
    RemoveCV,
    #[serde(rename = "remove_cvref")]
    RemoveCVRef,
    #[serde(rename = "remove_extent")]
    RemoveExtent,
    #[serde(rename = "remove_pointer")]
    RemovePointer,
    #[serde(rename = "remove_reference_t")]
    RemoveReference,
    #[serde(rename = "remove_restrict")]
    RemoveRestrict,
    #[serde(rename = "remove_volatile")]
    RemoveVolatile,
    #[serde(rename = "underlying_type")]
    EnumUnderlyingType,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ValueCategory {
    #[serde(rename = "lvalue")]
    LValue,
    #[serde(rename = "xvalue")]
    XValue,
    #[serde(rename = "rvalue")]
    RValue,
    #[serde(rename = "prvalue")]
    PRValue,
}

#[derive(Deserialize, Default, Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum Visibility {
    #[default]
    #[serde(rename = "default")]
    Default,
}

fn default_true() -> bool {
    true
}

#[cfg(target_pointer_width = "64")]
#[rustversion::since(2023-04-29)]
const _: [(); std::mem::size_of::<Node>()] = [(); 1472];

fn with_much_stack<F, T>(test: F) -> T
where
    F: FnOnce() -> T + Send,
    T: Send,
{
    thread::scope(|scope| {
        ThreadBuilder::new()
            .stack_size(6 * 1024 * 1024)
            .spawn_scoped(scope, test)
            .unwrap()
            .join()
            .unwrap()
    })
}

#[test]
fn test() -> Result<(), serde_json::Error> {
    let json = clang_ast_test_suite::cxx_ast_json();
    let result = with_much_stack(|| serde_json::from_slice::<Node>(&json).map(drop));

    if let Err(error) = &result {
        if env::var_os("CI").is_some() {
            if let Ok(json) = str::from_utf8(&json) {
                let mut stderr = io::stderr().lock();
                for line in json.lines().skip(error.line().saturating_sub(30)).take(60) {
                    let _ = writeln!(stderr, "{}", line);
                }
            }
        }
    }

    result
}
