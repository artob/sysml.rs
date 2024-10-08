// This is free and unencumbered software released into the public domain.

use crate::Span;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Keyword {
    About,
    Abstract,
    Accept,
    Action,
    Actor,
    After,
    Alias,
    All,
    Allocate,
    Allocation,
    Analysis,
    And,
    As,
    Assert,
    Assign,
    Assoc,
    Assume,
    At,
    Attribute,
    Bind,
    Binding,
    Block,
    By,
    Calc,
    Case,
    Comment,
    Concern,
    Connect,
    Connection,
    Constraint,
    Decide,
    Def,
    Default,
    Defined,
    Dependency,
    Derived,
    Do,
    Doc,
    Else,
    End,
    Entry,
    Enum,
    Event,
    Exhibit,
    Exit,
    Expose,
    Filter,
    First,
    Flow,
    For,
    Fork,
    Frame,
    From,
    HasType,
    If,
    Implies,
    Import,
    In,
    Include,
    Individual,
    InOut,
    Interface,
    IsType,
    Item,
    Join,
    Language,
    Loop,
    Merge,
    Message,
    Metadata,
    NonUnique,
    Not,
    Objective,
    Occurrence,
    Of,
    Or,
    Ordered,
    Out,
    Package,
    Parallel,
    Part,
    Perform,
    Port,
    Private,
    Protected,
    Public,
    ReadOnly,
    Redefines,
    Ref,
    References,
    Render,
    Rendering,
    Rep,
    Require,
    Requirement,
    Return,
    Satisfy,
    Send,
    Snapshot,
    Specializes,
    Stakeholder,
    State,
    Subject,
    Subsets,
    Succession,
    Then,
    TimeSlice,
    To,
    Transition,
    Until,
    Use,
    Variant,
    Variation,
    Verification,
    Verify,
    Via,
    View,
    Viewpoint,
    When,
    While,
    Xor,
}

impl<'a> TryFrom<Span<'a>> for Keyword {
    type Error = Span<'a>;

    fn try_from(input: Span<'a>) -> Result<Self, Self::Error> {
        Self::try_from(*input.fragment()).map_err(|_| input)
    }
}

impl<'a> TryFrom<&'a str> for Keyword {
    type Error = &'a str;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        use Keyword::*;
        Ok(match input {
            "about" => About,
            "abstract" => Abstract,
            "accept" => Accept,
            "action" => Action,
            "actor" => Actor,
            "after" => After,
            "alias" => Alias,
            "all" => All,
            "allocate" => Allocate,
            "allocation" => Allocation,
            "analysis" => Analysis,
            "and" => And,
            "as" => As,
            "assert" => Assert,
            "assign" => Assign,
            "assoc" => Assoc,
            "assume" => Assume,
            "at" => At,
            "attribute" => Attribute,
            "bind" => Bind,
            "binding" => Binding,
            "block" => Block,
            "by" => By,
            "calc" => Calc,
            "case" => Case,
            "comment" => Comment,
            "concern" => Concern,
            "connect" => Connect,
            "connection" => Connection,
            "constraint" => Constraint,
            "decide" => Decide,
            "def" => Def,
            "default" => Default,
            "defined" => Defined,
            "dependency" => Dependency,
            "derived" => Derived,
            "do" => Do,
            "doc" => Doc,
            "else" => Else,
            "end" => End,
            "entry" => Entry,
            "enum" => Enum,
            "event" => Event,
            "exhibit" => Exhibit,
            "exit" => Exit,
            "expose" => Expose,
            "filter" => Filter,
            "first" => First,
            "flow" => Flow,
            "for" => For,
            "fork" => Fork,
            "frame" => Frame,
            "from" => From,
            "hastype" => HasType,
            "if" => If,
            "implies" => Implies,
            "import" => Import,
            "in" => In,
            "include" => Include,
            "individual" => Individual,
            "inout" => InOut,
            "interface" => Interface,
            "istype" => IsType,
            "item" => Item,
            "join" => Join,
            "language" => Language,
            "loop" => Loop,
            "merge" => Merge,
            "message" => Message,
            "metadata" => Metadata,
            "nonunique" => NonUnique,
            "not" => Not,
            "objective" => Objective,
            "occurrence" => Occurrence,
            "of" => Of,
            "or" => Or,
            "ordered" => Ordered,
            "out" => Out,
            "package" => Package,
            "parallel" => Parallel,
            "part" => Part,
            "perform" => Perform,
            "port" => Port,
            "private" => Private,
            "protected" => Protected,
            "public" => Public,
            "readonly" => ReadOnly,
            "redefines" => Redefines,
            "ref" => Ref,
            "references" => References,
            "render" => Render,
            "rendering" => Rendering,
            "rep" => Rep,
            "require" => Require,
            "requirement" => Requirement,
            "return" => Return,
            "satisfy" => Satisfy,
            "send" => Send,
            "snapshot" => Snapshot,
            "specializes" => Specializes,
            "stakeholder" => Stakeholder,
            "state" => State,
            "subject" => Subject,
            "subsets" => Subsets,
            "succession" => Succession,
            "then" => Then,
            "timeslice" => TimeSlice,
            "to" => To,
            "transition" => Transition,
            "until" => Until,
            "use" => Use,
            "variant" => Variant,
            "variation" => Variation,
            "verification" => Verification,
            "verify" => Verify,
            "via" => Via,
            "view" => View,
            "viewpoint" => Viewpoint,
            "when" => When,
            "while" => While,
            "xor" => Xor,
            _ => return Err(input),
        })
    }
}
