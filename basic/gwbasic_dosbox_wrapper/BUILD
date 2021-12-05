load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "gwbasic_dosbox_wrapper",
    srcs = glob(["*.rs"]),
    deps = [
        "//basic/dosbox_wrapper:dosbox_lib",
    ],
)
