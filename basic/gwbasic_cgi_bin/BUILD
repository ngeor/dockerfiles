load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "gwbasic_cgi_bin",
    srcs = ["main.rs"],
    deps = [
        "//basic/dosbox_wrapper:dosbox_lib",
    ],
)
