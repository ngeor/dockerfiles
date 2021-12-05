"""
Rules that can execute a GWBasic/QBasic program.
"""

def _basic_binary_implementation(ctx):
    is_windows = ctx.target_platform_has_constraint(ctx.attr._windows_constraint[platform_common.ConstraintValueInfo])
    if is_windows:
        # runfiles don't get linked correctly with Bazel on Windows,
        # so we have to process the manifest file ourselves
        output = ctx.actions.declare_file("{}.bat".format(ctx.label.name))
        ctx.actions.write(
            output,
            """@ECHO OFF
REM Make environment variables local for this script.
SETLOCAL

REM Store the BAS file as it should appear in the manifest.
SET "_BAS_FILE_IN_MANIFEST=__main__/{}"

REM Store the EXE file of the wrapper as it should appear in the manifest.
SET "_EXE_FILE_IN_MANIFEST=__main__/{}"

REM Process MANIFEST file.
REM The manifest is space separated, which is the default for the FOR command.
REM Capture the two tokens in variables I and J and call the ":process" label.
REM The first token is the short path prefixed by __main__/,
REM the second token is the physical path on Windows.
FOR /F "tokens=1,2" %%I IN (MANIFEST) DO CALL :process %%I %%J
GOTO run

:process
IF "%1" == "%_BAS_FILE_IN_MANIFEST%" SET "_REAL_BAS=%2"
IF "%1" == "%_EXE_FILE_IN_MANIFEST%" SET "_REAL_EXE=%2"
GOTO :eof

:run
%_REAL_EXE% %_REAL_BAS%

REM restore environment variables
ENDLOCAL
""".format(
                ctx.file.src.short_path,
                ctx.executable._basic_dosbox_wrapper.short_path,
            ).replace("\r\n", "\n").replace("\n", "\r\n"),
            is_executable = True,
        )
    else:
        output = ctx.actions.declare_file("{}.sh".format(ctx.label.name))
        ctx.actions.write(
            output,
            "{} {}".format(ctx.executable._basic_dosbox_wrapper.short_path, ctx.file.src.short_path),
            is_executable = True,
        )
    return DefaultInfo(executable = output, runfiles = ctx.runfiles(
        files = [ctx.file.src],
    ).merge(ctx.attr._basic_dosbox_wrapper[DefaultInfo].default_runfiles))

gwbasic_binary = rule(
    implementation = _basic_binary_implementation,
    executable = True,
    attrs = {
        "src": attr.label(mandatory = True, allow_single_file = True),
        "_basic_dosbox_wrapper": attr.label(
            default = "//basic/gwbasic_dosbox_wrapper",
            cfg = "exec",
            executable = True,
        ),
        "_windows_constraint": attr.label(default = Label("@platforms//os:windows")),
    },
)

qbasic_binary = rule(
    implementation = _basic_binary_implementation,
    executable = True,
    attrs = {
        "src": attr.label(mandatory = True, allow_single_file = True),
        "_basic_dosbox_wrapper": attr.label(
            default = "//basic/qbasic_dosbox_wrapper",
            cfg = "exec",
            executable = True,
        ),
        "_windows_constraint": attr.label(default = Label("@platforms//os:windows")),
    },
)
