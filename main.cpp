// (1) Include the header file
#include "peglib.h"
#include <fstream>
#include <iostream>

using namespace peg;
using namespace std;


void my_ast_indent(std::string &s, int level) {
    for (auto i = 0; i < level; i++) {
        s += "    ";
    }
}

template <typename T> void my_ast_to_s_core(const std::shared_ptr<T> &ptr, std::string &s, int level) {
    const auto &ast = *ptr;
    auto name = ast.original_name;
    // if (ast.original_choice_count > 0) {
    //     name += ": " + std::to_string(ast.original_choice);
    // }
    if (ast.name != ast.original_name) { name += "[" + ast.name + "]"; }
    if (ast.is_token) { // トークン
        s += "\x1b[36m"+name+"\x1b[0m: ";
        s += ast.token;
    } else { // 構造
        //my_ast_indent(s,level);
        s += "\x1b[31m"+name+"\x1b[0m" + " {\n";
        for (auto node : ast.nodes) {
            my_ast_indent(s,level+1);
            my_ast_to_s_core(node, s, level + 1);
            s += ",\n";
        }
        my_ast_indent(s,level);
        s += "}";
    }
}

template <typename T> std::string my_ast_to_s(const std::shared_ptr<T> &ptr,int level=0) {
    std::string s;
    my_ast_indent(s,level);
    my_ast_to_s_core(ptr, s, level);
    s += "\n";
    return s;
}


int main(int argc, char *argv[]) {
    std::cout << "NJP Language\n";
    std::cout << "\n";
    
    std::cout << "Grammar File: " << argv[1];
    std::ifstream ifs(argv[1]);
    std::string line;
    std::string data;
    if (ifs.fail()) {
        std::cerr << "Failed to open file." << std::endl;
        return -1;
    }
    while (getline(ifs, line)) {
        data += line+"\n";
    }
    std::cout << "\n";
    std::cout << "\n";

    parser parser(data.c_str());
    parser.enable_ast();
    parser.enable_packrat_parsing();

    string input;
    while (cin >> input) {
        shared_ptr<peg::Ast> ast;
        cout << "[input]: " << input << "\n";
        if (parser.parse(input, ast)) {
            cout << my_ast_to_s(ast,2);
            // std::cout << "\n";
            // ast = parser.optimize_ast(ast);
            // cout << my_ast_to_s(ast);
        }
        else {
            cerr << "Failed to Parse\n";
        }
        cout << "\n";
    }
}