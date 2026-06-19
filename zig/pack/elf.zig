const std = @import("std");

pub const ElfSection = struct {
    name: []const u8,
    data: []const u8,
    offset: u64,
    size: u64,
};

pub fn read_elf_section(file_path: []const u8) ![]ElfSection {
    _ = file_path;
    // TODO:
    return &[_]ElfSection{};
}

pub fn add_custom_section(
    elf_path: []const u8,
    section_name: []const u8,
    section_data: []const u8,
    output_path: []const u8,
) !void {
    _ = elf_path;
    _ = section_name;
    _ = section_data;
    _ = output_path;
    // TODO: Implement for Linux.
}
