
//---------------------------------------
// NODE
// Collection of all 
// possible Nodes
//---------------------------------------
pub enum Node {
  // PRIMITIVES
  ID(IdNode),
  STR(StrNode),
  CHAR(CharNode),
  INT(IntNode),
  FLOAT(FloatNode),
  
  // OPERATORS
  OP_EM(OpEmNode), //    !    33   exclamation mark
  OP_NS(OpNsNode), //    #    35   number sign, pound
  OP_DS(OpDsNode), //    $    36   dollar sign
  OP_PC(OpPcNode), //    %    37   percent sign
  OP_AM(OpAmNode), //    &    38   ampersand
  OP_LP(OpLpNode), //    (    40   left paranthesis
  OP_RP(OpRpNode), //    )    41   right paranthesis
  OP_AS(OpAsNode), //    *    42   asterix
  OP_PS(OpPsNode), //    +    43   plus sign
  OP_CM(OpCmNode), //    ,    44   comma
  OP_MS(OpMsNode), //    -    45   minus sign
  OP_DP(OpDpNode), //    .    46   decimal point
  OP_SL(OpSlNode), //    /    47   slash
  OP_CN(OpCnNode), //    :    58   colon
  OP_SE(OpSeNode), //    ;    59   semicolon
  OP_LT(OpLtNode), //    <    60   less-than sign
  OP_EQ(OpEqNode), //    =    61   equal sign
  OP_GT(OpGtNode), //    >    62   greater-than sign
  OP_QM(OpQmNode), //    ?    63   question mark
  OP_AT(OpAtNode), //    @    64   commercial sign
  OP_LS(OpLsNode), //    [    91   left square bracket
  OP_BS(OpBsNode), //    \    92   backslash
  OP_RS(OpRsNode), //    ]    93   right square bracket
  OP_CI(OpCiNode), //    ^    94   circumflex
  OP_US(OpUsNode), //    _    95   underscore
  OP_LB(OpLbNode), //    {    123  left brace
  OP_VB(OpVbNode), //    |    124  vertical bar
  OP_RB(OpRbNode), //    }    125  right brace
  OP_TD(OpTdNode), //    ~    126  tilde

  // TYPES
  TY(TyNode),
  ITY(ITyNode), // Intrinsic Type
  PTY(PTyNode), // Pointer Type
  ATY(ATyNode), // Array Type
  CTY(CTyNode), // Compound Type
  FTY(FTyNode), // Function Type
  OTY(OTyNode), // Option Type
  VTY(VTyNode), // Void Type

  // VARIABLE
  VAR(VarNode), // Variable

  // TYPE_DECLARATION
  TDEC(TdecNode),   // Type Declaration
  STDEC(STdecNode), // Struct Type Declaration
  UTDEC(UTdecNode), // Union Type Declaration
  ETDEC(ETdecNode), // Enum Type Declaration
  
  // FUNCTION
  FN(FnNode),
  
  // MACRO 
  MAC(MacNode),
  
  // INTRINSIC_TYPE
  INTY(InTyNode), 
  
  // DATA_NODE (VARIABLE WITH DATA)
  DATA(DataNode),
  
  // STATEMENT
  STM(StmNode),
  ESTM(EStmNode), // Expression Statement
  LSTM(LStmNode), // Label Statement
  JSTM(JStmNode), // Jump Statement
  BSTM(BStmNode), // Block Statement

  // EXPRESSION
  EXP(ExpNode),
  
  // VOID
  VOID(VoidNode),
}

// PRIMITIVES
pub struct IdNode;
pub struct StrNode;
pub struct CharNode;
pub struct IntNode;
pub struct FloatNode;

// OPERATORS
pub struct OpEmNode; //    !    33   exclamation mark
pub struct OpNsNode; //    #    35   number sign, pound
pub struct OpDsNode; //    $    36   dollar sign
pub struct OpPcNode; //    %    37   percent sign
pub struct OpAmNode; //    &    38   ampersand
pub struct OpLpNode; //    (    40   left paranthesis
pub struct OpRpNode; //    )    41   right paranthesis
pub struct OpAsNode; //    *    42   asterix
pub struct OpPsNode; //    +    43   plus sign
pub struct OpCmNode; //    ,    44   comma
pub struct OpMsNode; //    -    45   minus sign
pub struct OpDpNode; //    .    46   decimal point
pub struct OpSlNode; //    /    47   slash
pub struct OpCnNode; //    :    58   colon
pub struct OpSeNode; //    ;    59   semicolon
pub struct OpLtNode; //    <    60   less-than sign
pub struct OpEqNode; //    =    61   equal sign
pub struct OpGtNode; //    >    62   greater-than sign
pub struct OpQmNode; //    ?    63   question mark
pub struct OpAtNode; //    @    64   commercial sign
pub struct OpLsNode; //    [    91   left square bracket
pub struct OpBsNode; //    \    92   backslash
pub struct OpRsNode; //    ]    93   right square bracket
pub struct OpCiNode; //    ^    94   circumflex
pub struct OpUsNode; //    _    95   underscore
pub struct OpLbNode; //    {    123  left brace
pub struct OpVbNode; //    |    124  vertical bar
pub struct OpRbNode; //    }    125  right brace
pub struct OpTdNode; //    ~    126  tilde

// TYPES
pub struct TyNode;
pub struct ITyNode; // Intrinsic Type
pub struct PTyNode; // Pointer Type
pub struct ATyNode; // Array Type
pub struct CTyNode; // Compound Type
pub struct FTyNode; // Function Type
pub struct OTyNode; // Option Type
pub struct VTyNode; // Void Type

// VARIABLE
pub struct VarNode; // Variable

// TYPE_DECLARATION
pub struct TdecNode;   // Type Declaration
pub struct STdecNode; // pub Struct Type Declaration
pub struct UTdecNode; // Union Type Declaration
pub struct ETdecNode; // Enum Type Declaration

// FUNCTION
pub struct FnNode;

// MACRO 
pub struct MacNode;

// INTRINSIC_TYPE
pub struct InTyNode;

// DATA_NODE (VARIABLE WITH DATA)
pub struct DataNode;

// STATEMENT
pub struct StmNode;
pub struct EStmNode; // Expression Statement
pub struct LStmNode; // Label Statement
pub struct JStmNode; // Jump Statement
pub struct BStmNode; // Block Statement

// EXPRESSION
pub struct ExpNode;

// VOID
pub struct VoidNode;


/*
pub enum TyNode {
  INTRINSIC(ITyNode),
  ARRAY(ATyNode),
  COMPOUND(CTyNode),
  FN(FTyNode),
}

impl TyNode {
  pub fn size(&self) -> usize {
    match self {
      INTRINSIC(var) => var.size(),
      ARRAY(var) => var.size(),
      COMPOUND(var) => var.size(),
      FN(var) => var.size(),
    }
  }
}

pub enum ITyNode {
  INT,
  UINT,
  I8,
  U8,
  I16,
  U16,
  I32,
  U32,
  I64,
  U64,
  F32,
  F64,
  BOOL,
}

impl ITyNode {
  pub fn size(&self) -> usize {
    match self {
      INT => 4, // TODO: machine dependent
      UINT => 4, // same
      I8 => 1,
      U8 => 1,
      I16 => 2,
      U16 => 2,
      I32 => 4,
      U32 => 4,
      I64 => 8,
      U64 => 8,
      F32 => 4,
      F64 => 8,
      BOOL => 1,
    }
  }
}

pub struct ATyNode {
  type: TyNode,
}

impl ATyNode {
  pub fn type(&self) -> &TyNode {
    &self.type
  }
  
  pub fn size(&self) -> usize {
    4 // TODO: ptr size
  }
}

pub struct CTyNode {
}

pub struct FTyNode {
}
*/
