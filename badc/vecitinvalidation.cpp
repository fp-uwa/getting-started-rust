#include <vector>
#include <iostream>

int main(int argc, char **argv) {
    std::vector<int> vec;

    vec.push_back(1);
    vec.push_back(2);
    vec.push_back(3);
    vec.push_back(4);

    std::vector<int>::iterator end = vec.end();

    for (std::vector<int>::iterator it = vec.begin();
         it != end;
         ++it) {
        if (*it > 1) {
            vec.erase(it);
        }

        (*it)++;
        std::cout << *it << std::endl;
    }

    return 0;
}
