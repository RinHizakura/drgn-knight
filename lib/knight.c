#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers.h"
#include "knight.h"

static struct drgn_error *__find_task(struct drgn_program *prog,
                                      uint64_t pid,
                                      struct drgn_object *ret_task);

prog_t *program_create()
{
    prog_t *p;
    struct drgn_program *prog = NULL;
    struct drgn_error *err = NULL;

    p = calloc(1, sizeof(prog_t));
    if (!p)
        goto out;

    err = drgn_program_create(NULL, &prog);
    if (err)
        goto out;

    err = drgn_program_set_core_dump(prog, "/proc/kcore");
    if (err)
        goto out;

    err = drgn_program_load_debug_info(prog, NULL, 0, true, false);
    if (err && err->code == DRGN_ERROR_MISSING_DEBUG_INFO) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        err = NULL;
    }

    p->prog = prog;

out:
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        drgn_program_destroy(prog);

        free(p);
        p = NULL;
    }
    return p;
}

void program_destroy(prog_t *p)
{
    if (!p)
        return;

    drgn_program_destroy(p->prog);
    free(p);
}

static struct drgn_object *object_alloc(struct drgn_program *prog)
{
    struct drgn_object *obj = malloc(sizeof(struct drgn_object));

    drgn_object_init(obj, prog);

    return obj;
}

void object_free(struct drgn_object *obj)
{
    if (obj == NULL)
        return;
    drgn_object_deinit(obj);
    free(obj);
}

static struct drgn_error *__find_task(struct drgn_program *prog,
                                      uint64_t pid,
                                      struct drgn_object *ret_task)
{
    struct drgn_error *err = NULL;
    DRGN_OBJECT(object, prog);

    err = drgn_program_find_object(prog, "init_pid_ns", NULL,
                                   DRGN_FIND_OBJECT_VARIABLE, &object);
    if (err)
        return err;

    err = drgn_object_address_of(&object, &object);
    if (err)
        return err;

    err = linux_helper_find_task(ret_task, &object, pid);
    if (err)
        return err;

    return err;
}

struct drgn_object *find_task(prog_t *p, uint64_t pid)
{
    struct drgn_program *prog = p->prog;
    struct drgn_object *task = object_alloc(prog);
    struct drgn_error *err = NULL;

    err = __find_task(prog, pid, task);
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        object_free(task);
        return NULL;
    }

    return task;
}

struct drgn_object *get_obj_member(struct drgn_object *obj, char *name)
{
    struct drgn_program *prog = drgn_object_program(obj);
    struct drgn_object *member = object_alloc(prog);
    struct drgn_error *err = NULL;

    err = drgn_object_member(member, obj, name);
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        object_free(member);
        return NULL;
    }

    return member;
}

struct drgn_object *deref_obj_member(struct drgn_object *obj, char *name)
{
    struct drgn_program *prog = drgn_object_program(obj);
    struct drgn_object *member = object_alloc(prog);
    struct drgn_error *err = NULL;

    err = drgn_object_member_dereference(member, obj, name);
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        object_free(member);
        return NULL;
    }

    return member;
}

bool obj_addr(struct drgn_object *obj, uint64_t *out)
{
    struct drgn_program *prog = drgn_object_program(obj);
    struct drgn_error *err = NULL;
    DRGN_OBJECT(addr_obj, prog);

    err = drgn_object_address_of(&addr_obj, obj);

    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        return false;
    }

    return obj2num(&addr_obj, out);
}

bool obj2num(struct drgn_object *obj, uint64_t *out)
{
    struct drgn_error *err = NULL;

    union drgn_value value_mem;
    const union drgn_value *value;
    err = drgn_object_read_value(obj, &value_mem, &value);
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        drgn_object_deinit_value(obj, value);
        return false;
    }

    // Ignore sign of interger
    *out = value->uvalue;

    drgn_object_deinit_value(obj, value);
    return true;
}
