#include <stdio.h>
#include <stdlib.h>

#include "drgn.h"
#include "helpers.h"
#include "util.h"

int main()
{
    const char *core = "/proc/kcore";

    struct drgn_program *prog = NULL;
    struct drgn_error *err = drgn_program_create(NULL, &prog);
    if (err)
        goto out;

    err = drgn_program_set_core_dump(prog, core);
    if (err)
        goto out;

    err = drgn_program_load_debug_info(prog, NULL, 0, true, false);
    if (err && err->code == DRGN_ERROR_MISSING_DEBUG_INFO) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
    } else if (err) {
        goto out;
    }

    struct drgn_object object;
    drgn_object_init(&object, prog);

    err = drgn_program_find_object(prog, "init_pid_ns", NULL,
                                   DRGN_FIND_OBJECT_VARIABLE, &object);
    if (err)
        goto obj_out;

    err = drgn_object_address_of(&object, &object);
    if (err)
        goto obj_out;

    err = linux_helper_find_task(&object, &object, 1);
    if (err)
        goto obj_out;

    printf("obj size %lx\n", drgn_object_size(&object));

obj_out:
    drgn_object_deinit(&object);
out:
    int status = err ? EXIT_FAILURE : EXIT_SUCCESS;
    if (err) {
        drgn_error_fwrite(stderr, err);
        drgn_error_destroy(err);
    }
    drgn_program_destroy(prog);
    return status;
}
