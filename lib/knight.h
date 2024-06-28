#ifndef KNIGHT_H
#define KNIGHT_H

#include <stdint.h>

typedef struct {
    struct drgn_program *prog;
} prog_t;

prog_t *program_create();
void program_destroy(prog_t *p);
struct drgn_object *find_task(prog_t *p, uint64_t pid);
bool find_task_member(prog_t *p, struct drgn_object *task, char *member);

#endif
