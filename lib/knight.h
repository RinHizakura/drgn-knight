#ifndef KNIGHT
#define KNIGHT

#include <stdint.h>

typedef struct {
    struct drgn_program *prog;
} prog_t;

prog_t *program_create();
void program_destroy(prog_t *p);
void find_task_member(prog_t *p, uint64_t pid);


#endif
