grammar movable;

movable
    : defineDecl
    ;

defineDecl
    : Identifier DOLLOAR Identifier ARROW ;


lineStringLiteral
    : '#'
    | SINGLE_QUOTE Identifier SINGLE_QUOTE
    ;

// keywords
DEFINE: 'define';
TYPE: 'type';
FUNCTION: 'function' ;

//symbol
AT: '@' ;
DOT: '.' ;
DOLLOAR: '$';
HASH: '#' ;
SINGLE_QUOTE: '\'' ;
QUOTE: '"' ;
//QUOTE_CLOSE: '"';

// companion
ARROW: '->' ;
DOUBLE_ARROW: '=>' ;


WS:                 [ \t\r\n\u000C]+ -> channel(HIDDEN);
COMMENT:            '/*' .*? '*/'    -> channel(HIDDEN);
LINE_COMMENT:       '//' ~[\r\n]*    -> channel(HIDDEN);


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
