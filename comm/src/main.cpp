#include <fstream>
#include <iostream>
#include <cassert>
#include <unordered_set>

std::unordered_set<std::string> read_lines(const char* path)
{
    std::unordered_set<std::string> lines_set;
    std::ifstream in(path);
    std::string line;
    while (std::getline(in, line)) {
        lines_set.insert(line);
    }
    return lines_set;
}

int main(int argc, char **argv)
{
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);

    assert(argc == 3);

    auto first_lines = read_lines(argv[1]);

    std::ifstream second_in(argv[2]);
    std::string second_line;
    while (std::getline(second_in, second_line)) {
        auto it = first_lines.find(second_line);
        if (it != first_lines.end()) {
            std::cout << second_line << '\n';
            first_lines.erase(it);
        }
    }

    return 0;
}
