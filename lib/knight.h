#ifndef KNIGHT_H
#define KNIGHT_H

#include <stdint.h>
#include "drgn.h"

typedef struct {
    struct drgn_program *prog;
} prog_t;

enum bus_type {
    BUS_PCI = 0,
    BUS_USB,
    BUS_PLATFORM,
};

prog_t *program_create();
void program_destroy(prog_t *p);
void object_free(struct drgn_object *obj);

struct drgn_object *find_task(prog_t *p, uint64_t pid);
struct drgn_object *find_object_variable(prog_t *p, char *name);
struct drgn_object *get_obj_member(struct drgn_object *obj, char *name);
struct drgn_object *deref_obj_member(struct drgn_object *obj, char *name);
struct drgn_object *container_of(struct drgn_object *ptr, char *type, char *member);
bool obj_addr(struct drgn_object *obj, uint64_t *out);
bool obj2num(struct drgn_object *obj, uint64_t *out);
bool obj2cstr(struct drgn_object *obj, char **out);

#endif
