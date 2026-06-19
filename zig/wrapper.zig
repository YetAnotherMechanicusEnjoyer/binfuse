const std = @import("std");
const server = @import("server.zig");

pub fn main(init: std.process.Init) !void {
    var allocator = init.arena.allocator();

    const args = try init.minimal.args.toSlice(allocator);
    const self_path = args[0];

    const self_file = std.Io.Dir.cwd().openFile(init.io, self_path, .{ .mode = .read_only }) catch |err| {
        std.log.err("Failed to open self: {}\n", .{err});
        return err;
    };
    defer self_file.close(init.io);

    var reader = self_file.reader(init.io, &[_]u8{});
    const self_data = reader.interface.readAlloc(allocator, std.math.maxInt(usize)) catch |err| {
        std.log.err("Failed to read self: {}\n", .{err});
        return err;
    };
    defer allocator.free(self_data);

    var assets = std.ArrayList(server.Asset).initCapacity(allocator, 0) catch |err| {
        std.log.err("Failed to initialize assets array list: {}", .{err});
        return err;
    };
    defer assets.deinit(allocator);

    const elf_header_size = 4096;
    var pos: usize = elf_header_size;

    while (pos < self_data.len) : (pos += 1) {
        const path_start = pos;
        while (pos < self_data.len and self_data[pos] != 0) : (pos += 1) {}
        if (pos >= self_data.len) break;

        const path = self_data[path_start..pos];
        pos += 1;

        const data_start = pos;
        while (pos < self_data.len and self_data[pos] != 0) : (pos += 1) {}
        const data = self_data[data_start..pos];
        if (pos < self_data.len) pos += 1;

        assets.append(allocator, .{
            .path = allocator.dupe(u8, path) catch |err| {
                std.log.err("Failed to append asset: {}", .{err});
                return err;
            },
            .data = allocator.dupe(u8, data) catch |err| {
                std.log.err("Failed to append asset: {}", .{err});
                return err;
            },
        }) catch |err| {
            std.log.err("Failed to append asset: {}", .{err});
            return err;
        };
    }

    try server.serve_assets(&allocator, init.io, 8080, assets.items);
}
