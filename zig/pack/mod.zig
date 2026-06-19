const std = @import("std");

pub const Asset = struct {
    path: []const u8,
    data: []const u8,
};

pub const PackConfig = struct {
    compression: []const u8,
    embed_assets: bool,
};

fn streamFile(io: anytype, dir: anytype, file_path: []const u8, writer_interface: anytype) !void {
    const file = try dir.openFile(io, file_path, .{ .mode = .read_only });
    defer file.close(io);

    var reader = file.reader(io, &[_]u8{});
    _ = reader.interface.stream(writer_interface, .unlimited) catch |err| switch (err) {
        error.EndOfStream => {},
        else => return err,
    };
}

fn serializeDirectoryFiles(io: anytype, allocator: std.mem.Allocator, dir_path: []const u8, writer_interface: anytype) !void {
    const dir = try std.Io.Dir.cwd().openDir(io, dir_path, .{ .access_sub_paths = true, .iterate = true });
    defer dir.close(io);

    var walker = try dir.walk(allocator);
    defer walker.deinit();

    while (try walker.next(io)) |entry| {
        if (entry.kind != .file) continue;

        try writer_interface.writeAll(entry.path);
        try writer_interface.writeAll(&[_]u8{0});
        try streamFile(io, dir, entry.path, writer_interface);
    }
}

fn generateEmbeddedBinary(io: anytype, allocator: std.mem.Allocator, binary_path: []const u8, assets_dir: []const u8, output_path: []const u8) !void {
    const output_file = try std.Io.Dir.cwd().createFile(io, output_path, .{});
    defer output_file.close(io);
    var writer = output_file.writer(io, &[_]u8{});

    try streamFile(io, std.Io.Dir.cwd(), binary_path, &writer.interface);

    try serializeDirectoryFiles(io, allocator, assets_dir, &writer.interface);
}

fn copyFileToPath(io: anytype, input_path: []const u8, output_path: []const u8) !void {
    const output_file = try std.Io.Dir.cwd().createFile(io, output_path, .{});
    defer output_file.close(io);
    var writer = output_file.writer(io, &[_]u8{});

    try streamFile(io, std.Io.Dir.cwd(), input_path, &writer.interface);
}

pub export fn zig_embed_assets(
    binary_path: [*c]const u8,
    assets_dir: [*c]const u8,
    output_path: [*c]const u8,
) callconv(.c) c_int {
    const allocator = std.heap.c_allocator;
    var threaded = std.Io.Threaded.init(allocator, .{});
    defer threaded.deinit();

    generateEmbeddedBinary(
        threaded.io(),
        allocator,
        std.mem.span(binary_path),
        std.mem.span(assets_dir),
        std.mem.span(output_path),
    ) catch |err| {
        std.log.err("Failed to embed assets: {}\n", .{err});
        return 1;
    };

    return 0;
}

pub export fn zig_generate_wrapper(
    binary_path: [*c]const u8,
    output_path: [*c]const u8,
    port: c_int,
) callconv(.c) c_int {
    _ = port;
    const allocator = std.heap.c_allocator;
    var threaded = std.Io.Threaded.init(allocator, .{});
    defer threaded.deinit();

    copyFileToPath(
        threaded.io(),
        std.mem.span(binary_path),
        std.mem.span(output_path),
    ) catch |err| {
        std.log.err("Failed to generate wrapper: {}\n", .{err});
        return 1;
    };

    return 0;
}
