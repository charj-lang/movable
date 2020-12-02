grammar movable;

movable
    : defineDecl* EOF
    ;

defineDecl
    : ruleDecl
    | templateDecl
    | specDecl
    ;

ruleDecl
    : DEFINE? Identifier DOLLOAR Identifier ARROW ruleExpr* ';'
    ;

ruleExpr
    : '@' Identifier
    | lineStringLiteral
    ;


templateDecl
    : Identifier DOLLOAR Identifier '#' INT  ARROW ruleExpr* ';'
    | functionDecl
    ;

specDecl
    : SPEC
    ;

functionDecl
    : 'function' ':' ruleDecl
    ;

lineStringLiteral
    : '#'
    | SINGLE_QUOTE Identifier SINGLE_QUOTE
    ;

// keywords
DEFINE: 'define';
TYPE: 'type';
FUNCTION: 'function';
SPEC: 'spec';

//symbol
AT:                 '@';
DOT:                '.';
DOLLOAR:            '$';
HASH:               '#';
SINGLE_QUOTE:       '\'';
QUOTE:              '"';
LBRACE:             '{';
RBRACE:             '}';

// companion
ARROW: '->' ;
DOUBLE_ARROW: '=>' ;

QUESTION
   : Question
   ;

STAR
   : Star
   ;

PLUS
   : Plus
   ;


WS:                 [ \t\r\n\u000C]+ -> channel(HIDDEN);
COMMENT:            '/*' .*? '*/'    -> channel(HIDDEN);
LINE_COMMENT:       '//' ~[\r\n]*    -> channel(HIDDEN);

fragment Question
   : '?'
   ;

fragment Star
   : '*'
   ;

fragment Plus
   : '+'
   ;

Identifier
    : (Nondigit| UniversalCharacterName)+
    ;

fragment Nondigit
    :   [a-zA-Z_]
    ;

fragment UniversalCharacterName
    :   '\\u' HexQuad
    |   '\\U' HexQuad HexQuad
    ;

fragment HexQuad
    :   HexadecimalDigit HexadecimalDigit HexadecimalDigit HexadecimalDigit
    ;

fragment HexadecimalDigit
    :   [0-9a-fA-F]
    ;

INT
   : DecimalNumeral
   ;

fragment DecimalNumeral
   : '0'
   | [1-9] DecDigit*
   ;

fragment HexDigit
   : [0-9a-fA-F]
   ;

fragment DecDigit
   : [0-9]
   ;
