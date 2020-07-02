#ifdef TEST_PASS
test "string_test" {

}
#endif

// note: Add a .test file that can be optionally included in the directory which allows a shell command to be executed prior to the test?
// todo: Finalization pass over tokenizer
//          Tokenizer will go into a parse layer which swaps test "foo" { } to an actual function call
//          int test_foo(test_ctx* Ctx) {}
//          these function calls are put into <orig_name>_generated.cpp
//          after tests are compiled and executed the generated files are deleted
//          maybe have the shell command stuff inside of ctx?
// todo: Write and research testing framework!

#include <direct.h>
#include <stdio.h>
#include <stdlib.h>

#include <types.h>
#include <shared.h>
#include <tokenizer.h>

static string msg_prefix = make_string("[Test Suite] ");

string ReadEntireFileIntoMemory(char* FileName) {
    string Result = { };

    FILE* File = fopen(FileName, "r");
	if (File) {
		fseek(File, 0, SEEK_END);
		Result.Count = ftell(File);
		fseek(File, 0, SEEK_SET);

		Result.Data = (u8*)malloc(Result.Count);
		fread(Result.Data, Result.Count, 1, File);

		fclose(File);
	}

    return Result;
}

#include <string.h>
#include <minwindows.h>

struct lexer_state {
	char** FileNames;
	u32 FileNameCount;
};

static lexer_state State;

void ListContents(const char* Cwd) {
	char Path[MAX_PATH];

	_WIN32_FIND_DATAA FindData;
	HANDLE FindHandle = INVALID_HANDLE_VALUE;

	//Specify a file mask. *.* = We want everything!
	sprintf(Path, "%s\\*.*", Cwd);
	FindHandle = FindFirstFileA(Path, &FindData);

	do {
		if (strcmp(FindData.cFileName, ".") != 0 && strcmp(FindData.cFileName, "..") != 0) {
			sprintf(Path, "%s\\%s", Cwd, FindData.cFileName);

			if (FindData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
				ListContents(Path);
			}
			else if (Substring(Path, "src") != NULL) {
				State.FileNames[State.FileNameCount] = (char*)malloc(sizeof(char) * MAX_PATH);
				strcpy(State.FileNames[State.FileNameCount], Path);
				++State.FileNameCount;
			}
		}
	} while (FindNextFileA(FindHandle, &FindData));
}



int main(char** Args, int ArgCount) {
    TCHAR** lppPart = {NULL}; // For GetFullPath :/

    char* cwd;
	cwd = _getcwd(NULL, 256);

    printf("%sTest Suite v0.1\n", msg_prefix.Data);
	printf("%sOperating in directory: %s\n\n", msg_prefix.Data, cwd);

	// Get the names of the files we will operate on
	State.FileNames = (char**)malloc(2048 * sizeof(char*));
	if (Substring(cwd, "build")) {
		ListContents("..\\tests");
	} else {
		ListContents("tests");
	}

	for (u32 FileIndex = 0; FileIndex < State.FileNameCount; ++FileIndex) {
		char* Filename = State.FileNames[FileIndex];
        char* FullFilename = (char*)malloc(MAX_PATH * sizeof(char));
        GetFullPathNameA(Filename, MAX_PATH, FullFilename, lppPart);
		printf("%s: \n", FullFilename);

		string Contents = ReadEntireFileIntoMemory(Filename);
		tokenizer Tokenizer = Tokenize(Contents, Filename);

		b32 Parsing = true;
		while (Parsing) {
			token Token = GetToken(&Tokenizer);
			switch (Token.Type) {
			case Token_EndOfStream: {
				Parsing = false;
			} break;

			case Token_Identifier: {
				if (StringsMatch(Token.String, "test")) {
					GetToken(&Tokenizer);
					token Next = GetToken(&Tokenizer);

					char* TestName = Next.String;
					printf("	test '%s'\n", TestName);

				}
			} break;

			case Token_Unknown:
			default: {

			} break;
			}
		}
	}

    return 1;
}