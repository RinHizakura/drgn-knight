#include "knight.h"

int main()
{
    prog_t *prog = program_create();
    if (!prog)
        return 1;

    find_task_member(prog, 1);
    program_destroy(prog);
    return 0;
}
