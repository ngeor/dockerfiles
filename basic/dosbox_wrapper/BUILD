load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "dosbox_wrapper",
    srcs = ["main.rs"],
    deps = [":dosbox_lib"],
)

rust_library(
    name = "dosbox_lib",
    srcs = [
        "lib.rs",
        "dosbox.rs",
        "find.rs",
        "path_util.rs",
    ],
)
