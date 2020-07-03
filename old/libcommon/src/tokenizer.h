#ifndef TOKENIZER_H
#define TOKENIZER_H

enum token_type {
    Token_Unknown,

    Token_OpenParen, // (
    Token_CloseParen, // )
    Token_Colon, // :
    Token_Semicolen, // ;
    Token_Asterik, // *
    Token_OpenBracket, // [
    Token_CloseBracket, // ]
    Token_OpenBrace, // {
    Token_CloseBrace, // }
    Token_Equals,
    Token_Comma,
    Token_Or,
    Token_Pound,

    Token_String,
    Token_Identifier,
    Token_Literal, // Number

    Token_Space,
    Token_EndOfLine,
    Token_Comment,

    Token_EndOfStream,
};

struct token {
    char* Filename;

    token_type Type;
    size_t TextLength;
    string Text;
    union {
        char* String;
        f32 Float;
        s32 Integer;
    };
};

struct tokenizer {
    char* Filename;
    u32 LinesCount;
    u32 TokensCount;
    u32 TotalCount;

    string Input;
    char* At;
};

b32 IsTokenValid(token Token);
b32 TokenEquals(token Token, char* Match);
API_EXPORT token GetToken(tokenizer* Tokenizer);
API_EXPORT token PeekToken(tokenizer* Tokenizer);
API_EXPORT tokenizer Tokenize(char* Data);
API_EXPORT tokenizer Tokenize(char* Filename);
API_EXPORT tokenizer Tokenize(string Data, char* Filename);

#endif