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

static struct drgn_error *__get_obj_member(struct drgn_object *obj,
                                           char *name,
                                           struct drgn_object *ret_member)
{
    struct drgn_error *err;
    err = drgn_object_member(ret_member, obj, name);
    if (err)
        return err;

    return err;
}

static struct drgn_error *__deref_obj_member(struct drgn_object *obj,
                                             char *name,
                                             struct drgn_object *ret_member)
{
    struct drgn_error *err = NULL;
    err = drgn_object_member_dereference(ret_member, obj, name);
    if (err)
        return err;

    return err;
}

struct drgn_object *find_object_variable(prog_t *p, char *name)
{
    struct drgn_error *err = NULL;
    struct drgn_program *prog = p->prog;
    struct drgn_object *object = object_alloc(prog);

    err = drgn_program_find_object(prog, name, NULL,
                                   DRGN_FIND_OBJECT_VARIABLE, object);
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        object_free(object);
        return NULL;
    }

    return object;
}

struct drgn_object *get_obj_member(struct drgn_object *obj, char *name)
{
    struct drgn_program *prog = drgn_object_program(obj);
    struct drgn_object *member = object_alloc(prog);
    struct drgn_error *err = NULL;

    err = __get_obj_member(obj, name, member);
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

    err = __deref_obj_member(obj, name, member);
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        object_free(member);
        return NULL;
    }

    return member;
}

struct drgn_object *container_of(struct drgn_object *ptr,
	char *type, char *member)
{
    struct drgn_program *prog = drgn_object_program(ptr);
    struct drgn_object *object = NULL;
    struct drgn_error *err = NULL;
    struct drgn_qualified_type qtype;

    err = drgn_program_find_type(prog, type, NULL, &qtype);
    if (err)
        goto container_of_end;

    object = object_alloc(prog);
    err = drgn_object_container_of(object, ptr, qtype, member);
    if (err)
        goto container_of_end;

container_of_end:
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        object_free(object);
        return NULL;
    }

    return object;
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

bool obj2cstr(struct drgn_object *obj, char **out)
{
    struct drgn_error *err = NULL;

    err = drgn_object_read_c_string(obj, out);
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
        return false;
    }

    return true;
}
