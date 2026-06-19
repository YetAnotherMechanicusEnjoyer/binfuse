const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const elf_mod = b.createModule(.{
        .root_source_file = b.path("zig/pack/elf.zig"),
        .target = target,
        .optimize = optimize,
    });
    const pe_mod = b.createModule(.{
        .root_source_file = b.path("zig/pack/pe.zig"),
        .target = target,
        .optimize = optimize,
    });
    const macho_mod = b.createModule(.{
        .root_source_file = b.path("zig/pack/macho.zig"),
        .target = target,
        .optimize = optimize,
    });

    const pack_module = b.addModule("pack", .{
        .root_source_file = b.path("zig/pack/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    pack_module.addImport("elf", elf_mod);
    pack_module.addImport("pe", pe_mod);
    pack_module.addImport("macho", macho_mod);

    const server_module = b.addModule("server", .{
        .root_source_file = b.path("zig/server.zig"),
        .target = target,
        .optimize = optimize,
    });

    const wrapper_module = b.addModule("wrapper", .{
        .root_source_file = b.path("zig/wrapper.zig"),
        .target = target,
        .optimize = optimize,
    });
    wrapper_module.addImport("pack", pack_module);
    wrapper_module.addImport("server", server_module);

    const ffi_lib = b.addLibrary(.{
        .linkage = .static,
        .name = "binfuse_zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("zig/pack/mod.zig"),
            .target = target,
            .optimize = optimize,
            .link_libc = true,
        }),
    });

    ffi_lib.root_module.addImport("pack", pack_module);
    ffi_lib.root_module.addImport("server", server_module);

    b.installArtifact(ffi_lib);

    const wrapper_exe = b.addExecutable(.{
        .name = "binfuse-wrapper",
        .root_module = wrapper_module,
    });

    b.installArtifact(wrapper_exe);
}
