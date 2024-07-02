#ifndef KNIGHT_H
#define KNIGHT_H

#include <stdint.h>
#include "drgn.h"

typedef struct {
    struct drgn_program *prog;
} prog_t;

prog_t *program_create();
void program_destroy(prog_t *p);
void object_free(struct drgn_object *obj);

struct drgn_object *find_task(prog_t *p, uint64_t pid);
struct drgn_object *deref_obj_member(prog_t *p,
                                     struct drgn_object *obj,
                                     char *name);
bool obj2num(struct drgn_object *obj, uint64_t *out);

#endif
