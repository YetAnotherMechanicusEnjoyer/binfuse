const std = @import("std");

pub fn add_custom_section(
    allocator: *std.mem.Allocator,
    macho_path: []const u8,
    section_name: []const u8,
    section_data: []const u8,
    output_path: []const u8,
) !void {
    _ = allocator;
    _ = macho_path;
    _ = section_name;
    _ = section_data;
    _ = output_path;
    // TODO: Implement for macOS.
}
