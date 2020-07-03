#include <math.h>

b32 IsTokenValid(token Token) {
    b32 Result = (Token.Type != Token_Unknown);
    return Result;
}

b32 TokenEquals(token Token, char* Match) {
    b32 Result = StringsMatch(Token.String, Match);
    return Result;
}

void Refill(tokenizer *Tokenizer){
    if(Tokenizer->Input.Count == 0) {
        Tokenizer->At[0] = 0;
        Tokenizer->At[1] = 0;
    } else if(Tokenizer->Input.Count == 1) {
        Tokenizer->At[0] = Tokenizer->Input.Data[0];
        Tokenizer->At[1] = 0;
    } else {
        char C0 = Tokenizer->Input.Data[0];
        char C1 = Tokenizer->Input.Data[1];
        Tokenizer->At[0] = C0;
        Tokenizer->At[1] = C1;
    }
}

void AdvanceInput(tokenizer* Tokenizer, u32 Count) {
    Tokenizer->TotalCount += Count;
    Advance(&Tokenizer->Input, Count);
    Refill(Tokenizer);
}

tokenizer Tokenize(string Data, char* Filename) {
    tokenizer Result = {};
    
    Result.Filename = Filename;
    Result.LinesCount = 1;
    Result.TokensCount = 1;   
    Result.TotalCount = 1;    
    Result.Input = Data;
    Result.At = (char*)malloc(2);
    Refill(&Result);
    
    return(Result);
}

tokenizer Tokenize(char* Data);
tokenizer Tokenize(char* Filename);

token GetToken(tokenizer* Tokenizer) {
	token Token = {};
    Token.Filename = Tokenizer->Filename;
	Token.TextLength = 1;
	Token.Text = Tokenizer->Input;

	char C = Tokenizer->At[0];
    AdvanceInput(Tokenizer, 1);
	switch (C) {
        case '\0': { Token.Type = Token_EndOfStream; } break;

        case '(': { Token.Type = Token_OpenParen; } break;
        case ')': { Token.Type = Token_CloseParen; } break;
        case ':': { Token.Type = Token_Colon; } break;
        case ';': { Token.Type = Token_Semicolen; } break;
        case '*': { Token.Type = Token_Asterik; } break;
        case '[': { Token.Type = Token_OpenBracket; } break;
        case ']': { Token.Type = Token_CloseBracket; } break;
        case '{': { Token.Type = Token_OpenBrace; } break;
        case '}': { Token.Type = Token_CloseBrace; } break;
        case '=': { Token.Type = Token_Equals; } break;
        case ',': { Token.Type = Token_Comma; } break;
        case '|': { Token.Type = Token_Or; } break;
        case '#': { Token.Type = Token_Pound; } break;

        case '"': { // note(jax): We've got a string
			Token.Type = Token_String;
            while(Tokenizer->At[0] && Tokenizer->At[0] != '"') {
                if ((Tokenizer->At[0] == '\\') && (Tokenizer->At[1])) {
                    AdvanceInput(Tokenizer, 1);
                }

                AdvanceInput(Tokenizer, 1);;
            }

            if (Tokenizer->At[0] == '"') {
                AdvanceInput(Tokenizer, 1);
            }

            // Speed: This block of code is pretty hacky...
            if (Token.Text.Count &&
                (Token.Text.Data[0] == '"')) {
                ++Token.Text.Data;
                --Token.Text.Count;

                int TextLength = 0;
                char* Temp = (char*)malloc(Token.Text.Count);
                sprintf(Temp, "%s", Token.Text.Data);
                while (Temp) {
                    if (*Temp == '\"') {
                        break;
                    }
                    ++TextLength;
                    ++Temp;
                }

                Token.String = (char*)malloc(TextLength);
                sprintf(Token.String, "%.*s", (int)TextLength, Token.Text.Data);
            }
        } break;

        default: {
            if (IsSpacing(C)) {
                Token.Type = Token_Space;
                while (IsSpacing(Tokenizer->At[0])) {
                    AdvanceInput(Tokenizer, 1);
                }
            } else if (IsEndOfLine(C)) {
                Token.Type =  Token_EndOfLine;
                if(((C == '\r') &&
                    (Tokenizer->At[0] == '\n')) ||
                   ((C == '\n') &&
                    (Tokenizer->At[0] == '\r'))) {
                    AdvanceInput(Tokenizer, 1);
                }
                
                ++Tokenizer->LinesCount;
            } else if((C == '/') && (Tokenizer->At[0] == '/')) {
                Token.Type = Token_Comment;
                
                AdvanceInput(Tokenizer, 2);
                while(Tokenizer->At[0] && !IsEndOfLine(Tokenizer->At[0])) {
                    AdvanceInput(Tokenizer, 1);
                }
            } else if((C == '/') &&
                    (Tokenizer->At[0] == '*')) {
                Token.Type = Token_Comment;
                
                AdvanceInput(Tokenizer, 2);
                while(Tokenizer->At[0] && !((Tokenizer->At[0] == '*') 
                    && (Tokenizer->At[1] == '/'))) {
                    if(((Tokenizer->At[0] == '\r') && (Tokenizer->At[1] == '\n')) 
                        || ((Tokenizer->At[0] == '\n') && (Tokenizer->At[1] == '\r'))) {
                        AdvanceInput(Tokenizer, 1);
                    }
                    
                    if (IsEndOfLine(Tokenizer->At[0])) {
                        ++Tokenizer->LinesCount;
                    }
                    
                    AdvanceInput(Tokenizer, 1);
                }
                
                if(Tokenizer->At[0] == '*') {
                    AdvanceInput(Tokenizer, 2);
                }
            } else if (IsAlphabetical(C)) {
				Token.Type = Token_Identifier;
                while (IsAlphabetical(Tokenizer->At[0]) || IsNumeric(Tokenizer->At[0]) || (Tokenizer->At[0] == '_')) {
                    AdvanceInput(Tokenizer, 1);
                }

                __int64 IndentLength = (Tokenizer->Input.Data - Token.Text.Data);
                Token.String = (char*)malloc(IndentLength);
                sprintf(Token.String, "%.*s", (int)IndentLength, Token.Text.Data);
            } else if (IsNumeric(C)) {
                f32 Number = (f32)(C - '0');
                
                while (IsNumeric(Tokenizer->At[0])) {
                    f32 Digit = (f32)(Tokenizer->At[0] - '0');
                    Number = 10.0f*Number + Digit;
                    AdvanceInput(Tokenizer, 1);
                }
                
                // This float detection code is copy pasted, I have no idea how it works!
                if (Tokenizer->At[0] == '.') {
                    AdvanceInput(Tokenizer, 1);
                    f32 Coefficient = 0.1f;
                    while (IsNumeric(Tokenizer->At[0])) {
                        f32 Digit = (f32)(Tokenizer->At[0] - '0');
                        Number += Coefficient * Digit;
                        Coefficient *= 0.1f;
                        AdvanceInput(Tokenizer, 1);
                    }
                }
                
                Token.Type = Token_Literal;
                Token.Float = Number;
                Token.Integer = (s32)Number;
            } else {
                Token.Type = Token_Unknown;
            }
        } break;
    }

    ++Tokenizer->TokensCount;
    Token.Text.Count = (Tokenizer->Input.Data - Token.Text.Data);
    return Token;
}

token PeekToken(tokenizer* Tokenizer) {
    tokenizer Temp = *Tokenizer;
    token Result = GetToken(Tokenizer);
    *Tokenizer = Temp;
    return Result;
}