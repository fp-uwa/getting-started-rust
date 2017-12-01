#include <stdlib.h>

struct mytype {
    int mem1;
    int mem2;
    int mem3;
    int mem4;
};

struct anotherstruct {
    int mem1;
};

int main(int argc, char ** argv) {
    size_t len = atoi(argv[1]);
    struct mytype *structs =
        calloc (1, sizeof (struct anotherstruct) * atoi(argv[1]));

    for (int i = 0; i < len; ++i) {
        structs[i].mem1 = 1;
        structs[i].mem2 = 2;
    }

    free(structs);

    return 0;
}
