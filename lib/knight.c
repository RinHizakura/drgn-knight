#include <stdio.h>
#include <stdlib.h>

#include "drgn.h"
#include "helpers.h"
#include "knight.h"
static struct drgn_error *find_task(struct drgn_program *prog,
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

static struct drgn_error *find_task(struct drgn_program *prog,
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

void find_task_member(prog_t *p, uint64_t pid)
{
    struct drgn_program *prog = p->prog;
    struct drgn_error *err = NULL;
    DRGN_OBJECT(task, prog);
    DRGN_OBJECT(member, prog);

    err = find_task(prog, pid, &task);
    if (err)
        goto find_task_err;

    err = drgn_object_member_dereference(&member, &task, "on_cpu");
    if (err)
        goto find_task_err;

    bool on_cpu;
    err = drgn_object_bool(&member, &on_cpu);
    if (err)
        goto find_task_err;

    printf("on_cpu %d\n", on_cpu);

find_task_err:
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
    }
    return;
}
