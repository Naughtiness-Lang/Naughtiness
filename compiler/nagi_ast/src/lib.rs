#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ASTNode {
    pub node_kind: ASTNodeKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASTNodeKind {
    Item { visibility: Visibility, item: Item },
    Expression(Expression),
    Type(Types),
    TypeAlias(TypeAlias),

    Visibility(Visibility),
    Statements(Vec<Statement>),
    Pattern(Pattern),
    StructFields(Vec<StructField>),
    EnumItems(Vec<EnumItem>),
    CallParams(Vec<Expression>),
    FunctionParameters(Vec<FunctionParam>),

    Field(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Visibility {
    Private,
    Path(Path),
    Public,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Function {
        name: String,
        params: Vec<FunctionParam>,
        return_type: Option<Types>,
        body: Option<Box<Expression>>,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
    },
    Enumeration {
        name: String,
        items: Vec<EnumItem>,
    },
    Union {
        name: String,
        fields: Vec<StructField>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnumItem {
    StructItem {
        visibility: Visibility,
        name: String,
        fields: Vec<StructField>,
    },

    // TODO
    TupleItem {
        visibility: Visibility,
        name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructField {
    pub visibility: Visibility,
    pub name: String,
    pub field_type: Types,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Literal(Literal),
    Path(Path),
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assign {
        operator: AssignOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Comparison {
        operator: ComparisonOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Array {
        elements: ArrayElements,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    FieldAccess {
        parent: Box<Expression>,
        field_name: String,
    },
    MethodCall(MethodCall),
    Index {
        parent: Box<Expression>,
        index: Box<Expression>,
    },
    Block(Vec<Statement>),
    Loop(LoopExpr),
    If {
        condition: Box<Expression>,
        then_body: Box<Expression>,
        else_body: Option<Box<Expression>>,
    },
    Continue {
        label: String,
    },
    Break {
        label: String,
        expression: Option<Box<Expression>>,
    },
    Return(Option<Box<Expression>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoopExpr {
    While {
        condition: Box<Expression>,
        body: Box<Expression>,
    },
    For {
        pattern: Pattern,
        iterator: Box<Expression>,
        body: Box<Expression>,
    },
    Loop {
        body: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Semicolon,
    Expression(Box<Expression>),
    Let {
        name: Pattern,
        variable_type: Option<Types>,
        initializer: Option<Box<Expression>>,
    },
    Item(Item),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Import {
    pub path: Vec<String>,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAlias {
    pub name: String,
    pub target: Types,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Types {
    // hoge::Fuga
    // Vec<T>, HashMap<K, V>
    // int, u32, String
    PathType(Box<Path>),

    // fn(i32) -> bool
    Function {
        params: Vec<Types>,
        return_type: Box<Types>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionParam {
    pub pattern: Pattern,
    pub param_type: Types,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArrayElements {
    List(Vec<Expression>),
    Repeat {
        value: Box<Expression>,
        count: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodCall {
    pub name: Path,
    pub params: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Float(String),
    Integer(String),
    Char(String),
    UnicodeChar(String),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Assign(AssignOperator),
    Binary(BinaryOperator),
    Comparison(ComparisonOperator),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    LeftShiftAssign,
    RightShiftAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Addition,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    Literal(Literal),
    Identifier {
        ident: String,
        mutable: bool,
        reference: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub ident: String,
    pub arguments: Vec<Types>,
}
